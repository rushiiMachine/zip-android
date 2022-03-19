use std::{
    fs::File,
    path::Path,
};

use jni::{
    JNIEnv,
    objects::{JClass, JObject, JString},
    sys::{jint, jobject, jobjectArray, jsize},
};
use jni::sys::jbyteArray;
use zip::{
    read::ZipFile,
    result::{ZipError, ZipResult},
    ZipArchive,
};

use crate::cache;
use crate::interop::{get_inner, set_inner, take_inner};

fn make_zip_entry<'a>(env: &JNIEnv<'a>, zip_result: ZipResult<ZipFile<'a>>) -> JObject<'a> {
    let file = match zip_result {
        Ok(file) => file,
        Err(ZipError::FileNotFound) => {
            return JObject::null().into();
        }
        Err(e) => {
            env.throw(format!("Failed to open zip entry! {:?}", e)).unwrap();
            return JObject::null().into();
        }
    };

    let gref_class = cache::cls_zipentry();
    let zip_entry = env.new_object_unchecked(
        JClass::from(gref_class.as_obj()),
        cache::ctor_zipentry(),
        &[],
    ).unwrap();
    set_inner(&env, zip_entry, file).unwrap();
    zip_entry
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipReader_open(
    env: JNIEnv,
    class: JClass,
    jstr_path: JString,
) {
    let path: String = env.get_string(jstr_path).unwrap().into();
    let file = match File::open(Path::new(&path)) {
        Ok(file) => file,
        Err(e) => {
            env.throw(format!("Failed to open file: {:?}", e)).unwrap();
            return;
        }
    };

    let zip = ZipArchive::new(file).unwrap();
    set_inner(&env, class.into(), zip).unwrap();
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipReader_close(
    env: JNIEnv,
    class: JClass,
) {
    take_inner::<ZipArchive<File>>(&env, class.into()).unwrap();
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipReader_openEntry__I(
    env: JNIEnv,
    class: JClass,
    index: jint,
) -> jobject {
    let mut zip = get_inner::<ZipArchive<File>>(&env, class.into()).unwrap();
    let result = zip.by_index(index as usize);

    make_zip_entry(&env, result).into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipReader_openEntry__Ljava_lang_String_2(
    env: JNIEnv,
    class: JClass,
    jstr_path: JString,
) -> jobject {
    let path: String = env.get_string(jstr_path).unwrap().into();

    let mut zip = get_inner::<ZipArchive<File>>(&env, class.into()).unwrap();
    let result = zip.by_name(path.as_str());

    make_zip_entry(&env, result).into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipReader_openEntryRaw__I(
    env: JNIEnv,
    class: JClass,
    index: jint,
) -> jobject {
    let mut zip = get_inner::<ZipArchive<File>>(&env, class.into()).unwrap();
    let result = zip.by_index_raw(index as usize);

    make_zip_entry(&env, result).into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipReader_getEntryCount(
    env: JNIEnv,
    class: JClass,
) -> jint {
    let zip = get_inner::<ZipArchive<File>>(&env, class.into()).unwrap();
    zip.len() as jint
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipReader_getRawComment(
    env: JNIEnv,
    class: JClass,
) -> jbyteArray {
    let zip = get_inner::<ZipArchive<File>>(&env, class.into()).unwrap();
    env.byte_array_from_slice(zip.comment()).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipReader_getEntryNames(
    env: JNIEnv,
    class: JClass,
) -> jobjectArray {
    let zip = get_inner::<ZipArchive<File>>(&env, class.into()).unwrap();
    let names_length = zip.file_names().collect::<Vec<&str>>().len();

    let gref_class = cache::cls_string();
    let array = env.new_object_array(
        names_length as jsize,
        JClass::from(gref_class.as_obj()),
        JObject::null(),
    ).unwrap();

    for (i, name) in zip.file_names().enumerate() {
        let jvm_name = env.new_string(name).unwrap();
        env.set_object_array_element(array, i as jsize, jvm_name).unwrap();
    }

    array
}

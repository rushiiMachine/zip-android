use std::fs::File;
use std::path::Path;

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jint, jobject, jobjectArray, jsize};
use zip::read::ZipFile;
use zip::result::{ZipError, ZipResult};
use zip::ZipArchive;

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

    let zip_entry_class = env.find_class("com/github/diamondminer88/zip/ZipEntry").unwrap();
    let constructor = env.get_method_id(zip_entry_class, "<init>", "()V").unwrap();

    let zip_entry = env.new_object_unchecked(zip_entry_class, constructor, &[]).unwrap();
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
    let file = File::open(Path::new(&path)).unwrap();
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
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipReader_getEntryNames(
    env: JNIEnv,
    class: JClass,
) -> jobjectArray {
    let zip = get_inner::<ZipArchive<File>>(&env, class.into()).unwrap();
    let names_length = zip.file_names().collect::<Vec<&str>>().len();

    let array = env.new_object_array(
        names_length as jsize,
        env.find_class("java/lang/String").unwrap(),
        JObject::null(),
    ).unwrap();

    for (i, name) in zip.file_names().enumerate() {
        let jvm_name = env.new_string(name).unwrap();
        env.set_object_array_element(array, i as jsize, jvm_name).unwrap();
    }

    array
}

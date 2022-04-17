use std::{
    fs::File,
    path::Path,
};

use jni::{
    JNIEnv,
    objects::{JClass, JObject, JString},
    sys::{jbyteArray, jint, jobject, jobjectArray, jsize},
};
use jni_fn::jni_fn;
use zip::{
    read::ZipFile,
    result::{ZipError, ZipResult},
    ZipArchive,
};

use crate::cache;
use crate::interop::{get_field, ReentrantReference, set_field, take_field};

fn get_archive<'a>(env: &JNIEnv<'a>, obj: JClass<'a>) -> ReentrantReference<'a, ZipArchive<File>> {
    get_field(&env, obj, cache::fld_zipreader_ptr()).unwrap()
}

fn set_archive<'a>(env: &JNIEnv<'a>, obj: JClass<'a>, archive: ZipArchive<File>) {
    set_field(&env, obj, cache::fld_zipreader_ptr(), archive).unwrap();
}

fn set_entry<'a>(env: &JNIEnv<'a>, obj: JClass<'a>, entry: ZipFile) {
    set_field(&env, obj, cache::fld_zipentry_ptr(), entry).unwrap();
}

fn take_archive<'a>(env: &JNIEnv<'a>, obj: JClass<'a>) -> ZipArchive<File> {
    take_field(&env, obj, cache::fld_zipreader_ptr()).unwrap()
}

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
    set_entry(&env, zip_entry.into(), file);
    zip_entry
}

#[jni_fn("com.github.diamondminer88.zip.ZipReader")]
pub fn open(
    env: JNIEnv,
    class: JClass,
    path: JString,
) {
    let path: String = env.get_string(path).unwrap().into();
    let file = match File::open(Path::new(&path)) {
        Ok(file) => file,
        Err(e) => {
            env.throw(format!("Failed to open file: {:?}", e)).unwrap();
            return;
        }
    };

    let zip = match ZipArchive::new(file) {
        Ok(zip) => zip,
        Err(e) => {
            env.throw(format!("Failed to open archive: {:?}", e)).unwrap();
            return;
        }
    };
    set_archive(&env, class, zip);
}

#[jni_fn("com.github.diamondminer88.zip.ZipReader")]
pub fn close(
    env: JNIEnv,
    class: JClass,
) {
    take_archive(&env, class);
}

#[jni_fn("com.github.diamondminer88.zip.ZipReader")]
pub fn openEntry(
    env: JNIEnv,
    class: JClass,
    index: jint,
) -> jobject {
    let index = index as usize;

    let mut zip = get_archive(&env, class);
    let result = zip.by_index(index);

    make_zip_entry(&env, result).into_inner()
}

#[jni_fn("com.github.diamondminer88.zip.ZipReader")]
pub fn openEntry(
    env: JNIEnv,
    class: JClass,
    path: JString,
) -> jobject {
    let path: String = env.get_string(path).unwrap().into();

    let mut zip = get_archive(&env, class);
    let result = zip.by_name(path.as_str());

    make_zip_entry(&env, result).into_inner()
}

#[jni_fn("com.github.diamondminer88.zip.ZipReader")]
pub fn openEntryRaw(
    env: JNIEnv,
    class: JClass,
    index: jint,
) -> jobject {
    let index = index as usize;

    let mut zip = get_archive(&env, class);
    let result = zip.by_index_raw(index);

    make_zip_entry(&env, result).into_inner()
}

#[jni_fn("com.github.diamondminer88.zip.ZipReader")]
pub fn getEntryCount(
    env: JNIEnv,
    class: JClass,
) -> jint {
    let zip = get_archive(&env, class);
    zip.len() as jint
}

#[jni_fn("com.github.diamondminer88.zip.ZipReader")]
pub fn getRawComment(
    env: JNIEnv,
    class: JClass,
) -> jbyteArray {
    let zip = get_archive(&env, class);
    env.byte_array_from_slice(zip.comment()).unwrap()
}

#[jni_fn("com.github.diamondminer88.zip.ZipReader")]
pub fn getEntryNames(
    env: JNIEnv,
    class: JClass,
) -> jobjectArray {
    let zip = get_archive(&env, class);
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

#![feature(thread_id_value)]

mod interop;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jbyte, jbyteArray, jint, jlong, jobject, jsize, jstring};
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

// ----------JNI ZipReader----------

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
) -> jlong {
    let zip = get_inner::<ZipArchive<File>>(&env, class.into()).unwrap();
    zip.len() as i64
}


// ----------JNI ZipEntry----------

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getName(
    env: JNIEnv,
    class: JClass,
) -> jstring {
    let file = get_inner::<ZipFile>(&env, class.into()).unwrap();
    env.new_string(file.name()).unwrap().into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getComment(
    env: JNIEnv,
    class: JClass,
) -> jstring {
    let zip = get_inner::<ZipFile>(&env, class.into()).unwrap();
    env.new_string(zip.comment()).unwrap().into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_isDir(
    env: JNIEnv,
    class: JClass,
) -> jboolean {
    let zip = get_inner::<ZipFile>(&env, class.into()).unwrap();
    zip.is_dir().into()
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getMode(
    env: JNIEnv,
    class: JClass,
) -> jint {
    let zip = get_inner::<ZipFile>(&env, class.into()).unwrap();
    zip.unix_mode().unwrap_or(0) as i32
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getCRC32(
    env: JNIEnv,
    class: JClass,
) -> jint {
    let zip = get_inner::<ZipFile>(&env, class.into()).unwrap();
    zip.crc32() as i32
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getExtraData(
    env: JNIEnv,
    class: JClass,
) -> jbyteArray {
    let zip = get_inner::<ZipFile>(&env, class.into()).unwrap();
    let byte_array = env.new_byte_array(zip.extra_data().len() as jsize).unwrap();
    // env.set_byte_array_region(byte_array, 0, zip.extra_data() as &[jbyte]);
    byte_array
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getSize(
    env: JNIEnv,
    class: JClass,
) -> jlong {
    let zip = get_inner::<ZipFile>(&env, class.into()).unwrap();
    zip.size() as i64
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getCompressedSize(
    env: JNIEnv,
    class: JClass,
) -> jlong {
    let zip = get_inner::<ZipFile>(&env, class.into()).unwrap();
    zip.compressed_size() as i64
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_readEntry(
    env: JNIEnv,
    class: JClass,
) -> jbyteArray {
    let mut zip = get_inner::<ZipFile>(&env, class.into()).unwrap();

    if zip.is_dir() {
        env.throw("Cannot read data from a dir entry!").unwrap();
        return JObject::null().into_inner();
    }

    let mut data = Vec::new();
    zip.read_to_end(&mut data).unwrap();

    let jbyte_data = unsafe { std::slice::from_raw_parts(data.as_ptr() as *const jbyte, data.len()) };
    let byte_array = env.new_byte_array(data.len() as jsize).unwrap();
    env.set_byte_array_region(byte_array, 0, jbyte_data).unwrap();
    byte_array
}

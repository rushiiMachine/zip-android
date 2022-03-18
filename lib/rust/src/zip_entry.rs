use std::io::Read;

use jni::JNIEnv;
use jni::objects::{JClass, JObject};
use jni::sys::{jboolean, jbyteArray, jint, jlong, jsize, jstring};
use zip::read::ZipFile;

use crate::bytes_to_jbytes;
use crate::interop::get_inner;

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
    let data = zip.extra_data();

    let byte_array = env.new_byte_array(zip.extra_data().len() as jsize).unwrap();
    env.set_byte_array_region(byte_array, 0, bytes_to_jbytes(&data)).unwrap();
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

    let byte_array = env.new_byte_array(data.len() as jsize).unwrap();
    env.set_byte_array_region(byte_array, 0, bytes_to_jbytes(&data)).unwrap();
    byte_array
}

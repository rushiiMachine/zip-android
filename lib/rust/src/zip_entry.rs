use std::io::Read;

use jni::{
    JNIEnv,
    objects::{JClass, JObject, JValue},
    signature::JavaType,
    signature::Primitive::Long,
    sys::{jboolean, jbyteArray, jint, jlong, jsize, jstring},
};
use zip::read::ZipFile;

use crate::{bytes_to_jbytes, cache};
use crate::interop::get_inner;

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getName(
    env: JNIEnv,
    class: JClass,
) -> jstring {
    let entry = get_inner::<ZipFile>(&env, class.into()).unwrap();
    env.new_string(entry.name()).unwrap().into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getComment(
    env: JNIEnv,
    class: JClass,
) -> jstring {
    let entry = get_inner::<ZipFile>(&env, class.into()).unwrap();
    env.new_string(entry.comment()).unwrap().into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getLastModified(
    env: JNIEnv,
    class: JClass,
) -> jlong {
    let modified = get_inner::<ZipFile>(&env, class.into()).unwrap().last_modified();
    let args: Vec<JValue> = vec![
        (modified.year() - 1900).into(),
        (modified.month() - 1).into(),
        modified.day().into(),
        modified.hour().into(),
        (modified.minute() - 1).into(),
        (modified.second() - 1).into(),
    ];

    // Yes I could do this natively, however I'm not adding chrono just for this
    let gref_class = cache::cls_date();
    let unix_time = env.call_static_method_unchecked(
        JClass::from(gref_class.as_obj()),
        cache::method_date_utc(),
        JavaType::Primitive(Long),
        &args,
    ).unwrap();

    unix_time.j().unwrap()
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_isDir(
    env: JNIEnv,
    class: JClass,
) -> jboolean {
    let entry = get_inner::<ZipFile>(&env, class.into()).unwrap();
    entry.is_dir().into()
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getMode(
    env: JNIEnv,
    class: JClass,
) -> jint {
    let entry = get_inner::<ZipFile>(&env, class.into()).unwrap();
    entry.unix_mode().unwrap_or(0) as i32
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getCRC32(
    env: JNIEnv,
    class: JClass,
) -> jint {
    let entry = get_inner::<ZipFile>(&env, class.into()).unwrap();
    entry.crc32() as i32
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getExtraData(
    env: JNIEnv,
    class: JClass,
) -> jbyteArray {
    let entry = get_inner::<ZipFile>(&env, class.into()).unwrap();
    let data = entry.extra_data();

    let byte_array = env.new_byte_array(entry.extra_data().len() as jsize).unwrap();
    env.set_byte_array_region(byte_array, 0, bytes_to_jbytes(&data)).unwrap();
    byte_array
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getSize(
    env: JNIEnv,
    class: JClass,
) -> jlong {
    let entry = get_inner::<ZipFile>(&env, class.into()).unwrap();
    entry.size() as i64
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getCompressedSize(
    env: JNIEnv,
    class: JClass,
) -> jlong {
    let entry = get_inner::<ZipFile>(&env, class.into()).unwrap();
    entry.compressed_size() as i64
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_readEntry(
    env: JNIEnv,
    class: JClass,
) -> jbyteArray {
    let mut entry = get_inner::<ZipFile>(&env, class.into()).unwrap();

    if entry.is_dir() {
        env.throw("Cannot read data from a dir entry!").unwrap();
        return JObject::null().into_inner();
    }

    let mut data = Vec::new();
    entry.read_to_end(&mut data).unwrap();

    let byte_array = env.new_byte_array(data.len() as jsize).unwrap();
    env.set_byte_array_region(byte_array, 0, bytes_to_jbytes(&data)).unwrap();
    byte_array
}

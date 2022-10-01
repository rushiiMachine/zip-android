use std::io::Read;

use jni::{
    JNIEnv,
    objects::{JClass, JObject, JValue},
    signature::JavaType,
    signature::Primitive::Long,
    sys::{jboolean, jbyteArray, jint, jlong, jstring},
};
use jni_fn::jni_fn;

use zip::read::ZipFile;

use crate::{
    cache,
    interop::{get_field, ReentrantReference, take_field},
};

fn get_entry<'a>(env: &JNIEnv<'a>, obj: JClass<'a>) -> ReentrantReference<'a, ZipFile<'static>> {
    get_field(&env, obj, cache::fld_zipentry_ptr()).unwrap()
}

fn take_entry<'a>(env: &JNIEnv<'a>, obj: JClass<'a>) -> ZipFile<'static> {
    take_field(&env, obj, cache::fld_zipentry_ptr()).unwrap()
}

#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn getName(
    env: JNIEnv,
    class: JClass,
) -> jstring {
    let entry = get_entry(&env, class);
    env.new_string(entry.name()).unwrap().into_inner()
}

#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn getComment(
    env: JNIEnv,
    class: JClass,
) -> jstring {
    let entry = get_entry(&env, class);
    env.new_string(entry.comment()).unwrap().into_inner()
}

#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn getLastModified(
    env: JNIEnv,
    class: JClass,
) -> jlong {
    let modified = get_entry(&env, class).last_modified();
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
        cache::mtod_date_utc(),
        JavaType::Primitive(Long),
        &args,
    ).unwrap();

    unix_time.j().unwrap()
}

#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn isDir(
    env: JNIEnv,
    class: JClass,
) -> jboolean {
    let entry = get_entry(&env, class);
    entry.is_dir().into()
}

#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn getMode(
    env: JNIEnv,
    class: JClass,
) -> jint {
    let entry = get_entry(&env, class);
    entry.unix_mode().unwrap_or(0) as i32
}

#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn getCRC32(
    env: JNIEnv,
    class: JClass,
) -> jint {
    let entry = get_entry(&env, class);
    entry.crc32() as i32
}

#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn getExtraData(
    env: JNIEnv,
    class: JClass,
) -> jbyteArray {
    let entry = get_entry(&env, class);
    env.byte_array_from_slice(entry.extra_data()).unwrap()
}

#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn getSize(
    env: JNIEnv,
    class: JClass,
) -> jlong {
    let entry = get_entry(&env, class);
    entry.size() as i64
}

#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn getCompressedSize(
    env: JNIEnv,
    class: JClass,
) -> jlong {
    let entry = get_entry(&env, class);
    entry.compressed_size() as i64
}

#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn read(
    env: JNIEnv,
    class: JClass,
) -> jbyteArray {
    let mut entry = get_entry(&env, class);

    if entry.is_dir() {
        env.throw("Cannot read data from a dir entry!").unwrap();
        return JObject::null().into_inner();
    }

    let mut data = Vec::new();
    entry.read_to_end(&mut data).unwrap();

    env.byte_array_from_slice(&data).unwrap()
}

#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn _finalize(
    env: JNIEnv,
    class: JClass,
) {
    take_entry(&env, class);
}

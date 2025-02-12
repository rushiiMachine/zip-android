use catch_panic::catch_panic;
use std::io::Read;

use jni::signature::ReturnType;
use jni::sys::jvalue;
use jni::{
    objects::{JObject, JValue},
    signature::Primitive::Long,
    sys::{jboolean, jbyteArray, jint, jlong, jstring},
    JNIEnv,
};
use jni_fn::jni_fn;

use zip::read::ZipFile;
use zip::CompressionMethod;

use crate::{cache, interop};

/// Obtains an exclusive reference to the rust zip entry from a pointer in a JVM class.
macro_rules! obtain_entry {
    (&mut $env:ident, &$class:ident) => {{
        let entry = crate::interop::get_field::<_, _, ZipFile<'static>>(
            &mut $env,
            &$class,
            crate::cache::ZipEntry_ptr(),
        );

        entry
            .unwrap()
            .expect("using a closed entry, after JVM finalizer called!")
    }};
}

#[catch_panic(default = "0")]
#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn getIndex(mut env: JNIEnv, class: JObject) -> jint {
    let entry = obtain_entry!(&mut env, &class);
    entry.index() as jint
}

#[catch_panic(default = "JObject::null().into_raw()")]
#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn getName(mut env: JNIEnv, class: JObject) -> jstring {
    let entry = obtain_entry!(&mut env, &class);

    env.new_string(entry.name()).unwrap().into_raw()
}

#[catch_panic(default = "JObject::null().into_raw()")]
#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn getComment(mut env: JNIEnv, class: JObject) -> jstring {
    let entry = obtain_entry!(&mut env, &class);

    env.new_string(entry.comment()).unwrap().into_raw()
}

#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub unsafe fn getLastModified(mut env: JNIEnv, class: JObject) -> jlong {
    let entry = obtain_entry!(&mut env, &class);
    let modified = entry.last_modified();
    let args: Vec<jvalue> = vec![
        JValue::from(modified.year() - 1900).as_jni(),
        JValue::from(modified.month() - 1).as_jni(),
        JValue::from(modified.day()).as_jni(),
        JValue::from(modified.hour()).as_jni(),
        JValue::from(modified.minute() - 1).as_jni(),
        JValue::from(modified.second() - 1).as_jni(),
    ];

    // Yes I could do this natively, however I'm not adding chrono just for this
    let unix_time = env
        .call_static_method_unchecked(
            &cache::Date(),
            cache::Date_UTC(),
            ReturnType::Primitive(Long),
            &*args,
        )
        .unwrap();

    unix_time.j().unwrap()
}

#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn isDir(mut env: JNIEnv, class: JObject) -> jboolean {
    let entry = obtain_entry!(&mut env, &class);
    entry.is_dir().into()
}

#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn getMode(mut env: JNIEnv, class: JObject) -> jint {
    let entry = obtain_entry!(&mut env, &class);
    entry.unix_mode().unwrap_or(0) as i32
}

#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn getCRC32(mut env: JNIEnv, class: JObject) -> jint {
    let entry = obtain_entry!(&mut env, &class);
    entry.crc32() as i32
}

#[catch_panic(default = "JObject::null().into_raw()")]
#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn getExtraData(mut env: JNIEnv, class: JObject) -> jbyteArray {
    let entry = obtain_entry!(&mut env, &class);
    env.byte_array_from_slice(entry.extra_data())
        .unwrap()
        .into_raw()
}

#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn getSize(mut env: JNIEnv, class: JObject) -> jlong {
    let entry = obtain_entry!(&mut env, &class);
    entry.size() as i64
}

#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn getCompressedSize(mut env: JNIEnv, class: JObject) -> jlong {
    let entry = obtain_entry!(&mut env, &class);
    entry.compressed_size() as i64
}

#[allow(deprecated)]
#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn _getCompression(mut env: JNIEnv, class: JObject) -> jlong {
    let entry = obtain_entry!(&mut env, &class);
    match entry.compression() {
        CompressionMethod::Unsupported(_) => -1,
        CompressionMethod::Stored => 0,
        CompressionMethod::Deflated => 1,
        CompressionMethod::Bzip2 => 2,
        CompressionMethod::Zstd => 3,
        _ => -1,
    }
}

#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn getDataOffset(mut env: JNIEnv, class: JObject) -> jlong {
    let entry = obtain_entry!(&mut env, &class);
    entry.data_start() as i64
}

#[catch_panic(default = "JObject::null().into_raw()")]
#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn read(mut env: JNIEnv, class: JObject) -> jbyteArray {
    let mut entry = obtain_entry!(&mut env, &class);

    if entry.is_dir() {
        env.throw("Cannot read data from a dir entry!").unwrap();
        return JObject::null().into_raw();
    }

    let mut data = Vec::new();
    entry.read_to_end(&mut data).unwrap();

    env.byte_array_from_slice(&data).unwrap().into_raw()
}

#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipEntry")]
pub fn _finalize(mut env: JNIEnv, class: JObject) {
    let _ = interop::take_field::<_, _, ZipFile<'static>>(&mut env, &class, cache::ZipEntry_ptr());
}

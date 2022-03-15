

mod interop;

use std::fs::File;
use std::path::Path;
use std::sync::MutexGuard;

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jbyte, jbyteArray, jint, jlong, jobject, jsize, jstring};
use zip::read::ZipFile;
use zip::result::{ZipError, ZipResult};
use zip::ZipArchive;

fn jni_underlying<'a, T : Send>(env: &'a JNIEnv, class: &'a JClass) -> T {
    let a: T = env.get_rust_field(class, "ptr").unwrap();
    1
}

fn make_zip_entry<'a>(env: &JNIEnv, zip_result: ZipResult<ZipFile>) -> JObject<'a> {
    let file = match zip_result {
        Ok(file) => file,
        Err(ZipError::FileNotFound) => {
            return JObject::null().into();
        }
        Err(e) => {
            env.throw(format!("Failed to open zip entry! {:?}", e)).unwrap();
            unreachable!() // TODO: remove this later
        }
    };

    let zip_entry_class = env.get_object_class("com/github/diamondminer88/zip/ZipEntry").unwrap();
    let constructor = env.get_method_id(zip_entry_class, "<init>", "").unwrap();

    let zip_entry = env.new_object_unchecked(zip_entry_class, constructor, &[]).unwrap();
    env.set_rust_field(zip_entry, "ptr", file);
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

    env.set_rust_field(class, "ptr", zip).unwrap();
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipReader_close(
    env: JNIEnv,
    class: JClass,
) {
    env.take_rust_field(class, "ptr").unwrap();
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipReader_openEntry__I(
    env: JNIEnv,
    class: JClass,
    index: jint,
) {
    let mut zip = jni_underlying::<ZipArchive<File>>(&env, &class);
    let result = zip.by_index(index as usize);

    make_zip_entry(&env, result).into()
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipReader_openEntry__Ljava_lang_String_2(
    env: JNIEnv,
    class: JClass,
    jstr_path: JString,
) -> jobject {
    let path: String = env.get_string(jstr_path).unwrap().into();

    let mut zip = jni_underlying::<ZipArchive<File>>(&env, &class);
    let result = zip.by_name(path.as_str());

    make_zip_entry(&env, result).into()
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipReader_openEntryRaw__I(
    env: JNIEnv,
    class: JClass,
    index: jint,
) {
    let mut zip = jni_underlying::<ZipArchive<File>>(&env, &class);
    let result = zip.by_index_raw(index as usize);

    make_zip_entry(&env, result).into()
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipReader_getEntryCount(
    env: JNIEnv,
    class: JClass,
) -> jlong {
    let zip = jni_underlying::<ZipArchive<File>>(&env, &class);
    zip.len() as i64
}


// ----------JNI ZipEntry----------

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getName(
    env: JNIEnv,
    class: JClass,
) -> jstring {
    let zip = jni_underlying::<ZipFile>(&env, &class);
    env.new_string(zip.name()).unwrap().into()
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getComment(
    env: JNIEnv,
    class: JClass,
) -> jstring {
    let zip = jni_underlying::<ZipFile>(&env, &class);
    env.new_string(zip.comment()).unwrap().into()
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_isDir(
    env: JNIEnv,
    class: JClass,
) -> jboolean {
    let zip = jni_underlying::<ZipFile>(&env, &class);
    zip.is_dir().into()
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getMode(
    env: JNIEnv,
    class: JClass,
) -> jint {
    let zip = jni_underlying::<ZipFile>(&env, &class);
    zip.unix_mode() as i32
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getCRC32(
    env: JNIEnv,
    class: JClass,
) -> jint {
    let zip = jni_underlying::<ZipFile>(&env, &class);
    zip.crc32() as i32
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getExtraData(
    env: JNIEnv,
    class: JClass,
) -> jbyteArray {
    let zip = jni_underlying::<ZipFile>(&env, &class);
    let byte_array = env.new_byte_array(zip.extra_data().len() as jsize).unwrap();
    env.set_byte_array_region(byte_array, 0, zip.extra_data() as &[jbyte]);
    byte_array
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getSize(
    env: JNIEnv,
    class: JClass,
) -> jlong {
    let zip = jni_underlying::<ZipFile>(&env, &class);
    zip.size() as i64
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipEntry_getCompressedSize(
    env: JNIEnv,
    class: JClass,
) -> jlong {
    let zip = jni_underlying::<ZipFile>(&env, &class);
    zip.compressed_size() as i64
}



// Utility functions
/// A port of `JNIEnv::get_rust_field` with type `T` modified to not require `Send`.


fn get_field<'a, O, S, T>(
    env: &JNIEnv<'a>,
    obj: O,
    field: S,
) -> JniResult<ReentrantReference<'a, T>>
    where
        O: Into<JObject<'a>>,
        S: Into<JNIString>,
        T: 'static,
{
    let obj = obj.into();
    let _guard = env.lock_obj(obj)?;

    let ptr = env.get_field(obj, field, "J")?.j()? as *mut ReentrantLock<T>;
    non_null!(ptr, "rust value from Java");
    unsafe {
        // dereferencing is safe, because we checked it for null
        Ok((*ptr).lock().unwrap())
    }
}

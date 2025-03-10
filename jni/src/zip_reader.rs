use crate::{cache, interop};
use catch_panic::catch_panic;
use jni::objects::JByteArray;
use jni::sys::jboolean;
use jni::{
    objects::{JObject, JString},
    sys::{jbyteArray, jint, jobject, jobjectArray, jsize},
    JNIEnv,
};
use jni_fn::jni_fn;
use std::io::{Cursor, Read, Seek};
use std::{fs::File, path::Path};
use zip::{result::ZipError, ZipArchive};

trait ReaderTrait: Read + Seek {}
impl<T: Read + Seek> ReaderTrait for T {}

/// Obtains an exclusive reference to the rust reader from a pointer in a JVM class.
macro_rules! obtain_reader {
    (&mut $env:ident, &$class:ident, $ret_value:expr) => {{
        let reader = crate::interop::get_field::<_, _, ZipArchive<Box<dyn ReaderTrait>>>(
            &mut $env,
            &$class,
            crate::cache::ZipReader_ptr(),
        );

        match reader.unwrap() {
            Some(w) => w,
            None => {
                $env.throw((
                    "java/lang/IllegalStateException",
                    "Cannot use a closed reader!",
                ))
                .unwrap();
                return $ret_value;
            }
        }
    }};
}

#[catch_panic]
#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipReader_open__Ljava_lang_String_2(
    mut env: JNIEnv,
    class: JObject,
    path: JString,
) {
    let path: String = env.get_string(&path).unwrap().into();
    let file = match File::open(Path::new(&path)) {
        Ok(file) => file,
        Err(e) => {
            env.throw(format!("Failed to open file: {:?}", e)).unwrap();
            return;
        }
    };

    let reader: Box<dyn ReaderTrait> = Box::new(file);
    let zip = match ZipArchive::new(reader) {
        Ok(zip) => zip,
        Err(e) => {
            env.throw(format!("Failed to open archive: {:?}", e))
                .unwrap();
            return;
        }
    };

    interop::set_field(&mut env, &class, cache::ZipReader_ptr(), zip).unwrap();
}

#[catch_panic]
#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipReader_open___3B(
    mut env: JNIEnv,
    class: JObject,
    bytes: JByteArray,
) {
    let bytes = env.convert_byte_array(bytes).unwrap();
    let cursor = Cursor::new(bytes);
    let reader: Box<dyn ReaderTrait> = Box::new(cursor);

    let zip = match ZipArchive::new(reader) {
        Ok(zip) => zip,
        Err(e) => {
            env.throw(format!("Failed to parse zip: {:?}", e)).unwrap();
            return;
        }
    };

    interop::set_field(&mut env, &class, cache::ZipReader_ptr(), zip).unwrap();
}

#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipReader")]
pub fn close(mut env: JNIEnv, class: JObject) {
    let _ = interop::take_field::<_, _, ZipArchive<Box<dyn ReaderTrait>>>(
        &mut env,
        &class,
        cache::ZipReader_ptr(),
    );
}

#[catch_panic(default = "JObject::null().into_raw()")]
#[jni_fn("com.github.diamondminer88.zip.ZipReader")]
pub fn openEntry0(
    mut env: JNIEnv,
    class: JObject,
    index: jint,
    name: JString,
    raw: jboolean,
) -> jobject {
    let raw = raw == 1;
    let index = index as usize;
    let name: Option<String> = match name.is_null() {
        true => None,
        false => Some(env.get_string(&name).unwrap().into()),
    };
    let mut zip = obtain_reader!(&mut env, &class, JObject::null().into_raw());

    let zip_result = match (raw, name) {
        (false, Some(name)) => zip.by_name(&*name),
        (true, Some(name)) => zip.by_name_raw(&*name),
        (false, None) => zip.by_index(index),
        (true, None) => zip.by_index_raw(index),
    };
    let zip_file = match zip_result {
        Ok(file) => file,
        Err(ZipError::FileNotFound) => {
            return JObject::null().into_raw();
        }
        Err(e) => {
            env.throw(format!("Failed to open zip entry! {:?}", e))
                .unwrap();
            return JObject::null().into_raw();
        }
    };

    let zip_entry = unsafe {
        env.new_object_unchecked(&cache::ZipEntry(), cache::ZipEntry_ctor(), &[])
            .unwrap()
    };
    interop::set_field(&mut env, &zip_entry, cache::ZipEntry_ptr(), zip_file).unwrap();

    zip_entry.into_raw()
}

#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipReader")]
pub fn getEntryCount(mut env: JNIEnv, class: JObject) -> jint {
    let zip = obtain_reader!(&mut env, &class, 0);
    zip.len() as jint
}

#[catch_panic(default = "JObject::null().into_raw()")]
#[jni_fn("com.github.diamondminer88.zip.ZipReader")]
pub fn getRawComment(mut env: JNIEnv, class: JObject) -> jbyteArray {
    let zip = obtain_reader!(&mut env, &class, JObject::null().into_raw());
    env.byte_array_from_slice(zip.comment()).unwrap().into_raw()
}

#[catch_panic(default = "JObject::null().into_raw()")]
#[jni_fn("com.github.diamondminer88.zip.ZipReader")]
pub fn getEntryNames(mut env: JNIEnv, class: JObject) -> jobjectArray {
    let zip = obtain_reader!(&mut env, &class, JObject::null().into_raw());
    let names_length = zip.file_names().collect::<Vec<&str>>().len();

    let array = env
        .new_object_array(names_length as jsize, &cache::String(), JObject::null())
        .unwrap();

    for (i, name) in zip.file_names().enumerate() {
        let jvm_name = env.auto_local(env.new_string(name).unwrap());
        env.set_object_array_element(&array, i as jsize, jvm_name)
            .unwrap();
    }

    array.into_raw()
}

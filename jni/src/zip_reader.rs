use crate::cache;
use crate::interop::{get_field, set_field, take_field, ReentrantReference};
use catch_panic::catch_panic;
use jni::objects::JByteArray;
use jni::{
    objects::{JObject, JString},
    sys::{jbyteArray, jint, jobject, jobjectArray, jsize},
    JNIEnv,
};
use jni_fn::jni_fn;
use std::io::{Cursor, Read, Seek};
use std::{fs::File, path::Path};
use zip::{
    read::ZipFile,
    result::{ZipError, ZipResult},
    ZipArchive, ZipWriter,
};

trait ReaderTrait: Read + Seek {}
impl<T: Read + Seek> ReaderTrait for T {}

fn get_archive<'a>(
    env: &mut JNIEnv<'a>,
    obj: &JObject<'a>,
) -> ReentrantReference<'a, ZipArchive<Box<dyn ReaderTrait>>> {
    get_field(env, obj, cache::ZipReader_ptr()).unwrap()
}

fn set_archive<'a>(
    env: &mut JNIEnv<'a>,
    obj: &JObject<'a>,
    archive: ZipArchive<Box<dyn ReaderTrait>>,
) {
    set_field(env, obj, cache::ZipReader_ptr(), archive).unwrap();
}

fn set_entry<'a>(env: &mut JNIEnv<'a>, obj: &JObject<'a>, entry: ZipFile) {
    set_field(env, obj, cache::ZipEntry_ptr(), entry).unwrap();
}

fn take_archive<'a>(env: &mut JNIEnv<'a>, obj: &JObject<'a>) -> ZipArchive<Box<dyn ReaderTrait>> {
    take_field(env, obj, cache::ZipReader_ptr()).unwrap()
}

fn make_zip_entry<'a>(env: &mut JNIEnv<'a>, zip_result: ZipResult<ZipFile>) -> JObject<'a> {
    let file = match zip_result {
        Ok(file) => file,
        Err(ZipError::FileNotFound) => {
            return JObject::null().into();
        }
        Err(e) => {
            env.throw(format!("Failed to open zip entry! {:?}", e))
                .unwrap();
            return JObject::null().into();
        }
    };

    let zip_entry = unsafe {
        env.new_object_unchecked(&cache::ZipEntry(), cache::ZipEntry_ctor(), &[])
            .unwrap()
    };
    set_entry(env, &zip_entry, file);
    zip_entry
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
    set_archive(&mut env, &class, zip);
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

    set_archive(&mut env, &class, zip);
}

#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipReader")]
pub fn close(mut env: JNIEnv, class: JObject) {
    take_archive(&mut env, &class);
}

#[catch_panic(default = "std::ptr::null_mut()")]
#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipReader_openEntry__I(
    mut env: JNIEnv,
    class: JObject,
    index: jint,
) -> jobject {
    let index = index as usize;

    let mut zip = get_archive(&mut env, &class);
    let result = zip.by_index(index);

    make_zip_entry(&mut env, result).into_raw()
}

#[catch_panic(default = "std::ptr::null_mut()")]
#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipReader_openEntry__Ljava_lang_String_2(
    mut env: JNIEnv,
    class: JObject,
    path: JString,
) -> jobject {
    let path: String = env.get_string(&path).unwrap().into();

    let mut zip = get_archive(&mut env, &class);
    let result = zip.by_name(path.as_str());

    make_zip_entry(&mut env, result).into_raw()
}

#[catch_panic(default = "std::ptr::null_mut()")]
#[jni_fn("com.github.diamondminer88.zip.ZipReader")]
pub fn openEntryRaw(mut env: JNIEnv, class: JObject, index: jint) -> jobject {
    let index = index as usize;

    let mut zip = get_archive(&mut env, &class);
    let result = zip.by_index_raw(index);

    make_zip_entry(&mut env, result).into_raw()
}

#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipReader")]
pub fn getEntryCount(mut env: JNIEnv, class: JObject) -> jint {
    let zip = get_archive(&mut env, &class);
    zip.len() as jint
}

#[catch_panic(default = "std::ptr::null_mut()")]
#[jni_fn("com.github.diamondminer88.zip.ZipReader")]
pub fn getRawComment(mut env: JNIEnv, class: JObject) -> jbyteArray {
    let zip = get_archive(&mut env, &class);
    env.byte_array_from_slice(zip.comment()).unwrap().into_raw()
}

#[catch_panic(default = "std::ptr::null_mut()")]
#[jni_fn("com.github.diamondminer88.zip.ZipReader")]
pub fn getEntryNames(mut env: JNIEnv, class: JObject) -> jobjectArray {
    let zip = get_archive(&mut env, &class);
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

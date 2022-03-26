use std::{
    fs::File,
    io::{Cursor, Write},
    path::Path,
};

use jni::{
    JNIEnv,
    objects::{JClass, JString},
    sys::{jboolean, jbyteArray},
};
use zip::write::FileOptions;
use zip::ZipWriter;

use crate::cache;
use crate::interop::{get_field, ReentrantReference, set_field};

fn get_writer<'a>(env: &JNIEnv<'a>, obj: JClass<'a>) -> ReentrantReference<'a, ZipWriter<File>> {
    get_field(&env, obj, cache::fld_zipwriter_ptr()).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipWriter_open__Ljava_lang_String_2Z(
    env: JNIEnv,
    class: JClass,
    jstr_path: JString,
    jbool_append: jboolean,
) {
    let append = jbool_append != 0;
    let path: String = env.get_string(jstr_path).unwrap().into();

    let fopen_result = File::options()
        .read(true)
        .write(true)
        .open(Path::new(&path));

    let file = match fopen_result {
        Ok(file) => file,
        Err(e) => {
            env.throw(format!("Failed to open file: {:?}", e)).unwrap();
            return;
        }
    };

    let writer = if append {
        ZipWriter::new(file)
    } else {
        match ZipWriter::new_append(file) {
            Ok(w) => w,
            Err(e) => {
                env.throw(format!("Failed to open zip in append mode: {:?}", e)).unwrap();
                return;
            }
        }
    };

    set_field(&env, class, cache::fld_zipwriter_ptr(), writer).unwrap();
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipWriter_open___3B(
    env: JNIEnv,
    class: JClass,
    jbytes: jbyteArray,
) {
    let bytes = env.convert_byte_array(jbytes).unwrap();
    let cursor = Cursor::new(bytes);

    let writer = match ZipWriter::new_append(cursor) {
        Ok(w) => w,
        Err(e) => {
            env.throw(format!("Failed to open zip in append mode: {:?}", e)).unwrap();
            return;
        }
    };

    set_field(&env, class, cache::fld_zipwriter_ptr(), writer).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipWriter_setComment(
    env: JNIEnv,
    class: JClass,
    jbytes: jbyteArray,
) {
    let mut writer = get_writer(&env, class);
    let bytes = env.convert_byte_array(jbytes).unwrap();

    writer.set_raw_comment(bytes);
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipWriter_writeEntry(
    env: JNIEnv,
    class: JClass,
    jpath: JString,
    jbytes: jbyteArray,
) {
    let mut writer = get_writer(&env, class);
    let path = env.get_string(jpath).unwrap();
    let bytes = env.convert_byte_array(jbytes).unwrap();

    writer.start_file(path, FileOptions::default()).unwrap();
    writer.write_all(&bytes).unwrap();
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipWriter_close(
    env: JNIEnv,
    class: JClass,
) {
    let mut writer = get_writer(&env, class);
    match writer.finish() {
        Err(e) => env.throw(format!("Failed to close zip: {:?}", e)).unwrap(),
        _ => {}
    }
}

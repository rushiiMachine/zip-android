use std::{
    fs::{File, OpenOptions},
    io::{Cursor, Seek, SeekFrom, Write},
    path::Path,
};

use jni::{
    JNIEnv,
    objects::{JClass, JString},
    sys::{jboolean, jbyteArray, jobjectArray, jsize},
};
use jni::sys::jshort;
use jni_fn::jni_fn;
use zip::{CompressionMethod, ZipArchive, ZipWriter};
use zip::write::FileOptions;

use crate::cache;
use crate::interop::{get_field, ReentrantReference, set_field, take_field};

fn set_writer<'a>(env: &JNIEnv<'a>, obj: JClass<'a>, writer: ZipWriter<File>) {
    set_field(&env, obj, cache::fld_zipwriter_ptr(), writer).unwrap()
}

fn get_writer<'a>(env: &JNIEnv<'a>, obj: JClass<'a>) -> ReentrantReference<'a, ZipWriter<File>> {
    get_field(&env, obj, cache::fld_zipwriter_ptr()).unwrap()
}

fn take_writer<'a>(env: &JNIEnv<'a>, obj: JClass<'a>) -> ZipWriter<File> {
    take_field(&env, obj, cache::fld_zipwriter_ptr()).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipWriter_open__Ljava_lang_String_2Z(
    env: JNIEnv,
    class: JClass,
    path: JString,
    append: jboolean,
) {
    let append = append != 0;
    let path: String = env.get_string(path).unwrap().into();

    let fopen = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(!append)
        .open(Path::new(&path));

    let file = match fopen {
        Ok(file) => file,
        Err(e) => {
            env.throw(format!("Failed to open file: {:?}", e)).unwrap();
            return;
        }
    };

    let writer = if !append {
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

    set_writer(&env, class, writer);
}

// #[no_mangle]
// pub extern "system" fn Java_com_github_diamondminer88_zip_ZipWriter_open___3B(
//     env: JNIEnv,
//     class: JClass,
//     bytes: jbyteArray,
// ) {
//     let bytes = env.convert_byte_array(bytes).unwrap();
//     let cursor = Cursor::new(bytes);
//
//     let writer = match ZipWriter::new_append(cursor) {
//         Ok(w) => w,
//         Err(e) => {
//             env.throw(format!("Failed to open zip in append mode: {:?}", e)).unwrap();
//             return;
//         }
//     };
//
//     set_writer(&env, class, writer);
// }

#[jni_fn("com.github.diamondminer88.zip.ZipWriter")]
pub fn setComment(
    env: JNIEnv,
    class: JClass,
    bytes: jbyteArray,
) {
    let bytes = env.convert_byte_array(bytes).unwrap();
    let mut writer = get_writer(&env, class);

    writer.set_raw_comment(bytes);
}

#[jni_fn("com.github.diamondminer88.zip.ZipWriter")]
pub fn writeEntry(
    env: JNIEnv,
    class: JClass,
    path: JString,
    bytes: jbyteArray,
) {
    let path = env.get_string(path).unwrap();
    let bytes = env.convert_byte_array(bytes).unwrap();

    let mut writer = get_writer(&env, class);

    writer.start_file(path, FileOptions::default()).unwrap();
    writer.write_all(&bytes).unwrap();
}

#[jni_fn("com.github.diamondminer88.zip.ZipWriter")]
pub fn writeEntryUncompressed(
    env: JNIEnv,
    class: JClass,
    path: JString,
    bytes: jbyteArray,
) {
    let path = env.get_string(path).unwrap();
    let bytes = env.convert_byte_array(bytes).unwrap();
    let mut writer = get_writer(&env, class);

    let options = FileOptions::default()
        .compression_method(CompressionMethod::Stored);

    writer.start_file(path, options).unwrap();
    writer.write_all(&bytes).unwrap();
}

#[jni_fn("com.github.diamondminer88.zip.ZipWriter")]
pub fn writeUncompressedAligned(
    env: JNIEnv,
    class: JClass,
    path: JString,
    alignment: jshort,
    bytes: jbyteArray,
) {
    let path = env.get_string(path).unwrap();
    let alignment = alignment as u16;
    let bytes = env.convert_byte_array(bytes).unwrap();
    let mut writer = get_writer(&env, class);

    let options = FileOptions::default()
        .compression_method(CompressionMethod::Stored);

    writer.start_file_aligned(path, options, alignment).unwrap();
    writer.write_all(&bytes).unwrap();
}

#[jni_fn("com.github.diamondminer88.zip.ZipWriter")]
pub fn writeAligned(
    env: JNIEnv,
    class: JClass,
    path: JString,
    alignment: jshort,
    bytes: jbyteArray,
) {
    let path = env.get_string(path).unwrap();
    let alignment = alignment as u16;
    let bytes = env.convert_byte_array(bytes).unwrap();
    let mut writer = get_writer(&env, class);

    writer.start_file_aligned(
        path,
        FileOptions::default(),
        alignment,
    ).unwrap();
    writer.write_all(&bytes).unwrap();
}

#[jni_fn("com.github.diamondminer88.zip.ZipWriter")]
pub fn writeDir(
    env: JNIEnv,
    class: JClass,
    path: JString,
) {
    let path = env.get_string(path).unwrap();
    let mut writer = get_writer(&env, class);

    let options = FileOptions::default()
        .compression_method(CompressionMethod::Stored);
    writer.add_directory(path, options).unwrap();
}

#[jni_fn("com.github.diamondminer88.zip.ZipWriter")]
pub fn close(
    env: JNIEnv,
    class: JClass,
) {
    let mut writer = take_writer(&env, class);
    match writer.finish() {
        Err(e) => env.throw(format!("Failed to close zip: {:?}", e)).unwrap(),
        _ => {}
    }
}


#[jni_fn("com.github.diamondminer88.zip.ZipWriter")]
pub fn deleteEntries(
    env: JNIEnv,
    class: JClass,
    jignored: jobjectArray,
) {
    let arr_length = env.get_array_length(jignored).unwrap() as usize;
    let ignored: Vec<String> = (0..arr_length)
        .map(|i| {
            let obj = env.get_object_array_element(jignored, i as jsize).unwrap();
            env.get_string(obj.into()).unwrap().into()
        })
        .collect();

    let mut old_writer = take_writer(&env, class.clone());
    let old_file = old_writer.finish().unwrap();
    drop(old_writer);

    let mut reader = ZipArchive::new(old_file).unwrap();
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));

    writer.set_raw_comment(reader.comment().into());

    for i in 0..reader.len() {
        let entry = reader.by_index_raw(i).unwrap();
        if !ignored.contains(&entry.name().to_string()) {
            writer.raw_copy_file(entry).unwrap();
        }
    }

    let bytes = writer.finish().unwrap().into_inner();
    drop(writer);

    let mut file = reader.into_inner();
    file.set_len(0).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    file.write_all(bytes.as_slice()).unwrap();
    drop(bytes);

    let archive = match ZipWriter::new_append(file) {
        Ok(a) => a,
        Err(e) => {
            env.throw(format!("Failed to open archive after deleting entries: {:?}", e)).unwrap();
            return;
        }
    };
    set_writer(&env, class, archive);
}

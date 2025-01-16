use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use catch_panic::catch_panic;

use jni::{
    JNIEnv,
    objects::{JClass, JString},
    sys::{jboolean, jbyteArray, jobjectArray, jsize},
};
use jni::sys::{jbyte, jshort};
use jni_fn::jni_fn;

use zip::{CompressionMethod, ZipWriter};
use zip::result::ZipError;
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

#[catch_panic]
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

// #[catch_panic]
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

#[catch_panic]
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

#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipWriter")]
pub fn writeEntry(
    env: JNIEnv,
    class: JClass,
    path: JString,
    bytes: jbyteArray,
    compression: jbyte,
    alignment: jshort,
) {
    let mut writer = get_writer(&env, class);
    let bytes = env.convert_byte_array(bytes).unwrap();
    let path = env.get_string(path).unwrap();
    let alignment = alignment as u16;
    let compression = match compression {
        -1 => None,
        0 => Some(CompressionMethod::Stored),
        1 => Some(CompressionMethod::Deflated),
        2 => Some(CompressionMethod::Bzip2),
        3 => Some(CompressionMethod::Zstd),
        _ => None,
    };

    if compression.is_none() {
        env.throw("Invalid compression type supplied!").unwrap();
        return;
    }

    let options = FileOptions::default()
        // .large_file(bytes.len() >= (1024 * 1024 * 1024 * 4)) // 4 GiB
        .compression_method(compression.unwrap());

    if alignment > 0 {
        writer.start_file_aligned(path, options, alignment).unwrap();
    } else {
        writer.start_file(path, options).unwrap()
    }
    writer.write_all(&bytes).unwrap();
}

#[catch_panic]
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

#[catch_panic]
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

#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipWriter")]
pub fn deleteEntry(
    env: JNIEnv,
    class: JClass,
    path: JString,
    fill_void: jboolean,
) {
    let path = env.get_string(path).unwrap();
    let fill_void = fill_void == 1;
    let mut writer = get_writer(&env, class);

    if let Err(err) = writer.remove_file(path, fill_void) {
        match err {
            ZipError::FileNotFound => {
                env.throw("Cannot find the target entry to delete!").unwrap();
            }
            _ => {
                env.throw("Unknown error trying to delete entry!").unwrap();
            }
        }
    }
}

#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipWriter")]
pub fn deleteEntries(
    env: JNIEnv,
    class: JClass,
    entries: jobjectArray,
) {
    let entries_len = env.get_array_length(entries).unwrap() as usize;
    let entries: Vec<String> = (0..entries_len)
        .map(|i| {
            let obj = env.auto_local(env.get_object_array_element(entries, i as jsize).unwrap());
            env.get_string(obj.as_obj().into()).unwrap().into()
        })
        .collect();

    let mut writer = get_writer(&env, class);

    for name in entries {
        if let Err(err) = writer.remove_file(name, false) {
            match err {
                ZipError::FileNotFound => {
                    env.throw("Cannot find the target entry to delete!").unwrap();
                }
                _ => {
                    env.throw("Unknown error trying to delete entry!").unwrap();
                }
            }
        }
    }
}

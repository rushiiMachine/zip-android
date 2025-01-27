use catch_panic::catch_panic;
use std::fs::{File, OpenOptions};
use std::io::{Cursor, Read, Seek, Write};
use std::path::Path;

use jni::objects::{JByteArray, JObjectArray};
use jni::sys::{jbyte, jbyteArray, jshort};
use jni::{
    objects::{JObject, JString},
    sys::{jboolean, jsize},
    JNIEnv,
};
use jni_fn::jni_fn;

use crate::cache;
use crate::interop::{get_field, set_field, take_field, ReentrantReference};
use zip::result::ZipError;
use zip::truncate::Truncate;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

trait WriterTrait: Write + Seek + Read + Truncate {
    fn into_bytes(self: Box<Self>) -> Option<Vec<u8>>;
}
impl WriterTrait for File {
    fn into_bytes(self: Box<Self>) -> Option<Vec<u8>> {
        None
    }
}
impl WriterTrait for Cursor<Vec<u8>> {
    fn into_bytes(self: Box<Self>) -> Option<Vec<u8>> {
        Some(self.into_inner())
    }
}

fn set_writer<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>, zip: ZipWriter<Box<dyn WriterTrait>>) {
    set_field(env, obj, cache::ZipWriter_ptr(), zip).unwrap()
}

fn get_writer<'a>(
    env: &mut JNIEnv<'a>,
    obj: JObject<'a>,
) -> ReentrantReference<'a, ZipWriter<Box<dyn WriterTrait>>> {
    get_field(env, obj, cache::ZipWriter_ptr()).unwrap()
}

fn take_writer<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> ZipWriter<Box<dyn WriterTrait>> {
    take_field(env, obj, cache::ZipWriter_ptr()).unwrap()
}

#[catch_panic]
#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipWriter_open__Ljava_lang_String_2Z(
    mut env: JNIEnv,
    class: JObject,
    path: JString,
    append: jboolean,
) {
    let append = append != 0;
    let path: String = env.get_string(&path).unwrap().into();

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

    let writer: Box<dyn WriterTrait> = Box::new(file);
    let zip = if !append {
        ZipWriter::new(writer)
    } else {
        match ZipWriter::new_append(writer) {
            Ok(w) => w,
            Err(e) => {
                env.throw(format!("Failed to open zip in append mode: {:?}", e))
                    .unwrap();
                return;
            }
        }
    };

    set_writer(&mut env, class, zip);
}

#[catch_panic]
#[no_mangle]
pub extern "system" fn Java_com_github_diamondminer88_zip_ZipWriter_open___3B(
    mut env: JNIEnv,
    class: JObject,
    bytes: JByteArray,
) {
    let bytes = env.convert_byte_array(bytes).unwrap();
    let cursor = Cursor::new(bytes);
    let writer: Box<dyn WriterTrait> = Box::new(cursor);

    let zip = match ZipWriter::new_append(writer) {
        Ok(w) => w,
        Err(e) => {
            env.throw(format!("Failed to parse zip: {:?}", e)).unwrap();
            return;
        }
    };

    set_writer(&mut env, class, zip);
}

#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipWriter")]
pub fn setComment(mut env: JNIEnv, class: JObject, bytes: JByteArray) {
    let bytes = env.convert_byte_array(bytes).unwrap();
    let mut writer = get_writer(&mut env, class);

    writer.set_raw_comment(bytes);
}

#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipWriter")]
pub fn writeEntry(
    mut env: JNIEnv,
    class: JObject,
    path: JString,
    bytes: JByteArray,
    compression: jbyte,
    alignment: jshort,
) {
    let mut writer = get_writer(&mut env, class);
    let bytes = env.convert_byte_array(bytes).unwrap();
    let path = env.get_string(&path).unwrap();
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
pub fn writeDir(mut env: JNIEnv, class: JObject, path: JString) {
    let path = env.get_string(&path).unwrap();
    let mut writer = get_writer(&mut env, class);

    let options = FileOptions::default().compression_method(CompressionMethod::Stored);
    writer.add_directory(path, options).unwrap();
}

#[catch_panic(default = "std::ptr::null_mut()")]
#[jni_fn("com.github.diamondminer88.zip.ZipWriter")]
pub fn toByteArray(mut env: JNIEnv, class: JObject) -> jbyteArray {
    match take_writer(&mut env, class).finish() {
        Err(e) => {
            env.throw(format!("Failed to close zip: {:?}", e)).unwrap();
            JObject::null().into_raw()
        }
        Ok(writer) => match writer.into_bytes() {
            None => {
                env.throw("Cannot convert an archive opened from a file into bytes!")
                    .unwrap();
                JObject::null().into_raw()
            }
            Some(bytes) => env.byte_array_from_slice(&*bytes).unwrap().into_raw(),
        },
    }
}

#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipWriter")]
pub fn close(mut env: JNIEnv, class: JObject) {
    let writer = take_field(&mut env, &class, cache::ZipWriter_ptr());
    let mut writer: ZipWriter<Box<dyn WriterTrait>> = match writer {
        Ok(w) => w,
        Err(_) => return,
    };

    match writer.finish() {
        Err(e) => env.throw(format!("Failed to close zip: {:?}", e)).unwrap(),
        _ => {}
    }
}

#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipWriter")]
pub fn deleteEntry(mut env: JNIEnv, class: JObject, path: JString, fill_void: jboolean) {
    let path = env.get_string(&path).unwrap();
    let fill_void = fill_void == 1;
    let mut writer = get_writer(&mut env, class);

    if let Err(err) = writer.remove_file(path, fill_void) {
        match err {
            ZipError::FileNotFound => {
                env.throw("Cannot find the target entry to delete!")
                    .unwrap();
            }
            _ => {
                env.throw("Unknown error trying to delete entry!").unwrap();
            }
        }
    }
}

#[catch_panic]
#[jni_fn("com.github.diamondminer88.zip.ZipWriter")]
pub fn deleteEntries(mut env: JNIEnv, class: JObject, entries: JObjectArray) {
    let entries_len = env.get_array_length(&entries).unwrap() as usize;
    let entries: Vec<String> = (0..entries_len)
        .map(|i| {
            let obj = env.get_object_array_element(&entries, i as jsize).unwrap();
            let obj = env.auto_local(obj);
            env.get_string((&*obj).into()).unwrap().into()
        })
        .collect();

    let mut writer = get_writer(&mut env, class);

    for name in entries {
        if let Err(err) = writer.remove_file(name, false) {
            match err {
                ZipError::FileNotFound => {
                    env.throw("Cannot find the target entry to delete!")
                        .unwrap();
                }
                _ => {
                    env.throw("Unknown error trying to delete entry!").unwrap();
                }
            }
        }
    }
}

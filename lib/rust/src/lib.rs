#![feature(thread_id_value)]

use jni::sys::jbyte;

mod interop;
mod zip_entry;
mod zip_archive;
mod cache;

pub fn bytes_to_jbytes(bytes: &[u8]) -> &[jbyte] {
    unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const jbyte, bytes.len()) }
}

#![feature(thread_id_value)]
#![allow(non_snake_case)]

use std::os::raw::c_void;

use jni::sys::JNI_ERR;
use jni::{
    sys::{jint, JNI_VERSION_1_6},
    JavaVM,
};

mod cache;
mod interop;
mod zip_entry;
mod zip_reader;
mod zip_writer;

#[no_mangle]
pub unsafe extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: c_void) -> jint {
    let env = vm.get_env().unwrap();

    if !cache::init(env) {
        return JNI_ERR;
    }

    JNI_VERSION_1_6
}

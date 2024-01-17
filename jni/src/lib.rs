#![feature(thread_id_value)]
#![allow(non_snake_case)]

use std::os::raw::c_void;

use jni::{
    JavaVM,
    sys::{jint, JNI_VERSION_1_6},
};

mod interop;
mod zip_entry;
mod zip_reader;
mod cache;
mod zip_writer;

#[no_mangle]
pub unsafe extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: c_void) -> jint {
    let env = vm.get_env().unwrap();

    cache::init(env);

    JNI_VERSION_1_6
}

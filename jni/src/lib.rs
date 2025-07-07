#![feature(thread_id_value)]
#![allow(non_snake_case)]

use std::os::raw::c_void;

use jni::sys::JNI_ERR;
use jni::{
    sys::{jint, JNI_VERSION_1_6},
    JavaVM,
};
use log::error;

mod cache;
mod interop;
mod zip_entry;
mod zip_reader;
mod zip_writer;

#[no_mangle]
pub unsafe extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: c_void) -> jint {
    #[cfg(debug_assertions)]
    android_log::init("zip-android").unwrap();

    let mut env = vm.get_env().unwrap();

    if let Err(err) = cache::init(&mut env) {
        error!("Failed to initialize cache: {err}");

        // I can't leave this function with a thrown exception, otherwise
        // the VM will crash internally for some reason.
        if matches!(err, jni::errors::Error::JavaException) {
            env.exception_describe().unwrap();
            env.exception_clear().unwrap();
        }

        return JNI_ERR;
    }

    JNI_VERSION_1_6
}

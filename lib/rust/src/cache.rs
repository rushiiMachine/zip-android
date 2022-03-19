use std::os::raw::c_void;

use jni::{
    descriptors::Desc,
    JavaVM,
    JNIEnv,
    objects::{GlobalRef, JClass, JMethodID, JStaticMethodID},
    sys::{jint, JNI_VERSION_1_6},
};

static mut CLASS_ZIP_ENTRY: Option<GlobalRef> = None;
static mut CLASS_STRING: Option<GlobalRef> = None;
static mut CLASS_DATE: Option<GlobalRef> = None;

static mut CTOR_ZIP_ENTRY: Option<JMethodID> = None;
static mut METHOD_DATE_UTC: Option<JStaticMethodID> = None;

pub fn class_zip_entry() -> GlobalRef {
    unsafe { CLASS_ZIP_ENTRY.clone().unwrap() }
}

pub fn class_string() -> GlobalRef {
    unsafe { CLASS_STRING.clone().unwrap() }
}

pub fn class_date() -> GlobalRef {
    unsafe { CLASS_DATE.clone().unwrap() }
}

pub fn ctor_zip_entry() -> JMethodID<'static> {
    unsafe { CTOR_ZIP_ENTRY.unwrap() }
}

pub fn method_date_utc() -> JStaticMethodID<'static> {
    unsafe { METHOD_DATE_UTC.unwrap() }
}

#[no_mangle]
pub unsafe extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: c_void) -> jint {
    let env = vm.get_env().unwrap();

    CLASS_ZIP_ENTRY = get_class(&env, "com/github/diamondminer88/zip/ZipEntry");
    CLASS_STRING = get_class(&env, "java/lang/String");
    CLASS_DATE = get_class(&env, "java/util/Date");

    CTOR_ZIP_ENTRY = get_method(
        &env,
        "com/github/diamondminer88/zip/ZipEntry",
        "<init>",
        "()V",
        false,
    ).instance;

    METHOD_DATE_UTC = get_method(
        &env,
        "java/util/Date",
        "UTC",
        "(IIIIII)J",
        true,
    ).static_;

    JNI_VERSION_1_6
}

fn get_class(env: &JNIEnv, class: &str) -> Option<GlobalRef> {
    let cls = env.find_class(class).unwrap();
    let cls_ref = env.new_global_ref(cls).unwrap();
    Some(cls_ref)
}

fn get_method<'c, C>(env: &JNIEnv<'c>, class: C, name: &str, sig: &str, is_static: bool) -> UnionGetMethod
    where C: Desc<'c, JClass<'c>>
{
    if is_static {
        let id = env.get_static_method_id(class, name, sig).unwrap()
            .into_inner().into();
        UnionGetMethod { static_: Some(id) }
    } else {
        let id = env.get_method_id(class, name, sig).unwrap()
            .into_inner().into();
        UnionGetMethod { instance: Some(id) }
    }
}

union UnionGetMethod {
    instance: Option<JMethodID<'static>>,
    static_: Option<JStaticMethodID<'static>>,
}

use std::os::raw::c_void;

use jni::{
    descriptors::Desc,
    JavaVM,
    JNIEnv,
    objects::{GlobalRef, JClass, JFieldID, JMethodID, JStaticMethodID},
    sys::{jint, JNI_VERSION_1_6},
};

// If anyone wants to improve these, please do
// Specifically inst_method + static_method could be merged into one
// I barely know anything about rust macros
macro_rules! class {
    ($variable:ident, $getter:ident) => {
        static mut $variable: Option<GlobalRef> = None;
        pub fn $getter() -> GlobalRef {
            unsafe { $variable.clone().unwrap() }
        }
    }
}

macro_rules! inst_method {
    ($variable:ident, $getter:ident) => {
        static mut $variable: Option<JMethodID> = None;
        pub fn $getter() -> JMethodID<'static> {
            unsafe { $variable.unwrap() }
        }
    }
}

macro_rules! static_method {
    ($variable:ident, $getter:ident) => {
        static mut $variable: Option<JStaticMethodID> = None;
        pub fn $getter() -> JStaticMethodID<'static> {
            unsafe { $variable.unwrap() }
        }
    }
}

macro_rules! field {
    ($variable:ident, $getter:ident) => {
        static mut $variable: Option<JFieldID> = None;
        pub fn $getter() -> JFieldID<'static> {
            unsafe { $variable.unwrap() }
        }
    }
}

class!(CLS_ZIPENTRY, cls_zipentry);
class!(CLS_STRING, cls_string);
class!(CLS_DATE, cls_date);
inst_method!(CTOR_ZIPENTRY, ctor_zipentry);
static_method!(MTOD_DATE_UTC, mtod_date_utc);
field!(FLD_ZIPENTRY_PTR, fld_zipentry_ptr);
field!(FLD_ZIPREADER_PTR, fld_zipreader_ptr);
field!(FLD_ZIPWRITER_PTR, fld_zipwriter_ptr);

#[no_mangle]
pub unsafe extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: c_void) -> jint {
    let env = vm.get_env().unwrap();

    CLS_ZIPENTRY = get_class(&env, "com/github/diamondminer88/zip/ZipEntry");
    CLS_STRING = get_class(&env, "java/lang/String");
    CLS_DATE = get_class(&env, "java/util/Date");

    CTOR_ZIPENTRY = get_method(
        &env,
        "com/github/diamondminer88/zip/ZipEntry",
        "<init>",
        "()V",
        false,
    ).instance;

    MTOD_DATE_UTC = get_method(
        &env,
        "java/util/Date",
        "UTC",
        "(IIIIII)J",
        true,
    ).static_;

    FLD_ZIPENTRY_PTR = get_field(
        &env,
        "com/github/diamondminer88/zip/ZipEntry",
        "ptr",
        "J",
    );

    FLD_ZIPREADER_PTR = get_field(
        &env,
        "com/github/diamondminer88/zip/ZipReader",
        "ptr",
        "J",
    );

    FLD_ZIPWRITER_PTR = get_field(
        &env,
        "com/github/diamondminer88/zip/ZipWriter",
        "ptr",
        "J",
    );

    JNI_VERSION_1_6
}

fn get_class(env: &JNIEnv, class: &str) -> Option<GlobalRef> {
    let cls = env.find_class(class).unwrap();
    let cls_ref = env.new_global_ref(cls).unwrap();
    Some(cls_ref)
}

fn get_field<'a, C>(env: &JNIEnv<'a>, class: C, name: &str, sig: &str) -> Option<JFieldID<'static>>
    where C: Desc<'a, JClass<'a>>
{
    let field = env.get_field_id(class, name, sig).unwrap()
        .into_inner().into();
    Some(field)
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

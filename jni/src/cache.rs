use catch_panic::catch_panic;
use jni::objects::{GlobalRef, JFieldID, JMethodID, JStaticMethodID};
use jni::JNIEnv;
use std::sync::Mutex;

macro_rules! cache_ref {
    ($name:ident : $ty:ty) => {
        paste::paste! {
            #[allow(non_upper_case_globals)]
            static [<INNER_ $name>]: Mutex<Option<$ty>> = Mutex::new(None);

            /// Unwraps this cache member as long as `JNI_OnUnload` has not been called yet.
            /// The values returned by this should be disposed of as quickly as possible and not held.
            #[allow(non_snake_case)]
            pub fn $name() -> $ty
                where $ty: Clone
            {
                [<INNER_ $name>].lock()
                    .expect("jni_cache mutex lock fail")
                    .as_ref()
                    .expect("JNI cache already cleaned up")
                    .clone()
            }

            /// Initializes this global cached value. If it already contains a value, a panic occurs.
            #[allow(non_snake_case)]
            fn [<init_ $name>](value: $ty)
                where $ty: Clone
            {
                let mut option = [<INNER_ $name>].lock()
                    .expect("jni_cache mutex lock fail");

                match option.as_ref() {
                    Some(_) => panic!("jni_cache member already initialized"),
                    None => { *option = Some(value); }
                };
            }
        }
    };
}

// Java Stdlib
cache_ref!(String: GlobalRef);
cache_ref!(Date: GlobalRef);
cache_ref!(Date_UTC: JStaticMethodID);

// zip-android
cache_ref!(ZipReader: GlobalRef);
cache_ref!(ZipReader_ptr: JFieldID);
cache_ref!(ZipWriter: GlobalRef);
cache_ref!(ZipWriter_ptr: JFieldID);
cache_ref!(ZipEntry: GlobalRef);
cache_ref!(ZipEntry_ctor: JMethodID);
cache_ref!(ZipEntry_ptr: JFieldID);

#[catch_panic(default = "false")]
pub(super) fn init(mut env: JNIEnv) -> bool {
    fn class_ref(env: &mut JNIEnv, name: &str) -> GlobalRef {
        env.find_class(name)
            .and_then(|cls| env.new_global_ref(cls))
            .expect("failed to get class for jni_cache member")
    }

    // TODO: better errors when failing to unwrap so users can figure out proguard issues

    // Java Stdlib
    init_String(class_ref(&mut env, "java/lang/String"));
    init_Date(class_ref(&mut env, "java/util/Date"));
    init_Date_UTC(
        env.get_static_method_id(&Date(), "UTC", "(IIIIII)J")
            .unwrap(),
    );

    // zip-android
    init_ZipReader(class_ref(
        &mut env,
        "com/github/diamondminer88/zip/ZipReader",
    ));
    init_ZipReader_ptr(env.get_field_id(&ZipReader(), "ptr", "J").unwrap());
    init_ZipWriter(class_ref(
        &mut env,
        "com/github/diamondminer88/zip/ZipWriter",
    ));
    init_ZipWriter_ptr(env.get_field_id(&ZipWriter(), "ptr", "J").unwrap());
    init_ZipEntry(class_ref(
        &mut env,
        "com/github/diamondminer88/zip/ZipEntry",
    ));
    init_ZipEntry_ctor(env.get_method_id(&ZipEntry(), "<init>", "()V").unwrap());
    init_ZipEntry_ptr(env.get_field_id(&ZipEntry(), "ptr", "J").unwrap());

    true
}

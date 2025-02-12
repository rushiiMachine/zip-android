// Modified version of interop.rs from here:
// https://github.com/kawamuray/wasmtime-java

use std::{
    sync::atomic::{AtomicU64, Ordering},
    sync::Mutex,
};

use jni::signature::ReturnType;
use jni::{
    errors::Result as JniResult,
    objects::{JFieldID, JObject},
    signature::Primitive,
    sys::jlong,
    JNIEnv,
};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("JNI error: {0}")]
    Jni(#[from] jni::errors::Error),
    #[error("{0}")]
    LockPoison(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl<G> From<std::sync::PoisonError<G>> for Error {
    fn from(err: std::sync::PoisonError<G>) -> Self {
        Error::LockPoison(err.to_string())
    }
}

pub struct ReentrantLock<T> {
    mutex: Mutex<T>,
    current_owner: AtomicU64,
}

pub enum ReentrantReference<'a, T> {
    Locked(&'a ReentrantLock<T>, std::sync::MutexGuard<'a, T>),
    Recursive(&'a mut T),
}

impl<'a, T> Drop for ReentrantReference<'a, T> {
    fn drop(&mut self) {
        match self {
            ReentrantReference::Locked(lock, _) => {
                lock.current_owner.store(0, Ordering::Relaxed);
            }
            ReentrantReference::Recursive(_) => {}
        }
    }
}

impl<T> ReentrantLock<T> {
    pub fn new(val: T) -> Self {
        Self {
            mutex: Mutex::new(val),
            current_owner: AtomicU64::new(0),
        }
    }

    fn lock(&mut self) -> Result<ReentrantReference<'_, T>> {
        let current_id = std::thread::current().id().as_u64().get();
        if current_id == self.current_owner.load(Ordering::Relaxed) {
            let reference = self.mutex.get_mut()?;
            return Ok(ReentrantReference::Recursive(reference));
        }
        let guard = self.mutex.lock()?;
        self.current_owner.store(current_id, Ordering::Relaxed);
        Ok(ReentrantReference::Locked(self, guard))
    }
}

impl<'a, T> std::ops::Deref for ReentrantReference<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            ReentrantReference::Locked(_, lock) => &*lock,
            ReentrantReference::Recursive(r) => r,
        }
    }
}

impl<'a, T> std::ops::DerefMut for ReentrantReference<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            ReentrantReference::Locked(_, lock) => &mut *lock,
            ReentrantReference::Recursive(r) => r,
        }
    }
}

/// Surrender a Rust object into a pointer.
/// The given value gets "forgotten" by Rust's memory management
/// so you have to get it back into a `T` at some point to avoid leaking memory.
pub fn into_raw<T>(val: T) -> jlong {
    Box::into_raw(Box::new(ReentrantLock::new(val))) as jlong
}

/// A port of `JNIEnv::set_rust_field` with type `T` modified to not require `Send`.
/// It still preserves Mutex around the value, not for atomic access but for making sure
/// the unique owner at the time it is taken.
pub fn set_field<'local, O, S, T>(
    env: &mut JNIEnv<'local>,
    obj: O,
    field_id: S,
    rust_object: T,
) -> JniResult<()>
where
    O: AsRef<JObject<'local>>,
    S: Into<JFieldID>,
{
    let _guard = env.lock_obj(&obj)?;

    let ptr = into_raw(rust_object);
    env.set_field_unchecked(&obj, field_id.into(), ptr.into())
}

/// A port of `JNIEnv::get_rust_field` with type `T` modified to not require `Send`.
pub fn get_field<'local, O, S, T>(
    env: &mut JNIEnv<'local>,
    obj: O,
    field: S,
) -> JniResult<Option<ReentrantReference<'local, T>>>
where
    O: AsRef<JObject<'local>>,
    S: Into<JFieldID>,
    T: 'static,
{
    let _guard = env.lock_obj(&obj)?;

    let ptr = env
        .get_field_unchecked(&obj, field.into(), ReturnType::Primitive(Primitive::Long))?
        .j()? as *mut ReentrantLock<T>;

    if ptr.is_null() {
        return Ok(None);
    }

    unsafe {
        // SAFETY: deref is safe, because we checked it for null
        Ok(Some((*ptr).lock().unwrap()))
    }
}

/// A port of `JNIEnv::take_rust_field` with type `T` modified to not require `Send`.
pub fn take_field<'local, O, S, T>(
    env: &mut JNIEnv<'local>,
    obj: O,
    field_id: S,
) -> JniResult<Option<T>>
where
    O: AsRef<JObject<'local>>,
    S: Into<JFieldID>,
    T: 'static,
{
    let field_id = field_id.into();

    let _guard = env.lock_obj(&obj)?;
    let mbox = {
        let ptr = env
            .get_field_unchecked(&obj, field_id, ReturnType::Primitive(Primitive::Long))?
            .j()? as *mut ReentrantLock<T>;

        if ptr.is_null() {
            return Ok(None);
        }

        let mbox = unsafe { Box::from_raw(ptr) };

        // attempt to acquire the lock. This prevents us from consuming the
        // mutex if there's an outstanding lock. No one else will be able to
        // get a new one as long as we're in the guarded scope.
        drop(mbox.mutex.lock());

        env.set_field_unchecked(
            obj,
            field_id,
            (::std::ptr::null_mut::<()>() as jlong).into(),
        )?;

        mbox
    };

    Ok(Some(mbox.mutex.into_inner().unwrap()))
}

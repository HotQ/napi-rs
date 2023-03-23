use napi_sys::{napi_env, napi_env__};

use crate::env::Env;
use std::ffi::c_void;
use std::ptr::null_mut;
use std::ptr::NonNull;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use std::sync::RwLock;

pub struct Lifecycle;

type AtomicEnv = AtomicPtr<napi_sys::napi_env__>;

thread_local! {
    static CRT_RAW_ENV0: AtomicEnv = AtomicPtr::new(null_mut());
    static CRT_RAW_ENV: Arc<AtomicEnv> = Arc::new(AtomicPtr::new(null_mut()));
}

impl Lifecycle {
  pub(crate) fn register(env: Env) {
    CRT_RAW_ENV.with(|env_ptr| {
      let raw_env = env.raw();
      env_ptr.store(raw_env, SeqCst);

      unsafe {
        napi_sys::napi_add_env_cleanup_hook(
          raw_env,
          Some(unregister),
          env_ptr as *const _ as *mut c_void,
        );
      }
    });
  }

  pub fn is_valid(env: Env) -> bool {
    CRT_RAW_ENV.with(|env_ptr| std::ptr::eq(env_ptr.load(SeqCst), env.raw()))
  }

  pub fn get_env(env: Env) -> Option<Env> {
    CRT_RAW_ENV.with(|env_ptr| {
      NonNull::new(env_ptr.load(SeqCst)).map(|raw| unsafe { Env::from_raw(raw.as_ptr()) })
    })
  }
}

unsafe extern "C" fn unregister(env_ptr: *mut c_void) {
  unsafe {
    let raw_env = (*env_ptr.cast::<AtomicEnv>()).load(SeqCst);
    napi_sys::napi_remove_env_cleanup_hook(raw_env, Some(unregister), env_ptr);
  }
}

pub struct Bound<T> {
    pub(crate) inner: T,
    pub(crate) raw_env: *mut napi_env,
  }

  pub struct AtomicBound<T> {
    pub(crate) inner: T,
    pub(crate) raw_env: *mut napi_env,
  }

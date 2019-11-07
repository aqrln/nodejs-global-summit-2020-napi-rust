use std::ptr;
use std::slice;

use crate::env::Env;
use crate::result::{Error, Result};
use crate::sys;

use super::{Any, AsObject, String, Value};

#[derive(Debug)]
pub struct ArrayBuffer<'buf> {
    value: sys::napi_value,
    data: &'buf mut [u8],
    env: Env,
}

impl<'buf> ArrayBuffer<'buf> {
    pub fn new(env: Env, len: usize) -> Result<ArrayBuffer<'buf>> {
        let mut value = ptr::null_mut();
        let mut data = ptr::null_mut();

        env.handle_status(unsafe {
            sys::napi_create_arraybuffer(env.as_sys_env(), len, &mut data, &mut value)
        })?;

        Ok(ArrayBuffer {
            value,
            data: unsafe { slice::from_raw_parts_mut(data as *mut u8, len) },
            env,
        })
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<'buf> Value for ArrayBuffer<'buf> {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> Env {
        self.env
    }

    fn from_sys_checked(env: Env, value: sys::napi_value) -> Result<ArrayBuffer<'buf>> {
        if !Any::with_value(env, value).is_arraybuffer()? {
            let message = String::from_str(env, "ArrayBuffer expected")?;
            return Err(Error::type_error(env, &message));
        }

        let mut data = ptr::null_mut();
        let mut len = 0;

        env.handle_status(unsafe {
            sys::napi_get_arraybuffer_info(env.as_sys_env(), value, &mut data, &mut len)
        })?;

        Ok(ArrayBuffer {
            env,
            value,
            data: unsafe { slice::from_raw_parts_mut(data as *mut u8, len) },
        })
    }
}

impl<'buf> AsObject for ArrayBuffer<'buf> {}

impl<'buf> AsRef<[u8]> for ArrayBuffer<'buf> {
    fn as_ref(&self) -> &[u8] {
        self.data
    }
}

impl<'buf> AsMut<[u8]> for ArrayBuffer<'buf> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.data
    }
}

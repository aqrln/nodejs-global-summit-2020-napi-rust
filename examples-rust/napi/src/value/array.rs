use std::ptr;

use crate::env::Env;
use crate::result::{Error, Result};
use crate::sys;

use super::{Any, AsObject, String, Value, ValueInternal};

#[derive(Clone, Copy, Debug)]
pub struct Array {
    value: sys::napi_value,
    env: Env,
}

impl Array {
    pub fn new(env: Env) -> Result<Array> {
        let mut value = ptr::null_mut();
        env.handle_status(unsafe { sys::napi_create_array(env.as_sys_env(), &mut value) })?;

        Ok(Array { value, env })
    }

    pub fn with_len(env: Env, len: usize) -> Result<Array> {
        let mut value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_array_with_length(env.as_sys_env(), len, &mut value)
        })?;

        Ok(Array { value, env })
    }

    pub fn len(&self) -> Result<u32> {
        let mut result = 0;

        self.env.handle_status(unsafe {
            sys::napi_get_array_length(self.env.as_sys_env(), self.as_sys_value(), &mut result)
        })?;

        Ok(result)
    }

    pub fn is_empty(&self) -> Result<bool> {
        self.len().map(|l| l == 0)
    }

    pub fn get(&self, index: u32) -> Result<Any> {
        self.as_napi_object().get_element(index)
    }

    pub fn set<T>(&self, index: u32, value: &T) -> Result<()>
    where
        T: Value,
    {
        self.as_napi_object().set_element(index, value)
    }
}

impl Value for Array {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> Env {
        self.env
    }

    fn from_sys_checked(env: Env, value: sys::napi_value) -> Result<Array> {
        if !Any::with_value(env, value).is_array()? {
            let message = String::from_str(env, "Array expected")?;
            return Err(Error::type_error(env, &message));
        }

        Ok(Array { env, value })
    }
}

impl ValueInternal for Array {
    fn construct(env: Env, value: sys::napi_value) -> Array {
        Array { env, value }
    }
}

impl AsObject for Array {}

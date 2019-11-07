use std::ptr;

use crate::env::Env;
use crate::result::{Error, Result};
use crate::sys;

use super::{Any, String, Value, ValueInternal, ValueType};

#[derive(Clone, Copy, Debug)]
pub struct Number {
    value: sys::napi_value,
    env: Env,
}

impl Number {
    pub fn from_i32(env: Env, value: i32) -> Result<Number> {
        let mut sys_value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_int32(env.as_sys_env(), value, &mut sys_value)
        })?;

        Ok(Number {
            value: sys_value,
            env,
        })
    }

    pub fn from_i64(env: Env, value: i64) -> Result<Number> {
        let mut sys_value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_int64(env.as_sys_env(), value, &mut sys_value)
        })?;

        Ok(Number {
            value: sys_value,
            env,
        })
    }

    pub fn from_f64(env: Env, value: f64) -> Result<Number> {
        let mut sys_value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_double(env.as_sys_env(), value, &mut sys_value)
        })?;

        Ok(Number {
            value: sys_value,
            env,
        })
    }

    pub fn to_i32(&self) -> Result<i32> {
        let mut result = 0;

        self.env.handle_status(unsafe {
            sys::napi_get_value_int32(self.env.as_sys_env(), self.value, &mut result)
        })?;

        Ok(result)
    }

    pub fn to_i64(&self) -> Result<i64> {
        let mut result = 0;

        self.env.handle_status(unsafe {
            sys::napi_get_value_int64(self.env.as_sys_env(), self.value, &mut result)
        })?;

        Ok(result)
    }

    pub fn to_f64(&self) -> Result<f64> {
        let mut result = 0.0;

        self.env.handle_status(unsafe {
            sys::napi_get_value_double(self.env.as_sys_env(), self.value, &mut result)
        })?;

        Ok(result)
    }
}

impl Value for Number {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> Env {
        self.env
    }

    fn from_sys_checked(env: Env, value: sys::napi_value) -> Result<Number> {
        if Any::with_value(env, value).value_type()? != ValueType::Number {
            let message = String::from_str(env, "Number expected")?;
            return Err(Error::type_error(env, &message));
        }

        Ok(Number { env, value })
    }
}

impl ValueInternal for Number {
    fn construct(env: Env, value: sys::napi_value) -> Number {
        Number { env, value }
    }
}

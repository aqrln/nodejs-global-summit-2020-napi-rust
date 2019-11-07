use std::ptr;

use crate::env::Env;
use crate::result::{Error, Result};
use crate::sys;

use super::{Any, String, Value, ValueInternal, ValueType};

#[derive(Clone, Copy, Debug)]
pub struct Boolean {
    value: sys::napi_value,
    env: Env,
}

impl Boolean {
    fn new(env: Env, value: bool) -> Result<Boolean> {
        let mut sys_value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_get_boolean(env.as_sys_env(), value, &mut sys_value)
        })?;

        Ok(Boolean {
            value: sys_value,
            env,
        })
    }

    pub fn truth(env: Env) -> Result<Boolean> {
        Boolean::new(env, true)
    }

    pub fn lie(env: Env) -> Result<Boolean> {
        Boolean::new(env, false)
    }

    pub fn to_bool(&self) -> Result<bool> {
        let mut result = false;

        self.env.handle_status(unsafe {
            sys::napi_get_value_bool(self.env.as_sys_env(), self.value, &mut result)
        })?;

        Ok(result)
    }
}

impl Value for Boolean {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> Env {
        self.env
    }

    fn from_sys_checked(env: Env, value: sys::napi_value) -> Result<Boolean> {
        if Any::with_value(env, value).value_type()? != ValueType::Boolean {
            let message = String::from_str(env, "Boolean expected")?;
            return Err(Error::type_error(env, &message));
        }

        Ok(Boolean { env, value })
    }
}

impl ValueInternal for Boolean {
    fn construct(env: Env, value: sys::napi_value) -> Boolean {
        Boolean { env, value }
    }
}

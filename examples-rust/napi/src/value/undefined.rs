use std::ptr;

use crate::env::Env;
use crate::result::{Error, Result};
use crate::sys;

use super::{Any, String, Value, ValueInternal, ValueType};

#[derive(Clone, Copy, Debug)]
pub struct Undefined {
    value: sys::napi_value,
    env: Env,
}

impl Undefined {
    pub fn new(env: Env) -> Result<Undefined> {
        let mut value = ptr::null_mut();
        env.handle_status(unsafe { sys::napi_get_undefined(env.as_sys_env(), &mut value) })?;

        Ok(Undefined { value, env })
    }
}

impl Value for Undefined {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> Env {
        self.env
    }

    fn from_sys_checked(env: Env, value: sys::napi_value) -> Result<Undefined> {
        if Any::with_value(env, value).value_type()? != ValueType::Undefined {
            let message = String::from_str(env, "Undefined expected")?;
            return Err(Error::type_error(env, &message));
        }

        Ok(Undefined { env, value })
    }
}

impl ValueInternal for Undefined {
    fn construct(env: Env, value: sys::napi_value) -> Undefined {
        Undefined { env, value }
    }
}

use std::ptr;

use crate::env::Env;
use crate::result::{Error, Result};
use crate::sys;

use super::{Any, String, Value, ValueInternal};

#[derive(Clone, Copy, Debug)]
pub struct Null {
    value: sys::napi_value,
    env: Env,
}

impl Null {
    pub fn new(env: Env) -> Result<Null> {
        let mut value = ptr::null_mut();
        env.handle_status(unsafe { sys::napi_get_null(env.as_sys_env(), &mut value) })?;

        Ok(Null { value, env })
    }
}

impl Value for Null {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> Env {
        self.env
    }

    fn from_sys_checked(env: Env, value: sys::napi_value) -> Result<Null> {
        let null = Null::new(env)?;
        if !Any::with_value(env, value).strict_equals(&null)? {
            let message = String::from_str(env, "Null expected")?;
            return Err(Error::type_error(env, &message));
        }

        Ok(Null { env, value })
    }
}

impl ValueInternal for Null {
    fn construct(env: Env, value: sys::napi_value) -> Null {
        Null { env, value }
    }
}

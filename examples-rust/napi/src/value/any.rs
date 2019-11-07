use std::ptr;

use crate::env::Env;
use crate::result::{Error, Result};
use crate::sys;

use super::{
    Array, Boolean, Null, Number, Object, String, Undefined, Value, ValueInternal, ValueType,
};

#[derive(Clone, Copy, Debug)]
pub struct Any {
    value: sys::napi_value,
    env: Env,
}

impl Any {
    pub fn new(env: Env) -> Result<Any> {
        let mut value = ptr::null_mut();
        env.handle_status(unsafe { sys::napi_get_undefined(env.as_sys_env(), &mut value) })?;

        Ok(Any { value, env })
    }

    pub fn with_value(env: Env, value: sys::napi_value) -> Any {
        Any { env, value }
    }

    pub fn as_undefined(&self) -> Result<Undefined> {
        match self.value_type()? {
            ValueType::Undefined => Ok(Undefined::construct(self.env(), self.as_sys_value())),
            _ => Err(Error::type_error(
                self.env(),
                &String::from_str(self.env(), "undefined expected")?,
            )),
        }
    }

    pub fn as_null(&self) -> Result<Null> {
        match self.value_type()? {
            ValueType::Null => Ok(Null::construct(self.env(), self.as_sys_value())),
            _ => Err(Error::type_error(
                self.env(),
                &String::from_str(self.env(), "null expected")?,
            )),
        }
    }

    pub fn as_boolean(&self) -> Result<Boolean> {
        match self.value_type()? {
            ValueType::Boolean => Ok(Boolean::construct(self.env(), self.as_sys_value())),
            _ => Err(Error::type_error(
                self.env(),
                &String::from_str(self.env(), "boolean expected")?,
            )),
        }
    }

    pub fn as_number(&self) -> Result<Number> {
        match self.value_type()? {
            ValueType::Number => Ok(Number::construct(self.env(), self.as_sys_value())),
            _ => Err(Error::type_error(
                self.env(),
                &String::from_str(self.env(), "number expected")?,
            )),
        }
    }

    pub fn as_string(&self) -> Result<String> {
        match self.value_type()? {
            ValueType::String => Ok(String::construct(self.env(), self.as_sys_value())),
            _ => Err(Error::type_error(
                self.env(),
                &String::from_str(self.env(), "string expected")?,
            )),
        }
    }

    pub fn as_object(&self) -> Result<Object> {
        match self.value_type()? {
            ValueType::Object | ValueType::String | ValueType::Function => {
                Ok(Object::construct(self.env(), self.as_sys_value()))
            }
            _ => Err(Error::type_error(
                self.env(),
                &String::from_str(self.env(), "object expected")?,
            )),
        }
    }

    pub fn as_array(&self) -> Result<Array> {
        if self.is_array()? {
            Ok(Array::construct(self.env(), self.as_sys_value()))
        } else {
            Err(Error::type_error(
                self.env(),
                &String::from_str(self.env(), "array expected")?,
            ))
        }
    }
}

impl Value for Any {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> Env {
        self.env
    }

    fn from_sys_checked(env: Env, value: sys::napi_value) -> Result<Any> {
        Ok(Any { env, value })
    }
}

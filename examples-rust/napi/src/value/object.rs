use std::ptr;

use crate::env::Env;
use crate::result::{Error, Result};
use crate::sys;

use super::{Any, Array, String, Value, ValueInternal, ValueType};

#[derive(Clone, Copy, Debug)]
pub struct Object {
    value: sys::napi_value,
    env: Env,
}

impl Object {
    pub fn new(env: Env) -> Result<Object> {
        let mut value = ptr::null_mut();
        env.handle_status(unsafe { sys::napi_create_object(env.as_sys_env(), &mut value) })?;

        Ok(Object { value, env })
    }

    pub fn prototype(&self) -> Result<Any> {
        let mut result = ptr::null_mut();

        self.env.handle_status(unsafe {
            sys::napi_get_prototype(self.env.as_sys_env(), self.value, &mut result)
        })?;

        Ok(Any::with_value(self.env(), result))
    }

    pub fn property_names(&self) -> Result<Array> {
        let mut result = ptr::null_mut();

        self.env.handle_status(unsafe {
            sys::napi_get_property_names(self.env.as_sys_env(), self.value, &mut result)
        })?;

        Ok(Array::construct(self.env, result))
    }

    pub fn set_property<T, U>(&self, key: &T, value: &U) -> Result<()>
    where
        T: Value,
        U: Value,
    {
        self.env.handle_status(unsafe {
            sys::napi_set_property(
                self.env.as_sys_env(),
                self.value,
                key.as_sys_value(),
                value.as_sys_value(),
            )
        })
    }

    pub fn get_property<T>(&self, key: &T) -> Result<Any>
    where
        T: Value,
    {
        let mut result = ptr::null_mut();

        self.env.handle_status(unsafe {
            sys::napi_get_property(
                self.env.as_sys_env(),
                self.value,
                key.as_sys_value(),
                &mut result,
            )
        })?;

        Ok(Any::with_value(self.env, result))
    }

    pub fn has_property<T>(&self, key: &T) -> Result<bool>
    where
        T: Value,
    {
        let mut result = false;

        self.env.handle_status(unsafe {
            sys::napi_has_property(
                self.env.as_sys_env(),
                self.value,
                key.as_sys_value(),
                &mut result,
            )
        })?;

        Ok(result)
    }

    pub fn has_own_property<T>(&self, key: &T) -> Result<bool>
    where
        T: Value,
    {
        let mut result = false;

        self.env.handle_status(unsafe {
            sys::napi_has_own_property(
                self.env.as_sys_env(),
                self.value,
                key.as_sys_value(),
                &mut result,
            )
        })?;

        Ok(result)
    }

    pub fn del_property<T>(&self, key: &T) -> Result<bool>
    where
        T: Value,
    {
        let mut result = false;

        self.env.handle_status(unsafe {
            sys::napi_delete_property(
                self.env.as_sys_env(),
                self.value,
                key.as_sys_value(),
                &mut result,
            )
        })?;

        Ok(result)
    }

    pub fn set_named_property<T>(&self, name: &str, value: &T) -> Result<()>
    where
        T: Value,
    {
        let key = String::from_str(self.env, name)?;
        self.set_property(&key, value)
    }

    pub fn get_named_property(&self, name: &str) -> Result<Any> {
        let key = String::from_str(self.env, name)?;
        self.get_property(&key)
    }

    pub fn has_named_property(&self, name: &str) -> Result<bool> {
        let key = String::from_str(self.env, name)?;
        self.has_property(&key)
    }

    pub fn del_named_property(&self, name: &str) -> Result<bool> {
        let key = String::from_str(self.env, name)?;
        self.del_property(&key)
    }

    pub fn set_element<T>(&self, index: u32, value: &T) -> Result<()>
    where
        T: Value,
    {
        self.env.handle_status(unsafe {
            sys::napi_set_element(
                self.env.as_sys_env(),
                self.value,
                index,
                value.as_sys_value(),
            )
        })
    }

    pub fn get_element(&self, index: u32) -> Result<Any> {
        let mut result = ptr::null_mut();

        self.env.handle_status(unsafe {
            sys::napi_get_element(self.env.as_sys_env(), self.value, index, &mut result)
        })?;

        Ok(Any::with_value(self.env, result))
    }

    pub fn has_element(&self, index: u32) -> Result<bool> {
        let mut result = false;

        self.env.handle_status(unsafe {
            sys::napi_has_element(self.env.as_sys_env(), self.value, index, &mut result)
        })?;

        Ok(result)
    }

    pub fn del_element(&self, index: u32) -> Result<bool> {
        let mut result = false;

        self.env.handle_status(unsafe {
            sys::napi_delete_element(self.env.as_sys_env(), self.value, index, &mut result)
        })?;

        Ok(result)
    }
}

impl Value for Object {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> Env {
        self.env
    }

    fn from_sys_checked(env: Env, value: sys::napi_value) -> Result<Object> {
        if Any::with_value(env, value).value_type()? != ValueType::Object {
            let message = String::from_str(env, "Object expected")?;
            return Err(Error::type_error(env, &message));
        }

        Ok(Object { env, value })
    }
}

impl ValueInternal for Object {
    fn construct(env: Env, value: sys::napi_value) -> Object {
        Object { env, value }
    }
}

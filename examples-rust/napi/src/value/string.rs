use std::ptr;

use crate::env::Env;
use crate::result::{Error, Result};
use crate::sys;

use super::{Any, AsObject, Value, ValueInternal, ValueType};

#[derive(Clone, Copy, Debug)]
pub struct String {
    value: sys::napi_value,
    env: Env,
}

impl String {
    pub fn from_str(env: Env, value: &str) -> Result<String> {
        let mut sys_value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_string_utf8(
                env.as_sys_env(),
                value.as_ptr() as *const i8,
                value.as_bytes().len(),
                &mut sys_value,
            )
        })?;

        Ok(String {
            value: sys_value,
            env,
        })
    }

    pub fn from_latin1(env: Env, value: &[u8]) -> Result<String> {
        let mut sys_value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_string_latin1(
                env.as_sys_env(),
                value.as_ptr() as *const i8,
                value.len(),
                &mut sys_value,
            )
        })?;

        Ok(String {
            value: sys_value,
            env,
        })
    }

    pub fn from_utf16(env: Env, value: &[u16]) -> Result<String> {
        let mut sys_value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_string_utf16(
                env.as_sys_env(),
                value.as_ptr(),
                value.len(),
                &mut sys_value,
            )
        })?;

        Ok(String {
            value: sys_value,
            env,
        })
    }

    fn to_vec<T, U>(
        &self,
        get_value: unsafe extern "C" fn(
            sys::napi_env,
            sys::napi_value,
            *mut U,
            usize,
            *mut usize,
        ) -> sys::napi_status,
    ) -> Result<Vec<T>>
    where
        T: Default + Copy,
        U: Copy,
    {
        let mut bufsize = 0;

        self.env.handle_status(unsafe {
            get_value(
                self.env.as_sys_env(),
                self.value,
                ptr::null_mut(),
                0,
                &mut bufsize,
            )
        })?;

        let mut buffer = vec![T::default(); bufsize + 1];

        self.env.handle_status(unsafe {
            get_value(
                self.env.as_sys_env(),
                self.value,
                buffer.as_mut_ptr() as *mut U,
                bufsize + 1,
                ptr::null_mut(),
            )
        })?;

        buffer.pop();

        Ok(buffer)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        self.to_vec::<_, i8>(sys::napi_get_value_string_utf8)
    }

    pub fn to_latin1(&self) -> Result<Vec<u8>> {
        self.to_vec::<_, i8>(sys::napi_get_value_string_latin1)
    }

    pub fn to_utf16(&self) -> Result<Vec<u16>> {
        self.to_vec::<_, u16>(sys::napi_get_value_string_utf16)
    }

    pub fn to_string(&self) -> Result<std::string::String> {
        let bytes = self.to_bytes()?;
        Ok(unsafe { std::string::String::from_utf8_unchecked(bytes) })
    }
}

impl Value for String {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> Env {
        self.env
    }

    fn from_sys_checked(env: Env, value: sys::napi_value) -> Result<String> {
        if Any::with_value(env, value).value_type()? != ValueType::String {
            let message = String::from_str(env, "String expected")?;
            return Err(Error::type_error(env, &message));
        }

        Ok(String { env, value })
    }
}

impl ValueInternal for String {
    fn construct(env: Env, value: sys::napi_value) -> String {
        String { env, value }
    }
}

impl AsObject for String {}

use std::ptr;

use crate::env::Env;
use crate::result::Result;
use crate::sys;

mod any;
mod array;
mod array_buffer;
mod boolean;
mod buffer;
mod null;
mod number;
mod object;
mod string;
mod typed_array;
mod undefined;

pub use self::any::Any;
pub use self::array::Array;
pub use self::array_buffer::ArrayBuffer;
pub use self::boolean::Boolean;
pub use self::buffer::Buffer;
pub use self::null::Null;
pub use self::number::Number;
pub use self::object::Object;
pub use self::string::String;
pub use self::typed_array::*;
pub use self::undefined::Undefined;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ValueType {
    Undefined,
    Null,
    Boolean,
    Number,
    String,
    Symbol,
    Object,
    Function,
    External,
    BigInt,
}

pub trait Value: Sized {
    fn as_sys_value(&self) -> sys::napi_value;

    fn env(&self) -> Env;

    fn from_sys_checked(env: Env, value: sys::napi_value) -> Result<Self>;

    fn to_napi_boolean(&self) -> Result<Boolean> {
        coerce(self, sys::napi_coerce_to_bool)
    }

    fn to_napi_number(&self) -> Result<Number> {
        coerce(self, sys::napi_coerce_to_number)
    }

    fn to_napi_object(&self) -> Result<Object> {
        coerce(self, sys::napi_coerce_to_object)
    }

    fn to_napi_string(&self) -> Result<String> {
        coerce(self, sys::napi_coerce_to_string)
    }

    fn as_napi_any(&self) -> Any {
        Any::with_value(self.env(), self.as_sys_value())
    }

    fn value_type(&self) -> Result<ValueType> {
        let env = self.env();
        let mut result = sys::napi_valuetype::napi_undefined;

        env.handle_status(unsafe {
            sys::napi_typeof(env.as_sys_env(), self.as_sys_value(), &mut result)
        })?;

        Ok(match result {
            sys::napi_valuetype::napi_undefined => ValueType::Undefined,
            sys::napi_valuetype::napi_null => ValueType::Null,
            sys::napi_valuetype::napi_boolean => ValueType::Boolean,
            sys::napi_valuetype::napi_number => ValueType::Number,
            sys::napi_valuetype::napi_string => ValueType::String,
            sys::napi_valuetype::napi_symbol => ValueType::Symbol,
            sys::napi_valuetype::napi_object => ValueType::Object,
            sys::napi_valuetype::napi_function => ValueType::Function,
            sys::napi_valuetype::napi_external => ValueType::External,
            sys::napi_valuetype::napi_bigint => ValueType::BigInt,
        })
    }

    fn instanceof(&self, constructor: &Object) -> Result<bool> {
        let env = self.env();
        let mut result = false;

        env.handle_status(unsafe {
            sys::napi_instanceof(
                env.as_sys_env(),
                self.as_sys_value(),
                constructor.as_sys_value(),
                &mut result,
            )
        })?;

        Ok(result)
    }

    fn is_array(&self) -> Result<bool> {
        check_type(self, sys::napi_is_array)
    }

    fn is_arraybuffer(&self) -> Result<bool> {
        check_type(self, sys::napi_is_arraybuffer)
    }

    fn is_buffer(&self) -> Result<bool> {
        check_type(self, sys::napi_is_buffer)
    }

    fn is_error(&self) -> Result<bool> {
        check_type(self, sys::napi_is_error)
    }

    fn is_typedarray(&self) -> Result<bool> {
        check_type(self, sys::napi_is_typedarray)
    }

    fn is_dataview(&self) -> Result<bool> {
        check_type(self, sys::napi_is_dataview)
    }

    fn strict_equals<T>(&self, other: &T) -> Result<bool>
    where
        T: Value + ?Sized,
    {
        let env = self.env();
        let mut result = false;

        env.handle_status(unsafe {
            sys::napi_strict_equals(
                env.as_sys_env(),
                self.as_sys_value(),
                other.as_sys_value(),
                &mut result,
            )
        })?;

        Ok(result)
    }
}

pub trait AsObject: Value {
    fn as_napi_object(&self) -> Object {
        Object::construct(self.env(), self.as_sys_value())
    }
}

trait ValueInternal: Value {
    fn construct(env: Env, value: sys::napi_value) -> Self;
}

fn coerce<T, U>(
    value: &T,
    napi_fn: unsafe extern "C" fn(
        sys::napi_env,
        sys::napi_value,
        *mut sys::napi_value,
    ) -> sys::napi_status,
) -> Result<U>
where
    T: Value + ?Sized,
    U: ValueInternal,
{
    let env = value.env();
    let mut coerced_value = ptr::null_mut();

    env.handle_status(unsafe {
        napi_fn(env.as_sys_env(), value.as_sys_value(), &mut coerced_value)
    })?;

    Ok(U::construct(env, coerced_value))
}

fn check_type<T>(
    value: &T,
    napi_fn: unsafe extern "C" fn(sys::napi_env, sys::napi_value, *mut bool) -> sys::napi_status,
) -> Result<bool>
where
    T: Value + ?Sized,
{
    let env = value.env();
    let mut result = false;

    env.handle_status(unsafe { napi_fn(env.as_sys_env(), value.as_sys_value(), &mut result) })?;

    Ok(result)
}

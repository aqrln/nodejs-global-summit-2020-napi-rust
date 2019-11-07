use std::ptr;
use std::slice;

use crate::env::Env;
use crate::result::{Error, Result};
use crate::sys;

use super::{Any, ArrayBuffer, AsObject, String, Value};

pub trait TypedArrayElement {
    type Element;
    fn array_type() -> crate::sys::napi_typedarray_type;
    fn array_type_name() -> &'static str;
}

#[derive(Debug)]
pub struct TypedArray<'buf, T: TypedArrayElement> {
    value: sys::napi_value,
    data: &'buf mut [T::Element],
    env: Env,
    array_buffer: crate::sys::napi_value,
    byte_offset: usize,
}

impl<'buf, T: TypedArrayElement> TypedArray<'buf, T> {
    pub fn from_array_buffer(
        mut array_buffer: ArrayBuffer<'buf>,
        byte_offset: usize,
        count_elements: usize,
    ) -> Result<TypedArray<'buf, T>> {
        let env = array_buffer.env();
        let mut value = ptr::null_mut();

        env.handle_status(unsafe {
            sys::napi_create_typedarray(
                env.as_sys_env(),
                T::array_type(),
                count_elements,
                array_buffer.as_sys_value(),
                byte_offset,
                &mut value,
            )
        })?;

        Ok(TypedArray {
            value,
            data: unsafe {
                slice::from_raw_parts_mut(
                    array_buffer.as_mut().as_ptr() as *mut T::Element,
                    count_elements,
                )
            },
            env,
            byte_offset,
            array_buffer: array_buffer.as_sys_value(),
        })
    }

    pub fn array_type(&self) -> crate::sys::napi_typedarray_type {
        T::array_type()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<'buf, T: TypedArrayElement> Value for TypedArray<'buf, T> {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> Env {
        self.env
    }

    fn from_sys_checked(env: Env, value: sys::napi_value) -> Result<TypedArray<'buf, T>> {
        if !Any::with_value(env, value).is_typedarray()? {
            let message = String::from_str(env, "TypedArray expected")?;
            return Err(Error::type_error(env, &message));
        }

        let mut array_type = crate::sys::napi_typedarray_type::napi_int8_array;
        let mut len = 0;
        let mut data = ptr::null_mut();
        let mut array_buffer = ptr::null_mut();
        let mut byte_offset = 0;

        env.handle_status(unsafe {
            sys::napi_get_typedarray_info(
                env.as_sys_env(),
                value,
                &mut array_type,
                &mut len,
                &mut data,
                &mut array_buffer,
                &mut byte_offset,
            )
        })?;

        if array_type != T::array_type() {
            let message = String::from_str(env, &format!("{} expected", T::array_type_name()))?;
            return Err(Error::type_error(env, &message));
        }

        Ok(TypedArray {
            env,
            value,
            array_buffer,
            byte_offset,
            data: unsafe { slice::from_raw_parts_mut(data as *mut T::Element, len) },
        })
    }
}

impl<'buf, T: TypedArrayElement> AsObject for TypedArray<'buf, T> {}

impl<'buf, T: TypedArrayElement> AsRef<[T::Element]> for TypedArray<'buf, T> {
    fn as_ref(&self) -> &[T::Element] {
        self.data
    }
}

impl<'buf, T: TypedArrayElement> AsMut<[T::Element]> for TypedArray<'buf, T> {
    fn as_mut(&mut self) -> &mut [T::Element] {
        self.data
    }
}

macro_rules! typed_array_instance {
    ($ty:ident, $st:ident, $arr_name:ident, $elem_name:ident) => {
        pub struct $elem_name;

        impl TypedArrayElement for $elem_name {
            type Element = $ty;

            fn array_type() -> crate::sys::napi_typedarray_type {
                crate::sys::napi_typedarray_type::$st
            }

            fn array_type_name() -> &'static str {
                stringify!($arr_name)
            }
        }

        pub type $arr_name<'buf> = TypedArray<'buf, $elem_name>;
    };
}

typed_array_instance!(i8, napi_int8_array, Int8Array, Int8ArrayElement);
typed_array_instance!(u8, napi_uint8_array, UInt8Array, UInt8ArrayElement);
typed_array_instance!(
    u8,
    napi_uint8_clamped_array,
    UInt8ClampedArray,
    UInt8ClampedArrayElement
);
typed_array_instance!(i16, napi_int16_array, Int16Array, Int16ArrayElement);
typed_array_instance!(u16, napi_uint16_array, UInt16Array, UInt16ArrayElement);
typed_array_instance!(i32, napi_int32_array, Int32Array, Int32ArrayElement);
typed_array_instance!(u32, napi_uint32_array, UInt32Array, UInt32ArrayElement);
typed_array_instance!(f32, napi_float32_array, Float32Array, Float32ArrayElement);
typed_array_instance!(f64, napi_float64_array, Float64Array, Float64ArrayElement);
typed_array_instance!(
    i64,
    napi_bigint64_array,
    BigInt64Array,
    BigInt64ArrayElement
);
typed_array_instance!(
    u64,
    napi_biguint64_array,
    BigUInt64Array,
    BigUInt64ArrayElement
);

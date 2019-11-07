use std::error::Error as StdError;
use std::fmt;
use std::fmt::Display;
use std::ptr;

use crate::env::Env;
use crate::sys::{
    napi_create_error, napi_create_range_error, napi_create_type_error, napi_status, napi_value,
};
use crate::value::{String, Value};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidArg,
    ObjectExpected,
    StringExpected,
    NameExpected,
    FunctionExpected,
    NumberExpected,
    BooleanExpected,
    ArrayExpected,
    GenericFailure,
    PendingException,
    Cancelled,
    EscapeCalledTwice,
    ApplicationError,
}

#[derive(Clone, Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: Option<std::string::String>,
    pub exception: Option<napi_value>,
}

pub type Result<T> = std::result::Result<T, Error>;

impl ErrorKind {
    pub fn from_napi_status(status: napi_status) -> Self {
        match status {
            napi_status::napi_invalid_arg => ErrorKind::InvalidArg,
            napi_status::napi_object_expected => ErrorKind::ObjectExpected,
            napi_status::napi_string_expected => ErrorKind::StringExpected,
            napi_status::napi_name_expected => ErrorKind::NameExpected,
            napi_status::napi_function_expected => ErrorKind::FunctionExpected,
            napi_status::napi_number_expected => ErrorKind::NumberExpected,
            napi_status::napi_boolean_expected => ErrorKind::BooleanExpected,
            napi_status::napi_array_expected => ErrorKind::ArrayExpected,
            napi_status::napi_generic_failure => ErrorKind::GenericFailure,
            napi_status::napi_pending_exception => ErrorKind::PendingException,
            napi_status::napi_cancelled => ErrorKind::Cancelled,
            napi_status::napi_escape_called_twice => ErrorKind::EscapeCalledTwice,
            _ => {
                // Both situations should never happen, so just panic.
                panic!(
                    "Either the JavaScript VM returned an unknown status code, \
                     or NapiErrorKind::from_napi_status was called with \
                     napi_status::napi_ok"
                );
            }
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::InvalidArg => "NapiError: invalid argument",
            ErrorKind::ObjectExpected => "NapiError: object expected",
            ErrorKind::StringExpected => "NapiError: string expected",
            ErrorKind::NameExpected => "NapiError: name expected",
            ErrorKind::FunctionExpected => "NapiError: function expected",
            ErrorKind::NumberExpected => "NapiError: number expected",
            ErrorKind::BooleanExpected => "NapiError: boolean argument",
            ErrorKind::ArrayExpected => "NapiError: array expected",
            ErrorKind::GenericFailure => "NapiError: generic failure",
            ErrorKind::PendingException => "NapiError: pending exception",
            ErrorKind::Cancelled => "NapiError: cancelled",
            ErrorKind::EscapeCalledTwice => "NapiError: escape called twice",
            ErrorKind::ApplicationError => "NapiError: application error",
        }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.description())
            .and_then(|result| {
                if let Some(ref message) = self.message {
                    write!(formatter, " ({})", message)
                } else {
                    Ok(result)
                }
            })
            .and_then(|result| {
                if self.exception.is_some() {
                    write!(formatter, ", JavaScript exception attached")
                } else {
                    Ok(result)
                }
            })
    }
}

macro_rules! error_constructor {
    ($name:ident => $napi_fn_name:ident) => {
        pub fn $name(env: Env, message: &String) -> Error {
            let mut exception = ptr::null_mut();
            let status = unsafe {
                $napi_fn_name(
                    env.as_sys_env(),
                    ptr::null_mut(),
                    message.as_sys_value(),
                    &mut exception,
                )
            };

            if let Err(error) = env.handle_status(status) {
                return error;
            }

            Error {
                kind: ErrorKind::ApplicationError,
                message: None,
                exception: Some(exception),
            }
        }
    };
}

impl Error {
    error_constructor!(error => napi_create_error);
    error_constructor!(type_error => napi_create_type_error);
    error_constructor!(range_error => napi_create_range_error);
}

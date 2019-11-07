#![recursion_limit = "128"]

extern crate proc_macro;

use quote::quote;

#[proc_macro_attribute]
pub fn callback(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let c_name = syn::parse_macro_input!(attr as proc_macro2::Ident);
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let callback = make_callback(c_name, input.clone().sig);

    let output = quote! {
        #input
        #callback
    };

    proc_macro::TokenStream::from(output)
}

fn make_callback(c_name: proc_macro2::Ident, sig: syn::Signature) -> proc_macro2::TokenStream {
    let rs_name = sig.ident.clone();
    let (get_args, pass_args) = make_args(sig.inputs);
    let error = return_error();

    quote! {
        #[no_mangle]
        pub extern "C" fn #c_name(
            env: napi::sys::napi_env,
            cb_info: napi::sys::napi_callback_info,
        ) -> napi::sys::napi_value {
            use std::error::Error;
            use napi::Value;

            let env_wrapper = napi::Env::from(env);

            #get_args

            fn typecheck_result<T: napi::Value>(_: &napi::Result<T>) {}
            let result = #rs_name(env_wrapper #pass_args);
            typecheck_result(&result);

            match result {
                Ok(value) => value.as_sys_value(),
                Err(error) => {
                    #error
                }
            }
        }
    }
}

fn make_args(
    args: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let count = args.len() - 1;

    if count == 0 {
        return (quote! {}, quote! {});
    }

    let args = args
        .iter()
        .skip(1)
        .map(|arg| {
            let (ident, ty) = match arg {
                syn::FnArg::Typed(typed) => {
                    let ident = match *typed.pat {
                        syn::Pat::Ident(ref pat_ident) => pat_ident.ident.clone(),
                        _ => panic!("incorrect argument"),
                    };
                    (ident, typed.ty.clone())
                }
                syn::FnArg::Receiver(_) => panic!("incorrect argument"),
            };

            let new_name = Box::new(format!("fn_arg_{}", ident));
            let new_ident = syn::Ident::new(&new_name, ident.span());

            (new_ident, ty)
        })
        .collect::<Vec<_>>();

    let error = return_error();

    let exprs = args.iter().enumerate().map(|(index, (ident, ty))| {
        quote! {
            let #ident = match <#ty as napi::Value>::from_sys_checked(env_wrapper, argv[#index]) {
                Ok(value) => value,
                Err(error) => {
                    #error
                }
            };
        }
    });

    let get_args = quote! {
        let mut argc = #count;
        let mut argv = [std::ptr::null_mut(); #count];

        let status = env_wrapper.handle_status(unsafe {
            napi::sys::napi_get_cb_info(
                env,
                cb_info,
                &mut argc,
                argv.as_mut_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        });

        if let Err(error) = status {
            #error
        }

        if argc != #count {
            let message = match napi::String::from_str(env_wrapper, &format!(
                "Expected {} arguments, but got {}",
                #count,
                argc,
            )) {
                Ok(msg) => msg,
                Err(error) => {
                    #error
                }
            };
            let error = napi::Error::type_error(env_wrapper, &message);
            #error
        }

        #(#exprs);*
    };

    let arg_names = args.iter().map(|(ident, _)| ident);
    let pass_args = quote! {
        , #(#arg_names),*
    };

    (get_args, pass_args)
}

fn return_error() -> proc_macro2::TokenStream {
    quote! {
        if let Some(exception) = error.exception {
            unsafe {
                napi::sys::napi_throw(env, exception);
            }
        } else {
            let message = format!("{}", &error);
            let c_string = std::ffi::CString::new(message)
                .unwrap_or_else(|_| std::ffi::CString::new(error.description()).unwrap());

            unsafe {
                napi::sys::napi_throw_error(env, std::ptr::null(), c_string.as_ptr());
            }
        }

        let mut result: napi::sys::napi_value = std::ptr::null_mut();
        unsafe {
            napi::sys::napi_get_undefined(env, &mut result);
        }
        return result;
    }
}

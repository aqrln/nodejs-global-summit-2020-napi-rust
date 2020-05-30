#include <node_api.h>

// From https://nodejs.org/dist/latest-v14.x/docs/api/n-api.html
#define NAPI_CALL(env, call)                                      \
  do {                                                            \
    napi_status status = (call);                                  \
    if (status != napi_ok) {                                      \
      const napi_extended_error_info* error_info = NULL;          \
      napi_get_last_error_info((env), &error_info);               \
      bool is_pending;                                            \
      napi_is_exception_pending((env), &is_pending);              \
      if (!is_pending) {                                          \
        const char* message = (error_info->error_message == NULL) \
            ? "empty error message"                               \
            : error_info->error_message;                          \
        napi_throw_error((env), NULL, message);                   \
        return NULL;                                              \
      }                                                           \
    }                                                             \
  } while(0)

napi_value add_numbers(napi_env env, napi_callback_info info) {
  size_t argc = 2;
  napi_value argv[2];
  NAPI_CALL(env, napi_get_cb_info(env, info, &argc, &argv[0], NULL, NULL));

  int a;
  NAPI_CALL(env, napi_get_value_int32(env, argv[0], &a));

  int b;
  NAPI_CALL(env, napi_get_value_int32(env, argv[0], &b));

  napi_value result;
  NAPI_CALL(env, napi_create_int32(env, a + b, &result));
  return result;
}

NAPI_MODULE_INIT() {
  napi_value result;
  NAPI_CALL(env, napi_create_object(env, &result));

  napi_value add_numbers_fn;
  NAPI_CALL(env, napi_create_function(env,
      "addNumbers",
      NAPI_AUTO_LENGTH,
      add_numbers,
      NULL,
      &add_numbers_fn));

  NAPI_CALL(env, napi_set_named_property(env,
    result,
    "addNumbers",
    add_numbers_fn));

  return result;
}

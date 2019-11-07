use rayon::prelude::*;

#[napi::callback(sum_of_squares_par_js)]
fn sum_of_squares_par(env: napi::Env, array: napi::Float64Array<'_>) -> napi::Result<napi::Number> {
    let sum = array.as_ref().par_iter().map(|&i| i * i).sum();
    napi::Number::from_f64(env, sum)
}

#[napi::callback(sum_of_squares_seq_js)]
fn sum_of_squares_seq(env: napi::Env, array: napi::Float64Array<'_>) -> napi::Result<napi::Number> {
    let sum = array.as_ref().iter().map(|&i| i * i).sum();
    napi::Number::from_f64(env, sum)
}

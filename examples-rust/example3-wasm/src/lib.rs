use js_sys as js;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sum_of_squares(array: js::Float64Array) -> f64 {
    array.to_vec().iter().map(|&i| i * i).sum()
}

#[wasm_bindgen]
pub fn sum_of_squares2(array: js::Float64Array) -> f64 {
    let mut result = 0.0;
    for i in 0..array.length() {
        let x = array.get_index(i);
        result += x * x;
    }
    result
}

#[napi::callback(example_hello)]
fn hello(env: napi::Env) -> napi::Result<napi::Undefined> {
    println!("Hello from the Rust land!");
    napi::Undefined::new(env)
}

#[napi::callback(example_add)]
fn add(env: napi::Env, first: napi::Number, second: napi::Number) -> napi::Result<napi::Number> {
    let first = first.to_i32()?;
    let second = second.to_i32()?;
    napi::Number::from_i32(env, first + second)
}

use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
extern crate console_error_panic_hook;
use std::panic;
mod one;
mod two;
mod three;
mod util;

#[wasm_bindgen]
pub fn hello() {
    log(&"Hello from the other side!".into());

    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn one_a(input: &str) -> String {
    return one::a(input);
}

#[wasm_bindgen]
pub fn one_b(input: &str) -> String {
    return one::b(input);
}

#[wasm_bindgen]
pub fn two_a(input: &str) -> String {
    return two::a(input);
}

#[wasm_bindgen]
pub fn two_b(input: &str) -> String {
    return two::b(input);
}

#[wasm_bindgen]
pub fn three_a(input: &str) -> String {
    return three::a(input);
}

#[wasm_bindgen]
pub fn three_b(input: &str) -> String {
    return three::b(input);
}

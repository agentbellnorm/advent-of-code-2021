use wasm_bindgen::prelude::wasm_bindgen;
extern crate console_error_panic_hook;
use crate::util::log_debug_js;
use std::panic;

mod eight;
mod five;
mod four;
mod nine;
mod one;
mod seven;
mod six;
mod ten;
mod three;
mod two;
mod util;

#[wasm_bindgen]
pub fn hello() {
    log_debug_js(&"Hello from the other side!");

    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn one_a(input: &str) -> String {
    one::a(input)
}
#[wasm_bindgen]
pub fn one_b(input: &str) -> String {
    one::b(input)
}

#[wasm_bindgen]
pub fn two_a(input: &str) -> String {
    two::a(input)
}
#[wasm_bindgen]
pub fn two_b(input: &str) -> String {
    two::b(input)
}

#[wasm_bindgen]
pub fn three_a(input: &str) -> String {
    three::a(input)
}
#[wasm_bindgen]
pub fn three_b(input: &str) -> String {
    three::b(input)
}

#[wasm_bindgen]
pub fn four_a(input: &str) -> String {
    four::a(input)
}
#[wasm_bindgen]
pub fn four_b(input: &str) -> String {
    four::b(input)
}

#[wasm_bindgen]
pub fn five_a(input: &str) -> String {
    five::a(input)
}
#[wasm_bindgen]
pub fn five_b(input: &str) -> String {
    five::b(input)
}

#[wasm_bindgen]
pub fn six_a(input: &str) -> String {
    six::a(input)
}
#[wasm_bindgen]
pub fn six_b(input: &str) -> String {
    six::b(input)
}

#[wasm_bindgen]
pub fn seven_a(input: &str) -> String {
    seven::a(input)
}
#[wasm_bindgen]
pub fn seven_b(input: &str) -> String {
    seven::b(input)
}

#[wasm_bindgen]
pub fn eight_a(input: &str) -> String {
    eight::a(input)
}
#[wasm_bindgen]
pub fn eight_b(input: &str) -> String {
    eight::b(input)
}

#[wasm_bindgen]
pub fn nine_a(input: &str) -> String {
    nine::a(input)
}
#[wasm_bindgen]
pub fn nine_b(input: &str) -> String {
    nine::b(input)
}

#[wasm_bindgen]
pub fn ten_a(input: &str) -> String {
    ten::a(input)
}
#[wasm_bindgen]
pub fn ten_b(input: &str) -> String {
    ten::b(input)
}

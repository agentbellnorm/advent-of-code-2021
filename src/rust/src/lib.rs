use wasm_bindgen::prelude::wasm_bindgen;
extern crate console_error_panic_hook;
use crate::util::log_debug_js;
use std::panic;

mod eight;
mod five;
mod four;
mod one;
mod seven;
mod six;
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

#[wasm_bindgen]
pub fn four_a(input: &str) -> String {
    return four::a(input);
}
#[wasm_bindgen]
pub fn four_b(input: &str) -> String {
    return four::b(input);
}

#[wasm_bindgen]
pub fn five_a(input: &str) -> String {
    return five::a(input);
}
#[wasm_bindgen]
pub fn five_b(input: &str) -> String {
    return five::b(input);
}

#[wasm_bindgen]
pub fn six_a(input: &str) -> String {
    return six::a(input);
}
#[wasm_bindgen]
pub fn six_b(input: &str) -> String {
    return six::b(input);
}

#[wasm_bindgen]
pub fn seven_a(input: &str) -> String {
    return seven::a(input);
}
#[wasm_bindgen]
pub fn seven_b(input: &str) -> String {
    return seven::b(input);
}
#[wasm_bindgen]
pub fn eight_a(input: &str) -> String {
    return eight::a(input);
}
#[wasm_bindgen]
pub fn eight_b(input: &str) -> String {
    return eight::b(input);
}

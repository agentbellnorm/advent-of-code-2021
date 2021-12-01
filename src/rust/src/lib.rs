use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
extern crate console_error_panic_hook;
use std::panic;

#[wasm_bindgen]
pub fn hello(name: &str) -> String {
    log(&"from rust!".into());
    return format!("Hello {}", &name);
}

#[wasm_bindgen]
pub fn one_a(input: &str) -> String {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let list: Vec<&str> = input.split("\n").collect();

    let mut increased = 0;
    for (i, item) in list.iter().enumerate() {
        let prev_index = (i as i32) - 1;
        let current_num: i32 = item.parse().unwrap();
        let mut prev_num: i32 = i32::MAX;
        if prev_index >= 0 && list.get(prev_index as usize).is_some() {
            prev_num = list.get(i - 1).unwrap().parse().unwrap();
        }

        if current_num > prev_num {
            increased = increased + 1;
        }
    }

    return format!("{:?}", increased);
}

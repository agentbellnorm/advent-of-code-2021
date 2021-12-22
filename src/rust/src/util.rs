use std::fmt::Debug;
use web_sys::console::log_1 as log;

pub fn split_lines(s: &str) -> Vec<&str> {
    s.split("\n").collect()
}

pub fn char_at(str: &str, i: i32) -> char {
    str.chars().nth(i as usize).unwrap()
}

pub fn log_debug_js(debugable: &impl Debug) {
    log(&format!("{:?}", debugable).into());
}

pub fn int(string: &str) -> i32 {
    match string.parse::<i32>() {
        Ok(number) => number,
        Err(_) => panic!("Could not parse {:?} to i32", string),
    }
}

pub fn int_big(string: &str) -> i64 {
    match string.parse::<i64>() {
        Ok(number) => number,
        Err(_) => panic!("Could not parse {:?} to i64", string),
    }
}

pub fn get_index((x, y): (i32, i32), n_cols: i32) -> i32 {
    (y * n_cols) + x
}

pub fn print_ret<T: Debug>(v: T) -> T {
    log_debug_js(&v);
    v
}

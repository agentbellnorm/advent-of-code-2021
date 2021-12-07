
pub fn split_lines(s: &str) -> Vec<&str> {
  return s.split("\n").collect();
}

pub fn char_at(str: &str, i: i32) -> char {
  return str.chars().nth(i as usize).unwrap();
}
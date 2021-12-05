pub fn a(input: &str) -> String {
  let list = crate::util::split_lines(input);

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

  return format!("{:?}", increased)
}

pub fn b(input: &str) -> String {
  let v = crate::util::split_lines(input);

  let mut increased = 0;
  let mut prev_window_sum = i32::MAX;

  for i in 2..v.len() {
    let one: i32 = v.get(i - 2).unwrap().parse().unwrap();
    let two: i32 = v.get(i - 1).unwrap().parse().unwrap();
    let three: i32 = v.get(i).unwrap().parse().unwrap();

    let this_window_sum = one + two + three;

    if this_window_sum > prev_window_sum {
      increased += 1;
    }

    prev_window_sum = this_window_sum;
  }

  return format!("{:?}", increased);
}
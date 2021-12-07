pub fn a(input: &str) -> String {
  let report = crate::util::split_lines(input);
  let length = report.get(0).unwrap().len();

  let mut gamma = "".to_string();
  let mut epsilon = "".to_string();

  for col in 0..length {
    let mut zeros = 0;
    let mut ones = 0;
    for row in &report {
        let bit = row.chars().nth(col).unwrap();
        match bit {
          '1' => ones += 1,
          '0' => zeros += 1,
          _ => panic!("not binary!")
        }
    }

    if ones > zeros {
      gamma.push_str("1");
      epsilon.push_str("0");
    } else if zeros > ones {
      gamma.push_str("0");
      epsilon.push_str("1");
    }
  }

  let int_gamma = isize::from_str_radix(&gamma, 2).unwrap();
  let int_epsilon = isize::from_str_radix(&epsilon, 2).unwrap();

  return format!("{:?}", int_epsilon * int_gamma);
}

pub fn b(input: &str) -> String {
  return format!("{:?}", "Not implemented");
}
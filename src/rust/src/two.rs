fn get_instruction_tuple(instruction: &str) -> (&str, i32) {
  let v: Vec<&str> = instruction.split(" ").collect();
  
  return (v.get(0).unwrap(), v.get(1).unwrap().parse().unwrap());
}

pub fn a(input: &str) -> String {
  let all_instructions = crate::util::split_lines(input);

  let mut horizontal = 0;
  let mut depth = 0;

  for instruction in all_instructions {
    let instruction_tuple = get_instruction_tuple(instruction);
    match instruction_tuple {
      ("forward", value) => horizontal += value,
      ("down", value) => depth += value,
      ("up", value) => depth -= value,
      (_, _) => panic!("unknown values!")
    }
  }

  return format!("{}", horizontal * depth);
}


pub fn b(input: &str) -> String {
  let all_instructions = crate::util::split_lines(input);

  let mut horizontal = 0;
  let mut depth = 0;
  let mut aim = 0;

  for instruction in all_instructions {
    let instruction_tuple = get_instruction_tuple(instruction);
    match instruction_tuple {
      ("forward", value) => {
        horizontal += value;
        depth += aim * value;
      },
      ("down", value) => {
        aim += value;
      },
      ("up", value) => {
        aim -= value;
      },
      (_, _) => panic!("unknown values!")
    }
  }

  return format!("{}", horizontal * depth);
}
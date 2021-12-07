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
    let report = crate::util::split_lines(input);

    let ogr = oxygen_generator(report.clone());
    let osr = c02_scrubber(report);

    let int_ogr = isize::from_str_radix(&ogr, 2).unwrap();
    let int_osr = isize::from_str_radix(&osr, 2).unwrap();
    return format!("{:?}", int_ogr * int_osr);
}

fn oxygen_generator(rows: Vec<&str>) -> &str {
    return x_generator(rows, 0, |x, y| x >= y);
}

fn c02_scrubber(rows: Vec<&str>) -> &str {
    return x_generator(rows, 0, |x, y| x < y);
}

fn x_generator(rows: Vec<&str>, index: i32, comparator: fn(i32, i32) -> bool) -> &str {
    if rows.len() == 1 {
        return rows.get(0).unwrap();
    }

    let x_common = x_common(&rows, index, comparator);
    let filtered = rows
        .into_iter()
        .filter(|row| crate::util::char_at(row, index).eq(&x_common))
        .clone()
        .collect();

    return x_generator(filtered, index + 1, comparator);
}

fn x_common(rows: &Vec<&str>, column: i32, comparator: fn(i32, i32) -> bool) -> char {
    let mut zeros = 0;
    let mut ones = 0;
    for row in rows {
        let bit = crate::util::char_at(row, column);
        match bit {
            '1' => ones += 1,
            '0' => zeros += 1,
            _ => panic!("not binary!")
        }
    }

    if comparator(ones, zeros) {
        return '1';
    } else {
        return '0';
    }
}

#[test]
fn life_supprt_rating() {
    assert_eq!(b("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"), "230");
}

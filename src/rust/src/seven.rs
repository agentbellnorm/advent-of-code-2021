use crate::util::int;

pub fn a(input: &str) -> String {
    let positions: Vec<i32> = input.split(",").map(int).collect();

    let mut minimum_fuel: i32 = i32::MAX;

    for position in &positions {
        let mut fuel: i32 = 0;
        for other_positions in &positions {
            fuel += (position - other_positions).abs();
        }

        if fuel < minimum_fuel {
            minimum_fuel = fuel;
        }
    }

    format!("{:?}", minimum_fuel)
}

#[test]
fn fuel() {
    assert_eq!(calculate_fuel(16, 5), 66);
    assert_eq!(calculate_fuel(1, 5), 10);
    assert_eq!(calculate_fuel(4, 5), 1);
}

fn calculate_fuel(a: i32, b: i32) -> i32 {
    let n = (a - b).abs();
    (n * (n + 1)) / 2
}

pub fn b(input: &str) -> String {
    let positions: Vec<i32> = input.split(",").map(int).collect();
    let max_position: i32 = positions.iter().cloned().max().unwrap();

    let mut minimum_fuel: i32 = i32::MAX;

    for position in 0..max_position {
        let mut fuel: i32 = 0;
        for other_position in &positions {
            fuel += calculate_fuel(position.clone(), other_position.clone())
        }
        if fuel < minimum_fuel {
            minimum_fuel = fuel;
        }
    }

    format!("{:?}", minimum_fuel)
}

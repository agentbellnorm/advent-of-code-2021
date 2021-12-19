use crate::util::int_big;
use std::collections::HashMap;

fn simulate_fish(input: &str, days: i32) -> i64 {
    let mut fishies: HashMap<i64, i64> = HashMap::from([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
    ]);

    for fish in input.split(",").map(int_big).collect::<Vec<i64>>() {
        if fishies.contains_key(&fish) {
            fishies.insert(fish.clone(), fishies.get(&fish).unwrap() + 1);
        } else {
            fishies.insert(fish.clone(), 1);
        }
    }

    for _ in 0..days {
        let zeros = fishies.get(&0).unwrap().clone();

        let mut tmp: i64 = fishies.insert(8, zeros).unwrap();

        tmp = fishies.insert(7, tmp.clone()).unwrap();
        tmp = fishies.insert(6, tmp.clone() + zeros).unwrap();
        tmp = fishies.insert(5, tmp.clone()).unwrap();
        tmp = fishies.insert(4, tmp.clone()).unwrap();
        tmp = fishies.insert(3, tmp.clone()).unwrap();
        tmp = fishies.insert(2, tmp.clone()).unwrap();
        tmp = fishies.insert(1, tmp.clone()).unwrap();
        fishies.insert(0, tmp.clone()).unwrap();
    }

    fishies.values().into_iter().sum::<i64>()
}

pub fn a(input: &str) -> String {
    format!("{:?}", simulate_fish(input, 80))
}

pub fn b(input: &str) -> String {
    format!("{:?}", simulate_fish(input, 256))
}

#[test]
fn test() {
    assert_eq!(simulate_fish("3,4,3,1,2", 80), 5934);
    assert_eq!(simulate_fish("3,4,3,1,2", 256), 26984457539);
}

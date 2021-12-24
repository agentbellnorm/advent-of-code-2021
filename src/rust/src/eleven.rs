use crate::util::{count_in_vec, get_index, int};
use std::collections::HashSet;

const WIDTH: i32 = 10;

type Coords = (i32, i32);

fn parse(input: &str) -> Vec<i32> {
    input
        .replace("\n", "")
        .chars()
        .map(|c| int(&c.to_string()))
        .collect::<Vec<i32>>()
}

fn get_coords(index: i32) -> Coords {
    let x = index % WIDTH;
    (x, (index - x) / WIDTH)
}

fn maybe_index((x, y): Coords) -> Option<i32> {
    match x >= 0 && x < WIDTH && y >= 0 && y < WIDTH {
        true => Some(get_index((x, y), WIDTH)),
        false => None,
    }
}

fn get_adjacent_indices((x, y): Coords) -> HashSet<i32> {
    vec![
        (x, y - 1),     // N
        (x + 1, y - 1), // NE
        (x + 1, y),     // E
        (x + 1, y + 1), // SE
        (x, y + 1),     // S
        (x - 1, y + 1), // SW
        (x - 1, y),     // W
        (x - 1, y - 1), // NW
    ]
    .into_iter()
    .filter_map(maybe_index)
    .collect::<HashSet<i32>>()
}

fn stringify(v: Vec<i32>) -> Vec<Vec<String>> {
    v.chunks(10)
        .map(|row| {
            row.into_iter()
                .map(|n| match n {
                    0 => format!("[{}]", n),
                    v => format!("{}", v),
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
}

#[test]
fn test_step() {
    let lines = parse(
        "6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637",
    );
    for _line in stringify(do_step(lines)) {
        // println!("{:?}", &line);
    }
}

fn get_flash_indices(dumbos: &Vec<i32>) -> HashSet<i32> {
    dumbos
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| match *v > 9 {
            true => Some(i as i32),
            _ => None,
        })
        .collect::<HashSet<i32>>()
}

fn inc(mut v: Vec<i32>, i: i32) -> Vec<i32> {
    v[i as usize] += 1;
    v
}

fn reset(mut v: Vec<i32>, i: i32) -> Vec<i32> {
    v[i as usize] = 0;
    v
}

fn do_step(mut dumbos: Vec<i32>) -> Vec<i32> {
    for i in 0..dumbos.len() {
        dumbos = inc(dumbos, i as i32);
    }

    let mut flashed_this_step: HashSet<i32> = HashSet::new();

    loop {
        let to_flash = &get_flash_indices(&dumbos) - &flashed_this_step;

        if to_flash.is_empty() {
            break;
        }

        flashed_this_step.extend(&to_flash);

        for flash_index in &to_flash {
            for adjacent_index in
                &get_adjacent_indices(get_coords(*flash_index as i32)) - &flashed_this_step
            {
                dumbos = inc(dumbos, adjacent_index)
            }
        }
    }

    for flashed in &flashed_this_step {
        dumbos = reset(dumbos, *flashed);
    }

    dumbos
}

pub fn a(input: &str) -> String {
    let mut dumbos = parse(input);

    let mut flashes = 0;
    for _ in 0..100 {
        dumbos = do_step(dumbos);
        flashes += count_in_vec(&dumbos, 0)
    }

    format!("{:?}", flashes)
}

fn all_equal(vec: &Vec<i32>, item: i32) -> bool {
    vec.into_iter().all(|dumbo| *dumbo == item)
}

pub fn b(input: &str) -> String {
    let mut dumbos = parse(input);
    let mut simultaneous_step = 0;

    for i in 0..(i32::MAX) {
        dumbos = do_step(dumbos);

        if all_equal(&dumbos, 0) {
            simultaneous_step = i + 1;
            break;
        }
    }

    format!("{:?}", simultaneous_step)
}

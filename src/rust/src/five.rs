use crate::log_debug_js;
use crate::util::int;
use crate::util::split_lines;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;

type PointPair = (i32, i32, i32, i32);

#[test]
fn parse_line_t() {
    assert_eq!(parse_line("0,9 -> 5,9"), (0, 9, 5, 9));
    assert_eq!(parse_line("01,99 -> 5,0"), (1, 99, 5, 0));
}

#[test]
fn mark_lines_t() {
    assert_eq!(
        mark_lines(vec![0; 9], (2, 2, 2, 1), 3),
        [0, 0, 0, 0, 0, 1, 0, 0, 1]
    )
}

fn parse_line(line: &str) -> PointPair {
    // Make one static reference instead of building new pattern every call
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+),(\d+)\s->\s(\d+),(\d+)").unwrap();
    }
    let cap = RE.captures_iter(line).next().unwrap();
    return (int(&cap[1]), int(&cap[2]), int(&cap[3]), int(&cap[4]));
}

fn world_size(all_points: &Vec<PointPair>) -> (i32, i32) {
    let (x_max, y_max) =
        all_points
            .into_iter()
            .fold((0, 0), |(curr_max_x, curr_max_y), (x1, y1, x2, y2)| {
                return (
                    max(max(curr_max_x, x2.to_owned()), x1.to_owned()),
                    max(max(curr_max_y, y2.to_owned()), y1.to_owned()),
                );
            });
    (x_max + 1, y_max + 1)
}

#[test]
fn get_index_t() {
    assert_eq!(get_index((1, 2), 3), 7);
    assert_eq!(get_index((3, 4), 4), 19);
    assert_eq!(get_index((2, 0), 3), 2);
    assert_eq!(get_index((9, 9), 10), 99);
}

fn get_index((x, y): (i32, i32), n_cols: i32) -> i32 {
    (y * n_cols) + x
}

#[test]
fn get_line_t() {
    assert_eq!(get_line((1, 1, 1, 3)), vec![(1, 1), (1, 2), (1, 3)]);
    assert_eq!(get_line((9, 7, 7, 7)), vec![(7, 7), (8, 7), (9, 7)]);
    assert_eq!(get_line((1, 1, 3, 3)), vec![(1, 1), (2, 2), (3, 3)]);
    assert_eq!(get_line((9, 7, 7, 9)), vec![(9, 7), (8, 8), (7, 9)]);
}

fn get_line((x1, y1, x2, y2): PointPair) -> Vec<(i32, i32)> {
    if x1 == x2 && y2 > y1 {
        // S
        return (y1..(y2 + 1)).map(|y| (x1, y)).collect();
    } else if x1 == x2 && y1 > y2 {
        // N
        return (y2..(y1 + 1)).map(|y| (x1, y)).collect();
    } else if y1 == y2 && x2 > x1 {
        // E
        return (x1..(x2 + 1)).map(|x| (x, y1)).collect();
    } else if y1 == y2 && x1 > x2 {
        // W
        return (x2..(x1 + 1)).map(|x| (x, y1)).collect();
    } else if x2 > x1 && y2 < y1 {
        // NE
        return (1..(x2 - x1 + 1)).fold(vec![(x1, y1)], |mut acc, v| {
            acc.push((x1 + v, y1 - v));
            acc
        });
    } else if x2 > x1 && y2 > y1 {
        // SE
        return (1..(x2 - x1 + 1)).fold(vec![(x1, y1)], |mut acc, v| {
            acc.push((x1 + v, y1 + v));
            acc
        });
    } else if x2 < x1 && y2 > y1 {
        // SW
        return (1..(y2 - y1 + 1)).fold(vec![(x1, y1)], |mut acc, v| {
            acc.push((x1 - v, y1 + v));
            acc
        });
    } else if x2 < x1 && y2 < y1 {
        // NW
        return (1..(x1 - x2 + 1)).fold(vec![(x1, y1)], |mut acc, v| {
            acc.push((x1 - v, y1 - v));
            acc
        });
    }

    panic!("wtf! {:?}", (x1, y1, x2, y2));
}

fn mark_lines(mut world: Vec<i32>, points: PointPair, x_max: i32) -> Vec<i32> {
    for point in get_line(points) {
        let index = get_index(point, x_max);
        world[index as usize] = world[index as usize] + 1;
    }

    return world;
}

fn _print_world(world: &Vec<i32>, cols: i32) {
    log_debug_js(&"world:");
    for row in world.chunks(cols as usize) {
        log_debug_js(&row);
    }
    log_debug_js(&"");
}

fn run(input: &str, include_diagonal: bool) -> usize {
    let point_pairs: Vec<PointPair> = split_lines(input)
        .into_iter()
        .map(parse_line)
        .filter(|(x1, y1, x2, y2)| include_diagonal || x1 == x2 || y1 == y2)
        .collect();
    let (x_max, y_max) = world_size(&point_pairs);
    let mut world: Vec<i32> = vec![0; (x_max * y_max) as usize];

    for point_pair in point_pairs {
        world = mark_lines(world, point_pair, x_max);
    }

    world
        .into_iter()
        .filter(|n| n.to_owned() >= 2)
        .collect::<Vec<i32>>()
        .len()
}

pub fn a(input: &str) -> String {
    format!("{:?}", run(input, false))
}

pub fn b(input: &str) -> String {
    format!("{:?}", run(input, true))
}

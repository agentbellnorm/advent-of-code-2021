use crate::util::get_index;
use crate::util::int;
use crate::util::log_debug_js;
use std::collections::HashSet;

type Coords = (i32, i32);

#[test]
fn t_coords() {
    assert_eq!(get_coords(10, 10), (0, 1));
    assert_eq!(get_coords(22, 10), (2, 2));
}

fn get_area_height(area: &Vec<i32>, width: i32) -> i32 {
    area.len() as i32 / width
}

fn get_coords(index: i32, width: i32) -> Coords {
    let x = index % width;
    (x, (index - x) / width)
}

fn is_inside((x, y): Coords, width: i32, height: i32) -> bool {
    x >= 0 && x < width && y >= 0 && y < height
}

fn get_adjacent_coords((x, y): Coords) -> HashSet<Coords> {
    HashSet::from([(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)])
}

fn get_height(area: &Vec<i32>, coords: Coords, width: i32, height: i32) -> i32 {
    match is_inside(coords, width, height) {
        false => i32::MAX,
        true => area.get(get_index(coords, width) as usize).unwrap().clone(),
    }
}

fn get_adjacent_heights(area: &Vec<i32>, (x, y): Coords, width: i32) -> HashSet<i32> {
    let height = get_area_height(area, width);

    get_adjacent_coords((x, y))
        .into_iter()
        .map(|coords| get_height(area, coords, width, height))
        .collect::<HashSet<i32>>()
}

fn is_low_point(area: &Vec<i32>, index: i32, width: i32) -> bool {
    let current = area.get(index as usize).unwrap().clone();
    let coords = get_coords(index, width);
    let adjacent = get_adjacent_heights(area, coords, width);

    adjacent.into_iter().all(|adj| adj > current)
}

fn get_risk(area: &Vec<i32>, index: i32, width: i32) -> i32 {
    match is_low_point(area, index, width) {
        true => area.get(index as usize).unwrap().clone() + 1,
        false => 0,
    }
}

fn calculate_risk(input: &str) -> i32 {
    let width = input.find("\n").unwrap() as i32;
    let area = input
        .replace("\n", "")
        .chars()
        .map(|c| int(&c.to_string()))
        .collect::<Vec<i32>>();

    let mut risk = 0;

    for i in 0..area.len() {
        risk += get_risk(&area, i as i32, width);
    }

    risk
}

#[test]
fn t_calculate_risk() {
    assert_eq!(
        calculate_risk(
            "2199943210
3987894921
9856789892
8767896789
9899965678"
        ),
        15
    )
}

pub fn a(input: &str) -> String {
    format!("{:?}", calculate_risk(input))
}

fn get_low_points(area: &Vec<i32>, width: i32) -> HashSet<i32> {
    HashSet::from_iter(
        area.into_iter()
            .enumerate()
            .filter(|(i, _)| is_low_point(area, i.clone() as i32, width))
            .map(|(i, _)| i as i32),
    )
}

fn get_basin(
    area: &Vec<i32>,
    width: i32,
    start_index: i32,
    visited_indices: &HashSet<i32>,
) -> HashSet<i32> {
    // if all adjacent are visited or peaks, mark as visited and return visited.
    let adj_coords = get_adjacent_coords(get_coords(start_index, width));
    let not_visited_not_peak_adj_indices = adj_coords
        .clone()
        .into_iter()
        .filter(|coords| is_inside(coords.clone(), width, get_area_height(area, width)))
        .map(|coords| get_index(coords, width))
        .filter(|i| !visited_indices.contains(i))
        .filter(|idx| area.get(idx.clone() as usize).unwrap() < &9)
        .collect::<HashSet<i32>>();

    let mut to_return = visited_indices.clone();

    to_return.insert(start_index);

    if not_visited_not_peak_adj_indices.is_empty() {
        return to_return;
    }

    for idx in not_visited_not_peak_adj_indices {
        to_return.extend(get_basin(area, width, idx, &to_return))
    }

    to_return
}

pub fn b(input: &str) -> String {
    let width = input.find("\n").unwrap() as i32;
    let area = input
        .replace("\n", "")
        .chars()
        .map(|c| int(&format!("{}", c)))
        .collect::<Vec<i32>>();

    let low_points = get_low_points(&area, width);

    log_debug_js(&low_points);

    let mut res = get_low_points(&area, width)
        .into_iter()
        .map(|low_points| get_basin(&area, width, low_points, &HashSet::new()))
        .map(|basin| basin.len() as i32)
        .collect::<Vec<i32>>();

    res.sort();

    format!(
        "{:?}",
        res.iter().rev().take(3).into_iter().product::<i32>()
    )
}

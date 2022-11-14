use crate::util::{log_all_items, log_debug_js, split_lines};
use itertools::all;
use std::collections::{HashMap, HashSet};

type Adjacent = HashMap<String, HashSet<String>>;

fn get_adjacency_map<'a>(pairs: Vec<(&'a str, &'a str)>) -> Adjacent {
    pairs
        .into_iter()
        .fold(HashMap::new(), |mut map, (from, to)| {
            let from_list: &mut HashSet<String> =
                map.entry(from.to_string()).or_insert(HashSet::new());
            (*from_list).insert(to.to_string());

            let to_list: &mut HashSet<String> = map.entry(to.to_string()).or_insert(HashSet::new());
            (*to_list).insert(from.to_string());
            map
        })
}

fn get_next_nodes_most_1(adjacent: &HashSet<String>, adj_map: &Adjacent) -> HashSet<String> {
    adjacent - &small_caves(adj_map)
}

fn small_caves(adj: &Adjacent) -> HashSet<String> {
    adj.keys()
        .into_iter()
        .filter(|visited_node| (*visited_node).bytes().all(|c| matches!(c, b'a'..=b'z')))
        .map(|s| s.clone())
        .collect::<HashSet<String>>()
}

fn get_paths_to_end(
    current_path: &Vec<String>,
    current: String,
    mut visited: HashMap<String, i32>,
    adjacency_map: &Adjacent,
) -> Vec<Vec<String>> {
    let adjacent_nodes = match adjacency_map.get(&current) {
        Some(adj) => adj,
        None => panic!("could not find {}", &current),
    };

    let next_nodes = adjacent_nodes - &small_caves(adjacency_map);

    let n_visit = visited.entry(current.clone()).or_insert(0);
    *n_visit += 1;

    if current == "end" {
        let mut with_end = current_path.clone();
        with_end.push(format!("end"));
        return vec![with_end];
    }

    let mut res = vec![];
    for node in next_nodes {
        let mut forked_path = current_path.clone();
        forked_path.append(&mut vec![current.clone()]);

        let forked_paths =
            &mut get_paths_to_end(&forked_path, node.clone(), visited.clone(), adjacency_map);

        res.append(forked_paths);
    }

    res
}

pub fn a(_input: &str) -> String {
    // let pairs = split_lines(input)
    //     .into_iter()
    //     .map(|pair| {
    //         let mut spl = pair.split("-");
    //         (spl.next().unwrap(), spl.next().unwrap())
    //     })
    //     .collect::<Vec<(&str, &str)>>();
    //
    // let adjacecy = get_adjacency_map(pairs);
    //
    // log_debug_js(&adjacecy);
    //
    // format!(
    //     "{:?}",
    //     get_paths_to_end(&vec![], format!("start"), HashMap::new(), &adjacecy,).len()
    // )
    "hej".into()
}

fn get_next_nodes() {}

fn get_paths_to_end_one_twice(
    current_path: &Vec<String>,
    current: String,
    mut visited: HashMap<String, i32>,
    adjacency_map: &Adjacent,
    allowed_twice: String,
) -> Vec<Vec<String>> {
    let adjacent_nodes = match adjacency_map.get(&current) {
        Some(adj) => adj,
        None => panic!("could not find {}", &current),
    };

    let n_visit = visited.entry(current.clone()).or_insert(0);
    *n_visit += 1;

    let allowed_two_times = match *visited.get(&allowed_twice).or(Some(&0)).unwrap() <= 2 {
        true => HashSet::new(),
        false => HashSet::from([allowed_twice.clone()]),
    };

    let next_nodes = &(adjacent_nodes - &HashSet::from(["start".to_string()])) - &allowed_two_times;

    log_debug_js(&next_nodes);

    if current == "end" {
        let mut with_end = current_path.clone();
        with_end.push(format!("end"));
        return vec![with_end];
    }

    let mut res = vec![];
    for node in next_nodes {
        let mut forked_path = current_path.clone();
        forked_path.append(&mut vec![current.clone()]);

        let forked_paths = &mut get_paths_to_end_one_twice(
            &forked_path,
            node.clone(),
            visited.clone(),
            adjacency_map,
            allowed_twice.clone(),
        );

        res.append(forked_paths);
    }

    res
}

pub fn b(input: &str) -> String {
    let pairs = split_lines(input)
        .into_iter()
        .map(|pair| {
            let mut spl = pair.split("-");
            (spl.next().unwrap(), spl.next().unwrap())
        })
        .collect::<Vec<(&str, &str)>>();

    let adjacecy = get_adjacency_map(pairs);

    log_debug_js(&adjacecy);

    let mut all_paths = vec![];

    for small_cave in small_caves(&adjacecy) {
        let mut paths = get_paths_to_end_one_twice(
            &vec![],
            format!("start"),
            HashMap::new(),
            &adjacecy,
            small_cave,
        );

        log_debug_js(&paths);

        all_paths.append(&mut paths);
    }

    log_all_items(&all_paths);

    format!("{:?}", all_paths)
}

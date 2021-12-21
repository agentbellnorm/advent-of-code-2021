use crate::util::split_lines;
use std::collections::{BTreeSet, HashMap};

fn is_unique(code: &str) -> bool {
    vec!["fdgacbe".len(), "gcbe".len(), "cgb".len(), "ca".len()].contains(&code.len())
}

pub fn a(input: &str) -> String {
    let n_unique: usize = split_lines(input)
        .into_iter()
        .map(|line| {
            line.split(" | ")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .clone()
        })
        .map(|output| {
            output
                .split(" ")
                .collect::<Vec<&str>>()
                .into_iter()
                .filter(|code| is_unique(code.to_owned()))
                .collect::<Vec<&str>>()
        })
        .fold(0, |acc, v| acc + v.len());

    format!("{:?}", &n_unique)
}

fn char_set(s: &str) -> BTreeSet<char> {
    BTreeSet::from_iter(s.chars())
}

fn get_of_len<'a>(v: &'a Vec<&str>, len: usize) -> &'a str {
    for i in 0..v.len() {
        let value = v.get(i).unwrap();
        if value.len().eq(&len) {
            return value;
        }
    }
    panic!("no value")
}

#[test]
fn decode_t() {
    assert_eq!(
        decode(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
        ),
        5353
    )
}

fn union_set(a: BTreeSet<char>, b: BTreeSet<char>) -> BTreeSet<char> {
    BTreeSet::from_iter(
        a.union(&b)
            .map(|x| x.clone())
            .collect::<Vec<char>>()
            .into_iter(),
    )
}

fn find_number<P: FnMut(&BTreeSet<char>) -> bool>(
    set_of_sets: &BTreeSet<BTreeSet<char>>,
    predicate: P,
) -> BTreeSet<char> {
    set_of_sets
        .clone()
        .into_iter()
        .filter(predicate)
        .collect::<Vec<BTreeSet<char>>>()
        .get(0)
        .unwrap()
        .clone()
}

type Key = HashMap<BTreeSet<char>, i32>;

fn get_key(inn: Vec<&str>) -> Key {
    let one = char_set(get_of_len(&inn, 2));
    let four = char_set(get_of_len(&inn, 4));
    let seven = char_set(get_of_len(&inn, 3));
    let eight = char_set(get_of_len(&inn, 7));

    let mut remaining = inn
        .into_iter()
        .map(char_set)
        .filter(|s| !vec![&one, &four, &seven, &eight].contains(&s))
        .collect::<BTreeSet<BTreeSet<char>>>();

    let six = find_number(&remaining, |remainer| (&eight - &one).is_subset(&remainer));
    remaining.remove(&six);

    let nine = find_number(&remaining, |remainer| four.is_subset(remainer));
    remaining.remove(&nine);

    let zero = find_number(&remaining, |remainer| remainer.len() == 6);
    remaining.remove(&zero);

    let three = find_number(&remaining, |remainer| seven.is_subset(remainer));
    remaining.remove(&three);

    let five = find_number(&remaining, |remainer| {
        four.is_subset(&union_set(remainer.clone(), one.clone()))
    });
    remaining.remove(&five);

    let two = find_number(&remaining, |_| true);

    HashMap::from([
        (zero, 0),
        (one, 1),
        (two, 2),
        (three, 3),
        (four, 4),
        (five, 5),
        (six, 6),
        (seven, 7),
        (eight, 8),
        (nine, 9),
    ])
}

fn combine_ints(ints: Vec<i32>) -> i32 {
    ints.into_iter()
        .map(|int| format!("{}", int))
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap()
}

fn decode(row: &str) -> i32 {
    let inn_and_out = row.split(" | ").into_iter().collect::<Vec<&str>>();
    let inn = inn_and_out.get(0).unwrap().clone().split(" ").collect();
    let out: Vec<&str> = inn_and_out.get(1).unwrap().clone().split(" ").collect();

    let key = get_key(inn);

    combine_ints(
        out.into_iter()
            .map(|output| key.get(&char_set(output)).unwrap().clone())
            .collect(),
    )
}

pub fn b(input: &str) -> String {
    format!(
        "{:?}",
        split_lines(input)
            .into_iter()
            .map(|row| decode(row))
            .sum::<i32>()
    )
}

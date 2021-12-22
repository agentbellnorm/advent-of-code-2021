use crate::util::split_lines;
use itertools::Itertools;

fn get_right(left: char) -> char {
    match left {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        v => panic!("unknown bracket: {}", v),
    }
}

fn is_open(c: char) -> bool {
    vec!['(', '[', '{', '<'].contains(&c)
}

fn corruption_score(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        v => panic!("unknown offender: {}", v),
    }
}

fn completion_score(c: char) -> i64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        v => panic!("unknown offender: {}", v),
    }
}

fn line_score_and_stack(chunks: Vec<char>) -> (i32, Vec<char>) {
    let mut stack: Vec<char> = vec![];
    for c in chunks {
        if stack.is_empty() || is_open(c) {
            stack.push(c);
            continue;
        }

        if !(c == get_right(stack.pop().unwrap())) {
            return (corruption_score(c), stack);
        }
    }

    (0, stack)
}

fn get_completion_sequence(incomplete: Vec<char>) -> Vec<char> {
    incomplete
        .into_iter()
        .map(get_right)
        .rev()
        .collect::<Vec<char>>()
}

fn get_total_completion_score(completion: Vec<char>) -> i64 {
    completion
        .into_iter()
        .fold(0, |total_score, c| total_score * 5 + completion_score(c))
}

fn maybe_get_incomplete_stack(chunk: Vec<char>) -> Option<Vec<char>> {
    let (score, stack) = line_score_and_stack(chunk.to_vec());
    match score {
        0 => Some(stack),
        _ => None,
    }
}

pub fn a(input: &str) -> String {
    format!(
        "{:?}",
        split_lines(input)
            .into_iter()
            .map(|chunks| chunks.chars().collect::<Vec<char>>())
            .map(|chunks| {
                let (score, _) = line_score_and_stack(chunks);
                score
            })
            .sum::<i32>()
    )
}

pub fn b(input: &str) -> String {
    let sorted_scores = split_lines(input)
        .into_iter()
        .map(|chunks| chunks.chars().collect::<Vec<char>>())
        .filter_map(maybe_get_incomplete_stack)
        .map(get_completion_sequence)
        .map(get_total_completion_score)
        .sorted()
        .collect::<Vec<i64>>();

    let middle_score = sorted_scores.get(sorted_scores.len() / 2).unwrap();

    format!("{:?}", middle_score)
}

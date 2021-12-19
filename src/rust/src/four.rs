#[derive(Clone, Debug)]
struct BoardCell {
    value: String,
    marked: bool,
}

type Board = Vec<Vec<BoardCell>>;

fn parse_row(row: &str) -> Vec<BoardCell> {
    return row
        .trim()
        .replace("  ", " ")
        .split(" ")
        .map(|v| BoardCell {
            value: v.to_owned(),
            marked: false,
        })
        .collect();
}

fn _print_board(board: &Board) -> String {
    return format!(
        "{}",
        board
            .into_iter()
            .map(|row| row
                .into_iter()
                .map(|cel| match cel.marked {
                    true => format!("[{}]", cel.value.clone()),
                    false => cel.value.clone(),
                })
                .collect::<Vec<String>>()
                .join(" "))
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn is_bingo(board: &Board) -> bool {
    for i in 0..5 {
        let row_bingo = board.get(i).unwrap().into_iter().all(|cel| cel.marked);
        let col_bingo = board
            .into_iter()
            .map(|row| row.get(i).unwrap().marked)
            .all(|x| x);
        if row_bingo || col_bingo {
            return true;
        }
    }

    return false;
}

#[test]
fn parse_board_t() {
    assert_eq!(
        parse_board("22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19")
            .len(),
        5
    )
}

fn parse_board(raw: &str) -> Board {
    return raw.split("\n").map(|row| parse_row(row)).collect();
}

#[test]
fn mark_board_t() {
    let mut board =
        parse_board("22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19");
    board = get_marked_board(board, "7");
    board = get_marked_board(board, "4");
    board = get_marked_board(board, "9");
    board = get_marked_board(board, "11");
    board = get_marked_board(board, "5");

    assert!(is_marked(&board, 3, 0).eq(&true));
    assert!(is_marked(&board, 3, 1).eq(&true));
    assert!(is_marked(&board, 1, 2).eq(&true));
    assert!(is_marked(&board, 4, 3).eq(&true));
    assert!(is_marked(&board, 0, 0).eq(&false));
    assert!(is_marked(&board, 0, 4).eq(&false));
    assert!(is_marked(&board, 4, 0).eq(&false));
    assert!(is_marked(&board, 4, 4).eq(&false));
}

#[test]
fn bingo_t() {
    let mut board =
        parse_board("22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19");
    board = get_marked_board(board, "7");
    board = get_marked_board(board, "4");
    board = get_marked_board(board, "9");
    board = get_marked_board(board, "11");
    board = get_marked_board(board, "5");
    board = get_marked_board(board, "21");
    board = get_marked_board(board, "14");
    board = get_marked_board(board, "16");

    print_board(&board);

    assert!(is_bingo(&board))
}

fn get_marked_board(board: Board, number: &str) -> Board {
    return board
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|cel| BoardCell {
                    value: cel.value.clone(),
                    marked: cel.marked || cel.value == number,
                })
                .collect()
        })
        .collect();
}

fn get_maybe_bingo(boards: &Vec<Board>) -> Option<&Board> {
    return boards.into_iter().find(|board| is_bingo(&board));
}

fn get_first_bingo_board(mut boards: Vec<Board>, bingo_numbers: Vec<&str>) -> (Board, i32) {
    for number in bingo_numbers {
        boards = boards
            .into_iter()
            .map(|board| get_marked_board(board, number))
            .collect();
        let maybe_bingo = get_maybe_bingo(&boards);
        if maybe_bingo.is_some() {
            return (
                maybe_bingo.unwrap().to_vec().clone(),
                number.parse().unwrap(),
            );
        }
    }

    panic!("no bingo! :(");
}

fn sum_non_marked_numbers(board: &Board) -> i32 {
    return board.into_iter().fold(0, |acc, v| {
        acc + v
            .into_iter()
            .filter(|cell| !cell.marked)
            .fold(0, |row_sum, cell| {
                row_sum + cell.value.parse::<i32>().unwrap()
            })
    });
}

pub fn a(input: &str) -> String {
    let mut top_level = input.split("\n\n").into_iter();
    let bingo_numbers: Vec<&str> = top_level.next().unwrap().split(",").collect();
    let boards: Vec<Board> = top_level.map(|board_raw| parse_board(board_raw)).collect();

    let (first_bingo_board, winning_number) = get_first_bingo_board(boards, bingo_numbers);

    return format!(
        "{:?}",
        sum_non_marked_numbers(&first_bingo_board) * winning_number
    );
}

/* B */

fn get_winners(boards: &Vec<Board>) -> Vec<usize> {
    let mut to_return: Vec<usize> = Vec::new();

    for (i, board) in boards.into_iter().enumerate() {
        if is_bingo(board) {
            to_return.push(i);
        }
    }

    return to_return;
}

fn get_board(boards: &Vec<Board>, i: usize) -> Board {
    return boards.get(i).unwrap().clone();
}

fn remove_multi(boards: Vec<Board>, idxs: Vec<usize>) -> Vec<Board> {
    let mut to_return: Vec<Board> = Vec::new();

    for (i, board) in boards.into_iter().enumerate() {
        if !idxs.contains(&i) {
            to_return.push(board);
        }
    }

    return to_return;
}

pub fn b(input: &str) -> String {
    let mut top_level = input.split("\n\n").into_iter();
    let bingo_numbers: Vec<&str> = top_level.next().unwrap().split(",").collect();
    let mut boards: Vec<Board> = top_level.map(|board_raw| parse_board(board_raw)).collect();

    let mut bingos: Vec<(Board, &str)> = Vec::new();

    for number in bingo_numbers {
        boards = boards
            .into_iter()
            .map(|board| get_marked_board(board, number))
            .collect();

        let winners = get_winners(&boards);

        for winner in winners.clone() {
            bingos.push((get_board(&boards, winner), number));
        }
        boards = remove_multi(boards, winners);

        if boards.is_empty() {
            break;
        }
    }

    let (last_bingo_board, winning_number) = match bingos.last() {
        Some((b, n)) => (b, n.trim().parse::<i32>().unwrap()),
        None => panic!("aj aj"),
    };

    return format!(
        "{:?}",
        sum_non_marked_numbers(last_bingo_board) * winning_number
    );
}

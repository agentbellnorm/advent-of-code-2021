use web_sys::console::log_1 as log;

struct BoardCell {
    value: String,
    marked: bool,
}

fn parse_row(row: &str) -> Vec<BoardCell> {
    return row
        .trim()
        .replace("  ", " ")
        .split(" ")
        .map(|v| BoardCell { value: v.to_owned(), marked: false })
        .collect();
}

fn print_board(board: &Vec<Vec<BoardCell>>) -> String {
    return format!("{}", board.into_iter()
        .map(|row| row
            .into_iter()
            .map(|cel| match cel.marked {
                true => format!("[{}]", cel.value.clone()),
                false => cel.value.clone()
            })
            .collect::<Vec<String>>()
            .join(" ")
        )
        .collect::<Vec<String>>()
        .join("\n"));
}

fn is_marked(board: &Vec<Vec<BoardCell>>, x: i32, y: i32) -> bool {
    return board.get(y as usize).unwrap().get(x as usize).unwrap().marked;
}

fn is_bingo(board: &Vec<Vec<BoardCell>>) -> bool {
    for i in 0..4 {
        let row_bingo = board.get(i).unwrap().into_iter().all(|cel| cel.marked);
        let col_bingo = board.into_iter().map(|row| row.get(i).unwrap().marked).all(|x| x);
        if row_bingo || col_bingo {
            return true;
        }
    }

    return false;
}

#[test]
fn parse_board_t() {
    assert_eq!(parse_board("22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19").len(), 5)
}

fn parse_board(raw: &str) -> Vec<Vec<BoardCell>> {
    return raw.split("\n").map(|row| parse_row(row)).collect();
}


#[test]
fn mark_board_t() {
    let mut board = parse_board("22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19");
    board = mark_board(board, "7");
    board = mark_board(board, "4");
    board = mark_board(board,"9");
    board = mark_board(board, "11");
    board = mark_board(board, "5");

    print_board(&board);

    assert!(is_marked(&board, 3, 0).eq(&true));
    assert!(is_marked(&board, 3, 1).eq(&true));
    assert!(is_marked(&board,1, 2).eq(&true));
    assert!(is_marked(&board, 4, 3).eq(&true));
    assert!(is_marked(&board, 0, 0).eq(&false));
    assert!(is_marked(&board, 0, 4).eq(&false));
    assert!(is_marked(&board, 4, 0).eq(&false));
    assert!(is_marked(&board, 4, 4).eq(&false));
}

#[test]
fn bingo_t() {
    let mut board = parse_board("22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19");
    board = mark_board(board, "7");
    board = mark_board(board, "4");
    board = mark_board(board,"9");
    board = mark_board(board, "11");
    board = mark_board(board, "5");
    board = mark_board(board, "21");
    board = mark_board(board, "14");
    board = mark_board(board, "16");

    print_board(&board);

    assert!(is_bingo(&board))
}

fn mark_board(board: Vec<Vec<BoardCell>>, number: &str) -> Vec<Vec<BoardCell>> {
    return board
        .into_iter()
        .map(|row|
            row
                .into_iter()
                .map(|cel| BoardCell { value: cel.value.clone(), marked: cel.marked || cel.value == number })
                .collect())
        .collect();
}

fn get_maybe_bingo(boards: &Vec<Vec<Vec<BoardCell>>>) -> Option<&Vec<Vec<BoardCell>>> {
    return boards.into_iter().find(|board| is_bingo(&board));
}

pub fn a(input: &str) -> String {
    let mut top_level = input.split("\n\n").into_iter();
    let bingo_numbers: Vec<&str> = top_level.next().unwrap().split(",").collect();
    let mut boards: Vec<Vec<Vec<BoardCell>>> = top_level.map(|board_raw| parse_board(board_raw)).collect();

    for number in bingo_numbers {
        boards = boards.into_iter().map(|board| mark_board(board, number)).collect();
        let maybe_bingo = get_maybe_bingo(&boards);
        if maybe_bingo.is_some() {
            log(&print_board(maybe_bingo.unwrap()).into());
            break;
        }
    }

    return format!("{:?}", boards.len());
}

pub fn b(input: &str) -> String {
    return format!("{:?}", "Not implemented");
}



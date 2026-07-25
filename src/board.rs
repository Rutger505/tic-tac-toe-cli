use crate::cell::Cell;

type Board = [[Cell; 3]; 3];

pub fn init_board() -> Board {
    [
        [Cell::Empty, Cell::Empty, Cell::Empty],
        [Cell::Empty, Cell::Empty, Cell::Empty],
        [Cell::Empty, Cell::Empty, Cell::Empty],
    ]
}

pub fn print_board(board: Board, title: &str) {
    println!("{}", title);
    for row in 0..3 {
        for column in 0..3 {
            print!(" {:^1}", board[row][column].to_string());
            if column != 2 {
                print!(" |");
            }
        }
        println!();
        if row != 2 {
            println!("-----------");
        }
    }
}

pub fn get_game_won(board: Board) -> Option<Cell> {
    let rows = (0..3).map(|r| [board[r][0], board[r][1], board[r][2]]);
    let cols = (0..3).map(|c| [board[0][c], board[1][c], board[2][c]]);
    let diags = [
        [board[0][0], board[1][1], board[2][2]],
        [board[0][2], board[1][1], board[2][0]],
    ];

    rows.chain(cols).chain(diags).find_map(check_slice)
}

pub fn check_slice(slice: [Cell; 3]) -> Option<Cell> {
    let first = slice[0];
    if first == Cell::Empty {
        return None;
    }
    slice[1..].iter().all(|&v| v == first).then_some(first)
}

mod board;
mod cell;

use crate::board::{get_game_won, init_board, is_board_full, print_board};
use crate::cell::Cell;
use std::io;
use std::io::Write;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    io::stdout().into_raw_mode().unwrap();

    let mut board = init_board();

    let mut cell_won: Option<Cell>;

    let mut turn = 0;

    loop {
        let key = io::stdin().keys().next().unwrap().unwrap();
        println!("{:?}", key);

        let cell = if turn % 2 == 0 {
            Cell::Cross
        } else {
            Cell::Circle
        };

        print_board(board, &format!("{}'s Turn!", cell));

        let (mut x, mut y) = get_user_coordinate();
        while board[x - 1][y - 1] != Cell::Empty {
            println!("Location taken!");
            (x, y) = get_user_coordinate();
        }

        board[x - 1][y - 1] = cell;

        cell_won = get_game_won(board);

        turn += 1;

        if cell_won.is_some() || is_board_full(board) {
            break;
        }
    }

    let title = if let Some(won) = cell_won {
        &format!("{} Won!!", won.to_string().to_uppercase())
    } else {
        "Tie.."
    };

    print_board(board, title);
}

// TODO replace with tui navigation \/
fn get_user_coordinate() -> (usize, usize) {
    (
        get_user_number("Enter row number", 1, 3)
            .try_into()
            .unwrap(),
        get_user_number("Enter column number", 1, 3)
            .try_into()
            .unwrap(),
    )
}

fn get_user_number(prompt: &str, min: i32, max: i32) -> i32 {
    let mut number = "".parse::<i32>();

    loop {
        match number {
            Ok(n) if n >= min && n <= max => break,
            _ => {
                let mut input = String::new();

                print!("{prompt} ({min}-{max}): ");
                io::stdout().flush().unwrap();

                io::stdin().read_line(&mut input).unwrap();
                input = input.trim_end().to_string();

                number = input.parse::<i32>();
            }
        }
    }

    number.unwrap()
}

mod board;
mod cell;

use crate::board::{get_game_won, init_board, print_board};
use crate::cell::Cell;
use std::io;
use std::io::{Read, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    // Get and lock the stdios.
    let stdout = io::stdout().into_raw_mode().unwrap();

    println!("{:?}", io::stdin().keys().next().unwrap().unwrap());

    let mut board = init_board();

    let mut field_value_won: Option<Cell> = None;

    let total_cells = 3 * 3;
    let mut cells_filled = 0;

    while field_value_won.is_none() && cells_filled < total_cells {
        let field_value = if cells_filled % 2 == 0 {
            Cell::Cross
        } else {
            Cell::Circle
        };

        print_board(board);

        println!("{}'s Turn!", field_value.to_string().to_uppercase());
        let (mut x, mut y) = get_user_coordinate();
        while board[x - 1][y - 1] != Cell::Empty {
            println!("Location taken!");
            (x, y) = get_user_coordinate();
        }

        board[x - 1][y - 1] = field_value;

        field_value_won = get_game_won(board);

        cells_filled += 1;
    }

    print_board(board);

    if let Some(won) = field_value_won {
        println!("{} Won!!", won.to_string().to_uppercase());
    } else {
        println!("Tie..");
    }
}

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

mod board;
mod cell;
mod interface;

use crate::board::{get_game_won, init_board, is_board_full};
use crate::cell::Cell;
use crate::interface::Interface;
use std::io;
use std::io::Write;
use termion::input::TermRead;

fn main() {
    let mut interface = Interface::new();
    let mut board = init_board();

    let mut cell_won: Option<Cell>;
    let mut turn = 0;

    loop {
        let cell = if turn % 2 == 0 {
            Cell::Cross
        } else {
            Cell::Circle
        };

        interface.print_board(board, &format!("{}'s Turn!", cell));

        let (mut x, mut y) = interface.get_user_cell();
        while board[x as usize][y as usize] != Cell::Empty {
            interface.print_board(board, &format!("{}'s Turn! *Location taken!*", cell));
            (x, y) = interface.get_user_cell();
        }

        board[x as usize][y as usize] = cell;

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
    interface.print_board(board, title);
}

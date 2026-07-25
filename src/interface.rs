use crate::board::Board;
use std::io::Write;
use std::io::{Stdout, stdout};
use termion::raw::{IntoRawMode, RawTerminal};

use termion::clear;
use termion::cursor::Goto;

pub struct Interface {
    pub raw_terminal: RawTerminal<Stdout>,
    x: u16,
    y: u16,
}

impl Interface {
    const BOARD_START: u16 = 2;
    const BOARD_END: u16 = 2 + 5;
    const BOARD_COLS: [u16; 3] = [2, 5, 8];
    const BOARD_ROWS: [u16; 3] = [
        Self::BOARD_START,
        Self::BOARD_START + 2,
        Self::BOARD_START + 4,
    ];

    pub fn new() -> Interface {
        let raw_terminal = stdout().into_raw_mode().unwrap();

        Interface {
            raw_terminal,
            x: Self::BOARD_COLS[0],
            y: Self::BOARD_ROWS[0],
        }
    }

    pub fn print_board(&self, board: Board, title: &str) {
        print!("{}", clear::All);

        print!("{}{}", Goto(1, 1), title);

        print!("{}   |   |   ", Goto(1, Self::BOARD_START));
        print!("{}===========", Goto(1, Self::BOARD_START + 1));
        print!("{}   |   |   ", Goto(1, Self::BOARD_START + 2));
        print!("{}===========", Goto(1, Self::BOARD_START + 3));
        print!("{}   |   |   ", Goto(1, Self::BOARD_START + 4));

        for (row_data, row_coordinate) in board.iter().zip(Self::BOARD_ROWS) {
            for (col_data, col_coordinate) in row_data.iter().zip(Self::BOARD_COLS) {
                print!("{}{}", Goto(row_coordinate, col_coordinate), col_data);
            }
        }

        print!("{}", Goto(self.x, self.y));

        stdout().flush().unwrap();
    }
}

impl Drop for Interface {
    fn drop(&mut self) {
        print!("{}", Goto(1, Self::BOARD_END));
    }
}

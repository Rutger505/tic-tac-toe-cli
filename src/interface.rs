use crate::board::Board;
use std::io::{Stdout, stdout};
use std::io::{Write, stdin};
use termion::clear;
use termion::cursor::Goto;
use termion::event::Key::{Char, Down, Left, Right, Up};
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Interface {
    pub raw_terminal: RawTerminal<Stdout>,
    board_x: u16,
    board_y: u16,
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
            board_x: 0,
            board_y: 0,
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

        stdout().flush().unwrap();
    }

    pub fn get_user_cell(&mut self) {
        loop {
            print!(
                "{}",
                Goto(
                    2 + (self.board_x * 4),
                    Self::BOARD_START + (self.board_y * 2)
                )
            );
            stdout().flush().unwrap();

            let key = stdin().keys().next().unwrap().unwrap();
            match key {
                Char('k') | Up => self.board_y = (self.board_y + 2) % 3,
                Char('j') | Down => self.board_y = (self.board_y + 1) % 3,
                Char('h') | Left => self.board_x = (self.board_x + 2) % 3,
                Char('l') | Right => self.board_x = (self.board_x + 1) % 3,
                _ => {}
            }
        }
    }
}

impl Drop for Interface {
    fn drop(&mut self) {
        print!("{}", Goto(1, Self::BOARD_END));
    }
}

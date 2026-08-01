use crate::cell::Cell;
use crate::game::{Game, GameResult, PlayRoundError, PlayRoundResult};
use std::io::{Stdout, stdout};
use std::io::{Write, stdin};
use termion::clear;
use termion::cursor::Goto;
use termion::event::Key::{Char, Ctrl, Down, Left, Right, Up};
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Interface {
    pub raw_terminal: RawTerminal<Stdout>,
    game: Game,
    board_x: u8,
    board_y: u8,
}

pub enum InterfaceResult {
    Exited,
    Cell(u8, u8),
}

impl Interface {
    const BOARD_START: u16 = 2;
    const BOARD_END: u16 = 2 + 5;
    const BOARD_COLS: [u16; 3] = [2, 6, 10];
    const BOARD_ROWS: [u16; 3] = [
        Self::BOARD_START,
        Self::BOARD_START + 2,
        Self::BOARD_START + 4,
    ];

    pub fn new(game: Game) -> Interface {
        Interface {
            raw_terminal: stdout().into_raw_mode().unwrap(),
            game,
            board_x: 0,
            board_y: 0,
        }
    }

    pub fn listen_to_input(&mut self) {
        let mut title: String;

        loop {
            title = format!("{}'s Turn!", self.game.get_turn_cell());
            self.print_ui(self.game.get_cells(), &title);

            let key = stdin().keys().next().unwrap().unwrap();
            match key {
                Char('k') | Up => self.board_y = (self.board_y + 2) % 3,
                Char('j') | Down => self.board_y = (self.board_y + 1) % 3,
                Char('h') | Left => self.board_x = (self.board_x + 2) % 3,
                Char('l') | Right => self.board_x = (self.board_x + 1) % 3,

                Char(' ') => match self.game.play_round(self.board_x, self.board_y) {
                    Err(reason) => match reason {
                        PlayRoundError::LocationTaken => {
                            title =
                                format!("{}'s Turn! *Location Taken*", self.game.get_turn_cell())
                        }
                    },

                    Ok(reason) => match reason {
                        PlayRoundResult::GameFinished(game_result) => {
                            match game_result {
                                GameResult::CellWon(cell_won) => {
                                    title = format!("{} Won!!", cell_won.to_string().to_uppercase())
                                }
                                GameResult::Tie => {
                                    title = "Tie..".to_string();
                                }
                            }
                            break;
                        }
                        PlayRoundResult::Placed => {}
                    },
                },

                Ctrl('c') | Char('q') => return,

                _ => {}
            }
        }

        self.print_ui(self.game.get_cells(), &title);
    }

    pub fn print_ui(&self, board: [[Cell; 3]; 3], title: &str) {
        print!("{}", clear::All);

        print!("{}{}", Goto(1, 1), title);
        print!("{}   │   │   ", Goto(1, Self::BOARD_START));
        print!("{}───┼───┼───", Goto(1, Self::BOARD_START + 1));
        print!("{}   │   │   ", Goto(1, Self::BOARD_START + 2));
        print!("{}───┼───┼───", Goto(1, Self::BOARD_START + 3));
        print!("{}   │   │   ", Goto(1, Self::BOARD_START + 4));

        for (&col, col_data) in Self::BOARD_COLS.iter().zip(board) {
            for (&row, row_data) in Self::BOARD_ROWS.iter().zip(col_data) {
                print!("{}{}", Goto(col, row), row_data);
            }
        }

        print!(
            "{}",
            Goto(
                2 + (self.board_x as u16 * 4),
                Self::BOARD_START + (self.board_y as u16 * 2)
            )
        );

        stdout().flush().unwrap();
    }
}

impl Drop for Interface {
    fn drop(&mut self) {
        print!("{}", Goto(1, Self::BOARD_END));
    }
}

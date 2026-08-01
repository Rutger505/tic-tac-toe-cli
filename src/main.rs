mod board;
mod cell;
mod game;
mod interface;

use crate::board::Board;
use crate::cell::Cell;
use crate::game::Game;
use crate::interface::{Interface, InterfaceResult};

fn main() {
    let board = Board::new();
    let game = Game::new(board);
    let mut interface = Interface::new(game);

    interface.listen_to_input();
}

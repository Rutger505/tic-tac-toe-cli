mod board;
mod cell;
mod interface;

use crate::board::Board;
use crate::cell::Cell;
use crate::interface::{Interface, InterfaceResult};

fn main() {
    let board = Board::new();
    let mut interface = Interface::new(board);

    interface.listen_to_input();
}

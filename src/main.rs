mod board;
mod cell;
mod interface;

use crate::board::Board;
use crate::cell::Cell;
use crate::interface::Interface;

fn main() {
    let mut interface = Interface::new();
    let mut board = Board::new();

    let mut turn = 0;
    while board.get_game_won().is_none() && !board.is_full() {
        let cell = if turn % 2 == 0 {
            Cell::Cross
        } else {
            Cell::Circle
        };

        interface.print_board(board.get_cells(), &format!("{}'s Turn!", cell));

        loop {
            let (x, y) = interface.get_user_cell();

            if !board.is_place_taken(x, y) {
                board.set_cell(x, y, cell);
                break;
            }

            interface.print_board(
                board.get_cells(),
                &format!("{}'s Turn! *Location taken!*", cell),
            );
        }

        turn += 1;
    }

    let title = if let Some(won) = board.get_game_won() {
        &format!("{} Won!!", won.to_string().to_uppercase())
    } else {
        "Tie.."
    };
    interface.print_board(board.get_cells(), title);
}

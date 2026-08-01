use crate::board::Board;
use crate::cell::Cell;

pub struct Game {
    turn: u8,
    board: Board,
}

pub enum GameResult {
    CellWon(Cell),
    Tie,
}

pub enum PlayRoundResult {
    GameFinished(GameResult),
    Placed,
}
pub enum PlayRoundError {
    LocationTaken,
}

impl Game {
    pub fn new(board: Board) -> Game {
        Game { turn: 0, board }
    }

    pub fn get_turn_cell(&self) -> Cell {
        if self.turn % 2 == 0 {
            Cell::Cross
        } else {
            Cell::Circle
        }
    }

    pub fn get_cells(&self) -> [[Cell; 3]; 3] {
        self.board.get_cells()
    }

    pub fn play_round(&mut self, x: u8, y: u8) -> Result<PlayRoundResult, PlayRoundError> {
        if self.board.is_place_taken(x, y) {
            return Err(PlayRoundError::LocationTaken);
        }

        self.board.set_cell(x, y, self.get_turn_cell());

        self.turn += 1;

        if let Some(cell_won) = self.board.get_game_won() {
            Ok(PlayRoundResult::GameFinished(GameResult::CellWon(cell_won)))
        } else if self.board.is_full() {
            Ok(PlayRoundResult::GameFinished(GameResult::Tie))
        } else {
            Ok(PlayRoundResult::Placed)
        }
    }
}

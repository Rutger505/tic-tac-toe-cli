use crate::cell::Cell;

pub struct Board {
    turn: u8,
    cells: [[Cell; 3]; 3],
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

impl Board {
    pub fn new() -> Board {
        Board {
            turn: 0,
            cells: [
                [Cell::Empty, Cell::Empty, Cell::Empty],
                [Cell::Empty, Cell::Empty, Cell::Empty],
                [Cell::Empty, Cell::Empty, Cell::Empty],
            ],
        }
    }

    pub fn get_cells(&self) -> [[Cell; 3]; 3] {
        self.cells
    }

    pub fn get_turn_cell(&self) -> Cell {
        if self.turn % 2 == 0 {
            Cell::Cross
        } else {
            Cell::Circle
        }
    }

    pub fn is_full(&self) -> bool {
        for row in self.cells {
            for col in row {
                if col == Cell::Empty {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_game_won(&self) -> Option<Cell> {
        let rows = (0..3).map(|r| [self.cells[r][0], self.cells[r][1], self.cells[r][2]]);
        let cols = (0..3).map(|c| [self.cells[0][c], self.cells[1][c], self.cells[2][c]]);
        let diags = [
            [self.cells[0][0], self.cells[1][1], self.cells[2][2]],
            [self.cells[0][2], self.cells[1][1], self.cells[2][0]],
        ];

        rows.chain(cols).chain(diags).find_map(Self::check_slice)
    }
    fn check_slice(slice: [Cell; 3]) -> Option<Cell> {
        let first = slice[0];
        if first == Cell::Empty {
            return None;
        }
        slice[1..].iter().all(|&v| v == first).then_some(first)
    }

    pub fn play_round(&mut self, x: u8, y: u8) -> Result<PlayRoundResult, PlayRoundError> {
        let cell_taken = self.cells[x as usize][y as usize] != Cell::Empty;
        if cell_taken {
            return Err(PlayRoundError::LocationTaken);
        }

        self.cells[x as usize][y as usize] = self.get_turn_cell();

        self.turn += 1;

        if let Some(cell_won) = self.get_game_won() {
            Ok(PlayRoundResult::GameFinished(GameResult::CellWon(cell_won)))
        } else if self.is_full() {
            Ok(PlayRoundResult::GameFinished(GameResult::Tie))
        } else {
            Ok(PlayRoundResult::Placed)
        }
    }
}

use std::fmt;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Cell {
    Empty,
    Cross,
    Circle,
}
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, ""),
            Cell::Cross => write!(f, "X"),
            Cell::Circle => write!(f, "O"),
        }
    }
}

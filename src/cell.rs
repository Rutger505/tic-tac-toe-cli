#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Cell {
    Empty,
    Cross,
    Circle,
}
impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, ""),
            Cell::Cross => write!(f, "X"),
            Cell::Circle => write!(f, "O"),
        }
    }
}

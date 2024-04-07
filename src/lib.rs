use std::option::Option;
#[derive(Debug)]
pub struct ChessPosition{
    rank: i32, // row
    file: i32, // column
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> { // new creates a new ChessPosition
        // if the rank and file are between 0 and 7, we return Some(ChessPosition); if not, we return None
        match (rank, file) { // match allows us to compare a value against a series of patterns
            (0..=7, 0..=7) => Some(ChessPosition { rank, file }),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen(position) // we create a new Queen with the position
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.0.rank == other.0.rank
            || self.0.file == other.0.file
            || (self.0.rank - other.0.rank).abs() == (self.0.file - other.0.file).abs()
    }
}


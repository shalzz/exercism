#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            None
        } else {
            Some(ChessPosition { rank, file })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        match (other.0.rank, other.0.file) {
            (x, _) if x == self.0.rank => true,
            (_, y) if y == self.0.file => true,
            (_, _) => (self.0.rank - other.0.rank).abs() == (self.0.file - other.0.file).abs(),
        }
    }
}

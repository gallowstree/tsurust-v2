pub type PathIndex = usize;


#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub row: usize,
    pub col: usize,
    pub path_index: PathIndex
}


#[derive(Debug, Copy, Clone)]
pub struct Stone {
    pub color: PlayerColor,
    //TODO: maybe this should be a fn computed from a Trajectory (stack (?) of Positions)
    position: Position
}

impl Stone {
    pub fn position(&self) -> Position {
        self.position
    }
}
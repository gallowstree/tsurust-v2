use serde::{Serialize, Deserialize};

pub type PathIndex = usize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub color: PlayerColor,
    pub name: String,
    trajectory: Vec<Position>
}

impl Player {
    pub fn position(&self) -> &Position {
        self.trajectory.last().expect("stone should have a position")
    }

    pub fn at(initial_pos: Position, color: PlayerColor, name: &str) -> Player {
        let trajectory = vec![initial_pos];
        Player {color, trajectory, name: name.to_string()}
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Position {
    pub row: usize,
    pub col: usize,
    pub path_index: PathIndex
}

impl Position {
    pub fn new(row: usize, col: usize, path: PathIndex) -> Position {
        Position {row, col, path_index: path}
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum PlayerColor {
    WHITE, RED, YELLOW,
    BLUE, GRAY, ORANGE,
    GREEN, BLACK
}

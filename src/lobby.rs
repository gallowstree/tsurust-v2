use crate::board::{Position, Stone};
use crate::lobby::LobbyEvent::{NewPlayer, ConfirmPosition};
use std::collections::HashMap;
use crate::states::Event;

// Valid events in this state
pub enum LobbyEvent {
    NewPlayer {name: str, pos: Position},
    Done
}

pub struct Lobby {
    pub players: HashMap<str, Stone>
}

impl Lobby {

    pub fn apply(self, event: LobbyEvent) -> Lobby {
        match event {
            NewPlayer {name, pos} => with_player(name, pos),
            Done => self
        }
    }
}

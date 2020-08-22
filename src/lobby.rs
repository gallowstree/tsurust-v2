use crate::board::{Position, Stone};
use std::collections::HashMap;
use crate::states::{PlayerEvent::*, PlayerEventConsumer, EventResult};

pub struct Lobby {
    pub players: HashMap<str, Stone>
}

impl PlayerEventConsumer for Lobby {
    fn consume(self, event: ClientEvent) -> EventResult {
        match event {
            NewPlayer {name, pos} => self.new_player(&name, pos),
            _ => panic!("invalid for lobby event")
        }
    }
}

impl Lobby {
    fn new_player(self, name: &str, pos: Position) -> EventResult {
        unimplemented!()
    }
}
use std::collections::HashMap;
use rand::{seq::SliceRandom, thread_rng};
use crate::board::{Position, Player, PlayerColor, PlayerColor::*};
use crate::states::{PlayerEvent::*, PlayerEventConsumer, EventResult, PlayerEvent};
use crate::states::GameEvent::{StartGame, PlayerJoined};

pub struct Lobby {
    players: HashMap<String, Player>,
    available_colors: Vec<PlayerColor>
}

impl Default for Lobby {
    fn default() -> Self {
        let players = HashMap::new();
        let mut rng = thread_rng();
        let mut available_colors = vec![WHITE, RED, YELLOW, BLUE, GRAY, ORANGE, GREEN, BLACK];
        available_colors.shuffle(&mut rng);

        Lobby {players, available_colors}
    }
}

impl PlayerEventConsumer for Lobby {
    fn consume(&mut self, event: PlayerEvent) -> EventResult {
        match event {
            NewPlayer {name, pos} => self.new_player(&name, pos),
            EndLobby => EventResult::event(StartGame),
            _ => EventResult::error("invalid event during lobby state")
        }
    }
}

impl Lobby {
    fn new_player(&mut self, name: &str, pos: Position) -> EventResult {
        if self.players.contains_key(name) {
            return EventResult::error("Player name exists");
        }

        let color = self.available_colors.pop();
        match color {
            None => EventResult::error("Lobby full"),
            Some(color) => self.add_player(name, pos, color)
        }
    }

    fn add_player(&mut self, name: &str, pos: Position, color: PlayerColor) -> EventResult {
        self.players.insert(name.to_string(), Player::at(pos, color, name));

        let mut game_events = Vec::new();
        game_events.push(PlayerJoined {players: self.players.values().collect()});

        if self.available_colors.is_empty() {
            game_events.push(StartGame);
        }
        EventResult::Ok(game_events)
    }
}
use crate::board::{Position, Player};
use crate::states::EventResult::Error;

//TODO maybe PlayerAction is more appropriate?
pub enum PlayerEvent {
    StartLobby, // Maybe this one will change. Game ID? Future...
    NewPlayer {name: String, pos: Position},
    EndLobby,
    // -------
    PlaceTile
}

pub trait PlayerEventConsumer {
    fn consume(&mut self, event: PlayerEvent) -> EventResult;
}

#[derive(Debug)]
pub enum GameEvent<'a> {
    PlayerJoined{ players: Vec<&'a Player>},
    StartGame,
    DealHand, //(s)?
    StartTurn,
}

//TODO: may become a Result<Vec<GameEvent>>?
#[derive(Debug)]
pub enum EventResult<'a> {
    Ok(Vec<GameEvent<'a>>),
    Error(String)//TODO: should errors be enum values instead?
}

impl EventResult<'_> {
    pub fn event(single_event: GameEvent) -> EventResult {
        EventResult::Ok(vec![single_event])
    }

    pub fn error(msg: &str) -> EventResult {
        Error(msg.to_string())
    }
}


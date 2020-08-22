use crate::board::Position;

pub enum PlayerEvent {
    StartLobby, // Maybe this one will change. Game ID? Future...
    NewPlayer {name: str, pos: Position},
    EndLobby,
    // -------
    PlaceTile
}

pub trait PlayerEventConsumer {
    fn consume(self, event: PlayerEvent) -> EventResult;
}

pub enum GameEvent {
    DealHand, //(s)?
    StartTurn,
}

pub enum EventResult {
    Mip {game_events: Vec<GameEvent>}
}



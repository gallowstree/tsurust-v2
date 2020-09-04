use crate::states::{PlayerEventConsumer, PlayerEvent};
use crate::lobby::Lobby;

pub struct Game {
    current_stage: Box<dyn PlayerEventConsumer>
}

impl Game {
    fn new() -> Game {
        Game { current_stage: Box::new(Lobby::default()) }
    }

    fn receive(&mut self, message: PlayerEvent) {
        self.current_stage.consume(message);
    }
}
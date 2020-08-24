use tsurust_v2::lobby::Lobby;
use tsurust_v2::states::{PlayerEventConsumer, PlayerEvent};
use tsurust_v2::board::Position;

fn main() {
    println!("TsuRust!");

    let mut lobby = Lobby::default();

    let event = PlayerEvent::NewPlayer {name: "mushi".to_string(), pos: Position::new(0,0,5)};
    let results = lobby.consume(event);

    println!("{:?}", results);

}

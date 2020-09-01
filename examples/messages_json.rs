use serde_json::Result;
use tsurust_v2::states::PlayerEvent;
use tsurust_v2::board::Position;
use tsurust_v2::states::PlayerEvent::{StartLobby, EndLobby};

fn main() -> Result<()>{
    println!("Lobby events:");

    let new_player = PlayerEvent::NewPlayer {name: "mushi".to_string(), pos: Position::new(0,0,5)};
    let new_player = serde_json::to_string(&new_player)?;

    println!("StartLobby:\n{}", serde_json::to_string(&StartLobby)?);
    println!("New Player:\n{}", new_player);
    println!("EndLobby:\n{}", serde_json::to_string(&EndLobby)?);

    Ok(())
}
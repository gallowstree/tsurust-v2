use std::any::Any;

use futures::{future::ready, StreamExt as FutureStreamExt};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender};
use warp::Filter;
use warp::ws::{Message as WsMessage, WebSocket};

use tsurust_v2::board::Position;
use tsurust_v2::lobby::Lobby;
use tsurust_v2::states::{PlayerEvent, PlayerEventConsumer};

#[tokio::main]
async fn main() {
    let (mut events_tx, mut events_rx) = mpsc::unbounded_channel();
    tokio::task::spawn(event_loop(events_rx));

    let events_tx_filter = warp::any()
        .map(move || events_tx.clone());

    let websocket_route = warp::path("tsurust-ws")
        .and(warp::ws())
        .and(events_tx_filter)
        .map(move |ws: warp::ws::Ws, events_tx: Sender<PlayerEvent>| {
            ws.on_upgrade(|socket| player_ws_loop(socket, events_tx))
        });

    warp::serve(websocket_route)
        .run(([127, 0, 0, 1], 3030)).await;
}

async fn event_loop(mut mpsc_rx: Receiver<PlayerEvent>) {
    println!("TsuRust event loop running!");

    while let Some(event) = mpsc_rx.next().await {
        print!("got {:?}", event);
    }
}

async fn player_ws_loop(ws: WebSocket, mut to_event_loop: Sender<PlayerEvent>) {
    let (ws_out, mut ws_in) = ws.split();

    while let Some(result) = ws_in.next().await { //and_then()?
        let msg = match result {
            Ok(msg) if msg.is_close() => "close".to_owned(), //would probably need to break instead
            Ok(msg) if msg.is_text() => msg.to_str().expect("expected text msg").to_owned(),
            Ok(_) => continue,
            Err(e) => {
                eprintln!("websocket error: {}", e);
                break;
            }
        };

        println!("Got {}", msg);
    }


}
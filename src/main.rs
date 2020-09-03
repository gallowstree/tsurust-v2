use std::any::Any;

use futures::{future::ready, StreamExt as FutureStreamExt};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender};
use warp::Filter;
use warp::ws::{Message as WsMessage, WebSocket};

use tsurust_v2::board::Position;
use tsurust_v2::lobby::Lobby;
use tsurust_v2::states::{PlayerEvent, PlayerEventConsumer};
use tsurust_v2::states::PlayerEvent::EndLobby;

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
            ws.on_upgrade(|socket| client_connected(socket, events_tx))
        });

    warp::serve(websocket_route)
        .run(([127, 0, 0, 1], 3030)).await;
}

async fn event_loop(mut mpsc_rx: Receiver<PlayerEvent>) {
    println!("TsuRust event loop running!");

    loop {
        match mpsc_rx.next().await {
            Some(event) => println!("got {:?}", event),
            _ => println!("wtff")
        }
    }
}

async fn client_connected(ws: WebSocket, mut to_event_loop: Sender<PlayerEvent>) {
    let (ws_out, mut ws_in) = ws.split(); //maybe spawn an outbound task?
    println!("connected");


    // can extract to async fn
    while let Some(result) = ws_in.next().await { //and_then()?
        match result {
            Ok(msg) => {
                parse_message(msg).map(|msg| {
                    println!("sending {:?}", msg);
                    to_event_loop.clone().send(msg);
                })
            } ,
            Err(e) => {
                eprintln!("websocket error: {}", e);
                break;
            }
        };

        //msg.map(|msg| to_event_loop.send(msg));
    }

    to_event_loop.send(EndLobby);
}

fn parse_message(msg: WsMessage) -> Option<PlayerEvent> {
    if !msg.is_text() {
        return None;
    }

    let text = msg.to_str().expect("converting WsMessage to string");

    match serde_json::from_str(text) {
        Ok(msg) => msg,
        Err(e) => {
            eprintln!("Got invalid player message: msg={}, err={}", text, e);
            None
        }
    }
}
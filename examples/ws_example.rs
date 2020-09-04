use futures::{future::ready, StreamExt as FutureStreamExt};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender};
use warp::Filter;
use warp::ws::{Message as WsMessage, WebSocket};

#[tokio::main]
async fn main() {
    let (mut events_tx, mut events_rx) = mpsc::unbounded_channel();
    tokio::task::spawn(event_loop(events_rx));

    let events_tx_filter = warp::any()
        .map(move || events_tx.clone());

    let websocket_route = warp::path("tsurust-ws")
        .and(warp::ws())
        .and(events_tx_filter)
        .map(move |ws: warp::ws::Ws, events_tx: Sender<String>| {
            ws.on_upgrade(|socket| client_connected(socket, events_tx))
        });

    warp::serve(websocket_route)
        .run(([127, 0, 0, 1], 3030)).await;
}

async fn event_loop(mut mpsc_rx: Receiver<String>) {
    println!("TsuRust event loop running!");

    while let Some(event) = mpsc_rx.next().await {
        println!("got {:?}", event);
    }
}

async fn client_connected(ws: WebSocket, mut to_event_loop: Sender<String>) {
    let (_ws_out, mut ws_in) = ws.split();
    println!("connected");

    while let Some(result) = ws_in.next().await { //and_then()?
        match result {
            Ok(msg) => {
                println!("message {:?}", msg);
                if msg.is_text() {
                    to_event_loop.send(msg.to_str().unwrap().to_owned());
                }
            } ,
            Err(e) => break,
        };
    };
}

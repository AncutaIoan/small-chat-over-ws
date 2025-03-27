use std::sync::{Arc, Mutex};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast;
use warp::ws::{Message, WebSocket};

/*
    Simulate new client with wscat -c ws://127.0.0.1:7000/ws
 */
pub async fn handle_connection(ws: WebSocket, tx: Arc<Mutex<broadcast::Sender<String>>>) {
    let (mut ws_sender, mut ws_receiver) = ws.split();
    let mut rx = tx.lock().unwrap().subscribe();

    // Handle sending messages to the WebSocket clients (broadcasting)
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // Ensure the message is only sent to the WebSocket client if it isn't the sender
            if ws_sender.send(Message::text(msg)).await.is_err() {
                break;  // If sending fails, we close the WebSocket connection
            }
        }
    });

    // Handle receiving messages from the WebSocket client
    while let Some(result) = ws_receiver.next().await {
        match result {
            Ok(message) => {
                if let Ok(text) = message.to_str() {
                    // Here we only broadcast the message to other clients, not the sender
                    let _ = tx.lock().unwrap().send(text.to_string());  // Broadcast the message to all clients except the sender
                }
            },
            Err(e) => break,  // If error occurs, break the loop
        }
    }
}

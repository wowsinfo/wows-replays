use std::{net::TcpListener, thread::spawn};
use tungstenite::accept;

fn main() {
    let server = TcpListener::bind("127.0.0.1:8615").unwrap();
    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                match websocket.read() {
                    Ok(msg) => {
                        println!("Received a message from the client: {:?}", msg);
                        // We do not want to send back ping/pong messages.
                        if msg.is_binary() || msg.is_text() {
                            // add a timestamp to the message
                            let msg = format!("{}: {}", chrono::Utc::now(), msg);
                            websocket.send(tungstenite::Message::Text(msg)).unwrap();
                        }
                    },
                    Err(_) => break, // the connection is closed
                }
            }
        });
    }
}

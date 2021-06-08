// use std::thread::spawn;
// use tlserver::{Cab, Direction, server};


use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;
use tungstenite::Message;

use tlserver::*;

fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    println!("Server running");
    for stream in server.incoming() {
        spawn (move || {
            println!("Client connected");
            let mut websocket = accept(stream.unwrap()).unwrap();
            websocket.write_message(Message::Text("{\"type\": \"config\", \"cabs\": {\"Train1\": \"1\", \"Train2\": \"2\"}, \"debug\": \"True\"}".to_string())).unwrap();
            websocket.write_message(Message::Text(r#"{"type": "state", "updateType": "cab", "cab": "1", "speed": 0, "direction": 1, "functions": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}"#.to_string())).unwrap();
            websocket.write_message(Message::Text(r#"{"type": "state", "updateType": "cab", "cab": "2", "speed": 0, "direction": 1, "functions": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}"#.to_string())).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    println!("{}",msg);
                    let msg_str = msg.to_text().unwrap().to_string();
                    let packet_prot: Option<PacketProt> = match parser::parse(msg_str).unwrap() {
                        Parsed::Speed(packet) => Some(update_state::speed(packet)),
                        Parsed::Function(packet) => Some(update_state::function(packet)),
                        Parsed::Power(packet) => Some(update_state::power(packet)),
                    };
                    serial::write_packet(packet_prot.unwrap()).unwrap();
                    // websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}

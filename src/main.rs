// use std::thread::spawn;
// use tlserver::{Cab, Direction, server};


use std::net::TcpListener;
use std::thread::spawn;
use std::sync::{Arc, Mutex};
use tungstenite::server::accept;
use std::collections::HashMap;

use tungstenite::Message;

use tlserver::*;

fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    println!("Server running");
    
    let mut known_cabs: HashMap<String, u32> = HashMap::new();
    known_cabs.insert(String::from("Train1"), 1);
    known_cabs.insert(String::from("Train2"), 2);
    // let known_cabs = known_cabs;
    let track_power = TrackPower::Off;
    let mut cabs: Vec<Cab> = vec!();

    for cab in known_cabs.iter() {
        cabs.push(Cab::new(*cab.1));
    } 
    println!("{:?}", cabs);

    let mut cabs_threads = Arc::new(Mutex::new(cabs));
    let known_cabs_threads = Arc::new(Mutex::new(known_cabs));
    let mut track_power_threads = Arc::new(Mutex::new(track_power));

    for stream in server.incoming() {
        let cabs_threads = Arc::clone(&mut cabs_threads);
        let known_cabs_threads = Arc::clone(&known_cabs_threads);
        let track_power_threads = Arc::clone(&mut track_power_threads);
        spawn ( move || {
            println!("Client connected");
            let mut websocket = accept(stream.unwrap()).unwrap();
            websocket.write_message(Message::Text("{\"type\": \"config\", \"cabs\": {\"Train1\": \"1\", \"Train2\": \"2\"}, \"debug\": \"True\"}".to_string())).unwrap();
            websocket.write_message(Message::Text(r#"{"type": "state", "updateType": "cab", "cab": "1", "speed": 0, "direction": 1, "functions": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}"#.to_string())).unwrap();
            websocket.write_message(Message::Text(r#"{"type": "state", "updateType": "cab", "cab": "2", "speed": 0, "direction": 1, "functions": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}"#.to_string())).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();
                let cabs_threads = Arc::clone(&cabs_threads);
                let known_cabs_threads = Arc::clone(&known_cabs_threads);
                let track_power_threads = Arc::clone(&track_power_threads);

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    println!("{}",msg);
                    let msg_str = msg.to_text().unwrap().to_string();
                    let packet_prot: Option<PacketProt> = match parser::parse(msg_str).unwrap() {
                        Parsed::Speed(packet) => Some(update_state::speed(packet, known_cabs_threads, cabs_threads)),
                        Parsed::Function(packet) => Some(update_state::function(packet, known_cabs_threads, cabs_threads)),
                        Parsed::Power(packet) => Some(update_state::power(packet, track_power_threads)),
                    };
                    serial::write_packet(packet_prot.unwrap()).unwrap();
                    // websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}

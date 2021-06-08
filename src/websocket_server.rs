use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;
use tungstenite::Message;

// struct ConfigPacket {
//     r#type: String,
//     cabs: String,
//     debug: bool,
// }

// impl Default for ConfigPacket {
//     fn default() -> ConfigPacket {
//         ConfigPacket {
//             r#type: "config".to_string(),
//             cabs: "Thingy".to_string(),
//             debug: true,
//         }
//     }
// }

/// A WebSocket echo server
pub fn server () {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            websocket.write_message(Message::Text("{\"type\": \"config\", \"cabs\": {\"Train1\": \"1\", \"Train2\": \"2\"}, \"debug\": \"True\"}".to_string())).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    println!("{}",msg);
                    // websocket.write_message(msg).unwrap();
                }
            }
        });
    }
    println!("Server started");
}
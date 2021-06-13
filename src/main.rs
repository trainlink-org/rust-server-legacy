// use std::thread::spawn;
// use tlserver::{Cab, Direction, server};


use std::net::TcpListener;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, mpsc};
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
    let (tx_serial, rx_serial) = mpsc::channel();
    // let (tx_socket, rx_socket) = mpsc::channel();

    let mut tx_socket_master: Arc<Mutex<Vec<mpsc::Sender<String>>>> = Arc::new(Mutex::new(vec!()));
    let (tx_socket_controller, rx_socket_controller): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
    // let rx_socket_controller = Arc::new(Mutex::new(rx_socket_controller));

    thread::spawn( move || {
        let mut port = serial::open("/dev/ttyACM0").unwrap();
        port.reconfigure(&|settings| {
            settings.set_baud_rate(serial::Baud115200).unwrap();
            Ok(())
        }).unwrap();
        for received in rx_serial {
            println!("{}", received);
            serial_utils::write_packet(received, &mut port).unwrap();
        }
    });

    {
        let tx_socket_master = Arc::clone(&mut tx_socket_master);
        thread::spawn(move || {
            for received in rx_socket_controller {
                println!("Master: packet");
                for tx in tx_socket_master.lock().unwrap().iter(){
                    tx.send(received.clone()).unwrap();
                    println!("Master: send");
                }
            }
        });
    }

    for stream in server.incoming() {
        let tx_serial = mpsc::Sender::clone(&tx_serial);
        let tx_socket_controller = mpsc::Sender::clone(&tx_socket_controller);
        // let tx_socket = mpsc::Sender::clone(&tx_socket);
        // let rx_socket = mpsc::Receiver::clone(&rx_socket);
        // let (tx_socket, rx_socket) = mpsc::channel();
        let cabs_threads = Arc::clone(&mut cabs_threads);
        let known_cabs_threads = Arc::clone(&known_cabs_threads);
        let track_power_threads = Arc::clone(&mut track_power_threads);
        let tx_socket_master = Arc::clone(&mut tx_socket_master);
        let websocket = Arc::new(Mutex::new(accept(stream.unwrap()).unwrap()));
        let (tx_socket, rx_socket) = mpsc::channel();
        thread::spawn( move || {
            let read_websocket = Arc::clone(&websocket);
            println!("Client connected");
            let mut tx_socket_master = tx_socket_master.lock().unwrap();
            &mut tx_socket_master.push(tx_socket);
            thread::spawn(move || {
                {
                    let mut websocket = read_websocket.lock().unwrap();
                    // let tx_socket_master = Arc::clone(&mut tx_socket_master);
                    // let mut websocket = accept(stream.unwrap()).unwrap();
                    websocket.write_message(Message::Text("{\"type\": \"config\", \"cabs\": {\"Train1\": \"1\", \"Train2\": \"2\"}, \"debug\": \"True\"}".to_string())).unwrap();
                    websocket.write_message(Message::Text(r#"{"type": "state", "updateType": "cab", "cab": "1", "speed": 0, "direction": 1, "functions": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}"#.to_string())).unwrap();
                    websocket.write_message(Message::Text(r#"{"type": "state", "updateType": "cab", "cab": "2", "speed": 0, "direction": 1, "functions": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}"#.to_string())).unwrap();
                }
                loop {
                    {
                        let mut websocket = read_websocket.lock().unwrap();
                        println!("Read");
                        let msg = websocket.read_message().unwrap();
                        let cabs_threads = Arc::clone(&cabs_threads);
                        let known_cabs_threads = Arc::clone(&known_cabs_threads);
                        let track_power_threads = Arc::clone(&track_power_threads);
                        let tx_serial = mpsc::Sender::clone(&tx_serial);
                        // let tx_socket = mpsc::Sender::clone(&tx_socket);
                        // let rx_socket = mpsc::Receiver::clone(&rx_serial);

                        // We do not want to send back ping/pong messages.
                        if msg.is_binary() || msg.is_text() {
                            let mut update_packet: String = String::new();
                            println!("{}",msg);
                            let msg_str = msg.to_text().unwrap().to_string();
                            let packet_prot: Option<PacketProt> = match parser::parse(msg_str).unwrap() {
                                Parsed::Speed(packet) => Some(update_state::speed(packet, &mut update_packet, known_cabs_threads, cabs_threads)),
                                Parsed::Function(packet) => Some(update_state::function(packet, &mut update_packet, known_cabs_threads, cabs_threads)),
                                Parsed::Power(packet) => Some(update_state::power(packet, &mut update_packet, track_power_threads)),
                            };
                            serial_utils::send_packet(packet_prot.unwrap(), tx_serial).unwrap();
                            tx_socket_controller.send(update_packet).unwrap();
                            // websocket.write_message(msg).unwrap();
                        }
                    }
                    thread::sleep(Duration::from_millis(10));
                }
            });
            thread::spawn(move || {
                println!("Thread spawned");
                let write_websocket = Arc::clone(&websocket);
                println!("Thread ready");
                // loop {
                //     let mut websocket = write_websocket.lock().unwrap();
                //     websocket.write_message(Message::Text("Hello world".to_string())).unwrap();
                //     println!("Sent");
                // }
                for received in rx_socket {
                    println!("{}", received);
                    {
                        let mut websocket = write_websocket.lock().unwrap();
                        println!("Write");
                        websocket.write_message(Message::Text(received)).unwrap();
                    }
                    // thread::sleep(Duration::from_millis(1));
                }
                println!("End of thread");
            });
        });
    }
}

// use std::thread::spawn;
// use tlserver::{Cab, Direction, server};


// use std::net::TcpListener;
// use std::thread;
// use std::time::Duration;
// use std::sync::{Arc, Mutex, mpsc};
// use tungstenite::server::accept;
// use std::collections::HashMap;

// use tungstenite::Message;

// use tlserver::*;

// fn main() {
//     let server = TcpListener::bind("127.0.0.1:9001").unwrap();
//     println!("Server running");
    
//     let mut known_cabs: HashMap<String, u32> = HashMap::new();
//     known_cabs.insert(String::from("Train1"), 1);
//     known_cabs.insert(String::from("Train2"), 2);
//     // let known_cabs = known_cabs;
//     let track_power = TrackPower::Off;
//     let mut cabs: Vec<Cab> = vec!();

//     for cab in known_cabs.iter() {
//         cabs.push(Cab::new(*cab.1));
//     } 
//     println!("{:?}", cabs);

//     let mut cabs_threads = Arc::new(Mutex::new(cabs));
//     let known_cabs_threads = Arc::new(Mutex::new(known_cabs));
//     let mut track_power_threads = Arc::new(Mutex::new(track_power));
//     let (tx_serial, rx_serial) = mpsc::channel();
//     // let (tx_socket, rx_socket) = mpsc::channel();

//     let mut tx_socket_master: Arc<Mutex<Vec<mpsc::Sender<String>>>> = Arc::new(Mutex::new(vec!()));
//     let (tx_socket_controller, rx_socket_controller): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
//     // let rx_socket_controller = Arc::new(Mutex::new(rx_socket_controller));

//     thread::spawn( move || {
//         let mut port = serial::open("/dev/ttyACM0").unwrap();
//         port.reconfigure(&|settings| {
//             settings.set_baud_rate(serial::Baud115200).unwrap();
//             Ok(())
//         }).unwrap();
//         for received in rx_serial {
//             println!("{}", received);
//             serial_utils::write_packet(received, &mut port).unwrap();
//         }
//     });

//     {
//         let tx_socket_master = Arc::clone(&mut tx_socket_master);
//         thread::spawn(move || {
//             for received in rx_socket_controller {
//                 println!("Master: packet");
//                 for tx in tx_socket_master.lock().unwrap().iter(){
//                     tx.send(received.clone()).unwrap();
//                     println!("Master: send");
//                 }
//             }
//         });
//     }

//     for stream in server.incoming() {
//         let tx_serial = mpsc::Sender::clone(&tx_serial);
//         let tx_socket_controller = mpsc::Sender::clone(&tx_socket_controller);
//         // let tx_socket = mpsc::Sender::clone(&tx_socket);
//         // let rx_socket = mpsc::Receiver::clone(&rx_socket);
//         // let (tx_socket, rx_socket) = mpsc::channel();
//         let cabs_threads = Arc::clone(&mut cabs_threads);
//         let known_cabs_threads = Arc::clone(&known_cabs_threads);
//         let track_power_threads = Arc::clone(&mut track_power_threads);
//         let tx_socket_master = Arc::clone(&mut tx_socket_master);
//         let websocket = Arc::new(Mutex::new(accept(stream.unwrap()).unwrap()));
//         let (tx_socket, rx_socket) = mpsc::channel();
//         thread::spawn( move || {
//             let read_websocket = Arc::clone(&websocket);
//             println!("Client connected");
//             let mut tx_socket_master = tx_socket_master.lock().unwrap();
//             &mut tx_socket_master.push(tx_socket);
//             thread::spawn(move || {
//                 {
//                     let mut websocket = read_websocket.lock().unwrap();
//                     // let tx_socket_master = Arc::clone(&mut tx_socket_master);
//                     // let mut websocket = accept(stream.unwrap()).unwrap();
//                     websocket.write_message(Message::Text("{\"type\": \"config\", \"cabs\": {\"Train1\": \"1\", \"Train2\": \"2\"}, \"debug\": \"True\"}".to_string())).unwrap();
//                     websocket.write_message(Message::Text(r#"{"type": "state", "updateType": "cab", "cab": "1", "speed": 0, "direction": 1, "functions": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}"#.to_string())).unwrap();
//                     websocket.write_message(Message::Text(r#"{"type": "state", "updateType": "cab", "cab": "2", "speed": 0, "direction": 1, "functions": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}"#.to_string())).unwrap();
//                 }
//                 loop {
//                     {
//                         let mut websocket = read_websocket.lock().unwrap();
//                         println!("Read");
//                         let msg = websocket.read_message().unwrap();
//                         let cabs_threads = Arc::clone(&cabs_threads);
//                         let known_cabs_threads = Arc::clone(&known_cabs_threads);
//                         let track_power_threads = Arc::clone(&track_power_threads);
//                         let tx_serial = mpsc::Sender::clone(&tx_serial);
//                         // let tx_socket = mpsc::Sender::clone(&tx_socket);
//                         // let rx_socket = mpsc::Receiver::clone(&rx_serial);

//                         // We do not want to send back ping/pong messages.
//                         if msg.is_binary() || msg.is_text() {
//                             let mut update_packet: String = String::new();
//                             println!("{}",msg);
//                             let msg_str = msg.to_text().unwrap().to_string();
//                             let packet_prot: Option<PacketProt> = match parser::parse(msg_str).unwrap() {
//                                 Parsed::Speed(packet) => Some(update_state::speed(packet, &mut update_packet, known_cabs_threads, cabs_threads)),
//                                 Parsed::Function(packet) => Some(update_state::function(packet, &mut update_packet, known_cabs_threads, cabs_threads)),
//                                 Parsed::Power(packet) => Some(update_state::power(packet, &mut update_packet, track_power_threads)),
//                             };
//                             serial_utils::send_packet(packet_prot.unwrap(), tx_serial).unwrap();
//                             tx_socket_controller.send(update_packet).unwrap();
//                             // websocket.write_message(msg).unwrap();
//                         }
//                     }
//                     thread::sleep(Duration::from_millis(10));
//                 }
//             });
//             thread::spawn(move || {
//                 println!("Thread spawned");
//                 let write_websocket = Arc::clone(&websocket);
//                 println!("Thread ready");
//                 // loop {
//                 //     let mut websocket = write_websocket.lock().unwrap();
//                 //     websocket.write_message(Message::Text("Hello world".to_string())).unwrap();
//                 //     println!("Sent");
//                 // }
//                 for received in rx_socket {
//                     println!("{}", received);
//                     {
//                         let mut websocket = write_websocket.lock().unwrap();
//                         println!("Write");
//                         websocket.write_message(Message::Text(received)).unwrap();
//                     }
//                     // thread::sleep(Duration::from_millis(1));
//                 }
//                 println!("End of thread");
//             });
//         });
//     }
// }
// use tlserver::*;

// use tokio::net::{TcpListener, TcpStream};

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     Ok(())
// }
/*
use std::{env, io::Error};

use futures_util::StreamExt;
use log::info;
use tokio::net::{TcpListener, TcpStream};
use env_logger::*;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};

// // #[tokio::main]
// // async fn main() -> Result<(), Error> {

// fn main() {
//     println!("Hello world");
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Running");
    let _ = env_logger::try_init();
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream.peer_addr().expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (write, read) = ws_stream.split();
    read.forward(write).await.expect("Failed to forward message")
}*/

use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex, mpsc},
};

use tlserver::*;

use futures_channel::mpsc::{unbounded, UnboundedSender, UnboundedReceiver};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;

use tokio::runtime::Runtime;



use std::thread;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr,mut port: Arc<Mutex<serial::SystemPort>>, /*tx_serial: mpsc::Sender<String>,*/ mut cabs_threads: Arc<Mutex<Vec<Cab>>>, known_cabs_threads: Arc<Mutex<HashMap<String, u32>>>, mut track_power_threads: Arc<Mutex<TrackPower>> ) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();
    &tx.unbounded_send(Message::Text("{\"type\": \"config\", \"cabs\": {\"Train1\": \"1\", \"Train2\": \"2\"}, \"debug\": \"True\"}".to_string())).unwrap();
    &tx.unbounded_send(Message::Text(r#"{"type": "state", "updateType": "cab", "cab": "1", "speed": 0, "direction": 1, "functions": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}"#.to_string())).unwrap();
    &tx.unbounded_send(Message::Text(r#"{"type": "state", "updateType": "cab", "cab": "2", "speed": 0, "direction": 1, "functions": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}"#.to_string())).unwrap();
    peer_map.lock().unwrap().insert(addr, tx);

    let (outgoing, incoming) = ws_stream.split();


    let broadcast_incoming = incoming.try_for_each(|msg| {
        println!("Received a message from {}: {}", addr, msg.to_text().unwrap());
        
//        let tx_serial = mpsc::Sender::clone(&tx_serial);
        let cabs_threads = Arc::clone(&mut cabs_threads);
        let known_cabs_threads = Arc::clone(&known_cabs_threads);
        let track_power_threads = Arc::clone(&mut track_power_threads);
        let port = Arc::clone(&mut port);

        let mut update_packet: String = String::new();
        println!("{}",msg);
        let msg_str = msg.to_text().unwrap().to_string();
        let packet_prot: Option<PacketProt> = match parser::parse(msg_str).unwrap() {
            Parsed::Speed(packet) => Some(update_state::speed(packet, &mut update_packet, known_cabs_threads, cabs_threads)),
            Parsed::Function(packet) => Some(update_state::function(packet, &mut update_packet, known_cabs_threads, cabs_threads)),
            Parsed::Power(packet) => Some(update_state::power(packet, &mut update_packet, track_power_threads)),
        };
//        serial_utils::send_packet(packet_prot.unwrap(), tx_serial).unwrap();
        let  port = port.lock().unwrap();
        serial_utils::write_packet(packet_prot.unwrap(),port).unwrap();
        let update_msg = Message::Text(update_packet);
        // tx_socket_controller.send(update_packet).unwrap();
        // websocket.write_message(msg).unwrap();

        let peers = peer_map.lock().unwrap();

        // We want to broadcast the message to everyone except ourselves.
        let broadcast_recipients = peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);

        for recp in broadcast_recipients {
            recp.unbounded_send(update_msg.clone()).unwrap();
        }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    
    let mut known_cabs: HashMap<String, u32> = HashMap::new();
    known_cabs.insert(String::from("Train1"), 1);
    known_cabs.insert(String::from("Train2"), 2);

    let track_power = TrackPower::Off;
    let mut cabs: Vec<Cab> = vec!();

    for cab in known_cabs.iter() {
        cabs.push(Cab::new(*cab.1));
    } 
    println!("{:?}", cabs);

    let mut cabs_threads = Arc::new(Mutex::new(cabs));
    let known_cabs_threads = Arc::new(Mutex::new(known_cabs));
    let mut track_power_threads = Arc::new(Mutex::new(track_power));

//    let (tx_serial, rx_serial) = mpsc::channel();
    // let (tx_serial, rx_serial): (UnboundedSender<String>, UnboundedReceiver<String>) = unbounded();
    /*thread::spawn( move || {
        let mut port = serial::open("/dev/ttyACM0").unwrap();
        port.reconfigure(&|settings| {
            settings.set_baud_rate(serial::Baud115200).unwrap();
            Ok(())
        }).unwrap();
        //let recieved = rx_serial.map(Ok);
        //println!("{:?}", recieved);
        /*for received in rx_serial.iter() {
            println!("{}", received);
            serial_utils::write_packet(received, &mut port).unwrap();
        }*/
        let mut loop_cont = Some("None");
        while loop_cont != None {
            loop_cont = match rx_socket.try_next() {
                Ok(Some(String)) => println!("Some"),
                Ok(None) => println!("None")
            }
        }
    });*/

    let mut port = serial::open("/dev/ttyACM0").unwrap();
    let mut port = Arc::new(Mutex::new(port));

    let addr = env::args().nth(1).unwrap_or_else(|| "0.0.0.0:6789".to_string());

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        //let tx_serial = mpsc::Sender::clone(&tx_serial);
        let cabs_threads = Arc::clone(&mut cabs_threads);
        let known_cabs_threads = Arc::clone(&known_cabs_threads);
        let track_power_threads = Arc::clone(&mut track_power_threads);
        let port = Arc::clone(&mut port);
        tokio::spawn(handle_connection(state.clone(), stream, addr, port, /*tx_serial,*/ cabs_threads, known_cabs_threads, track_power_threads));
    }

    Ok(())
}

// use mini_redis::{client, Result};

// #[tokio::main]
// pub async fn main() -> Result<()> {
//     // Open a connection to the mini-redis address.
//     let mut client = client::connect("127.0.0.1:6379").await?;

//     // Set the key "hello" with value "world"
//     client.set("hello", "world".into()).await?;

//     // Get key "hello"
//     let result = client.get("hello").await?;

//     println!("got value from the server; result={:?}", result);

//     Ok(())
// }

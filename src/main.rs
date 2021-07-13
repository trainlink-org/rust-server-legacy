use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use tlserver::*;

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

#[tokio::main]
async fn main() -> Result<(), IoError> {
    
    // Used to store the cabs that are defined in the config file
    let mut known_cabs: HashMap<String, u32> = HashMap::new();

    // Hardcoding - remove and replace with config loading!
    known_cabs.insert(String::from("Train1"), 1);
    known_cabs.insert(String::from("Train2"), 2);
    // ----------------------------------------------------

    // Stores the state of the track and all cabs
    let track_power = TrackPower::Off;
    let mut cabs: Vec<Cab> = vec!();

    // Creates a cab for each known cab
    for cab in known_cabs.iter() {
        cabs.push(Cab::new(*cab.1));
    } 
    // println!("{:?}", cabs);

    // Creates the mutexes that allow the threads to access central values
    let known_cabs_threads = Arc::new(Mutex::new(known_cabs));
    let mut track_power_threads = Arc::new(Mutex::new(track_power));
    let mut cabs_threads = Arc::new(Mutex::new(cabs));

    // Opens the serial port and creates a mutex for it
    let port = serial::open("/dev/ttyACM0").unwrap();
    let mut port = Arc::new(Mutex::new(port));

    // Sets the servers IP address and port
    let addr = env::args().nth(1).unwrap_or_else(|| "0.0.0.0:6789".to_string());

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener to accept connections on
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Spawn the handling of each connection in a separate task
    while let Ok((stream, addr)) = listener.accept().await {
        // Create a separate copy of the mutex handler for each task
        let cabs_threads = Arc::clone(&mut cabs_threads);
        let known_cabs_threads = Arc::clone(&known_cabs_threads);
        let track_power_threads = Arc::clone(&mut track_power_threads);
        let port = Arc::clone(&mut port);
        // Create the task
        tokio::spawn(handle_connection(state.clone(), stream, addr, port, cabs_threads, known_cabs_threads, track_power_threads));
    }

    Ok(())
}

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr,mut port: Arc<Mutex<serial::SystemPort>>, /*tx_serial: mpsc::Sender<String>,*/ mut cabs_threads: Arc<Mutex<Vec<Cab>>>, known_cabs_threads: Arc<Mutex<HashMap<String, u32>>>, mut track_power_threads: Arc<Mutex<TrackPower>> ) {
    println!("Incoming TCP connection from: {}", addr);

    // Accept connection and report to console()
    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // Create peer
    let (tx, rx) = unbounded();
    
    // These are hardcoded values, need to be changed to send the actual current value
    &tx.unbounded_send(Message::Text(r#"{"type": "config", "cabs": {"Train1": "1", "Train2", "2"}, "debug": "True"}"#.to_string())).unwrap();
    &tx.unbounded_send(Message::Text(r#"{"type": "state", "updateType": "cab", "cab": "1", "speed": 0, "direction": 1, "functions": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}"#.to_string())).unwrap();
    &tx.unbounded_send(Message::Text(r#"{"type": "state", "updateType": "cab", "cab": "2", "speed": 0, "direction": 1, "functions": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}"#.to_string())).unwrap();
    // -------------------------------------------------------------------------------
    
    // Insert the write part of this peer to the peer map.
    peer_map.lock().unwrap().insert(addr, tx);


    // Split websocket stream
    let (outgoing, incoming) = ws_stream.split();


    // Run closure for each incoming message
    let broadcast_incoming = incoming.try_for_each(|msg| {
        println!("Received a message from {}: {}", addr, msg.to_text().unwrap());
        
        // Clone a copy of the mutex handlers
        let cabs_threads = Arc::clone(&mut cabs_threads);
        let known_cabs_threads = Arc::clone(&known_cabs_threads);
        let track_power_threads = Arc::clone(&mut track_power_threads);
        let port = Arc::clone(&mut port);

        // Create a blank string for the update packet
        let mut update_packet: String = String::new();
        // Convert the message to a string
        let msg_str = msg.to_text().unwrap().to_string();
        // Parse packet to a PacketProt
        let packet_prot: Option<PacketProt> = match parser::parse(msg_str).unwrap() {
            Parsed::Speed(packet) => Some(update_state::speed(packet, &mut update_packet, known_cabs_threads, cabs_threads)),
            Parsed::Function(packet) => Some(update_state::function(packet, &mut update_packet, known_cabs_threads, cabs_threads)),
            Parsed::Power(packet) => Some(update_state::power(packet, &mut update_packet, track_power_threads)),
        };
//        serial_utils::send_packet(packet_prot.unwrap(), tx_serial).unwrap();

        // Obtain serial port lock
        let port = port.lock().unwrap();
        // Write packet to serial
        if let Err(error) = serial_utils::write_packet(packet_prot.unwrap(),port) {
            println!("{}", error);
            return future::ok(());
        }

        // Obtain peer map lock
        let peers = peer_map.lock().unwrap();

        // Broadcast the message to everyone except ourselves.
        let update_msg = Message::Text(update_packet);
        let broadcast_recipients = peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);

        for recp in broadcast_recipients {
            if let Err(error) = recp.unbounded_send(update_msg.clone()) {
                println!("{}", error);
                return future::ok(());
            }
        }

        future::ok(())
    });

    // Handle update packets
    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    // Handle device disconnecting
    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}
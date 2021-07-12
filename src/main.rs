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

    let port = serial::open("/dev/ttyACM0").unwrap();
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
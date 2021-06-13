use std::collections::HashMap;
use crate::parser::*;
use crate::packet_gen::*;
use crate::{Cab, Direction, FnState, TrackPower};
use std::sync::{Arc, Mutex};

pub fn speed(msg: SpeedMsg, update_packet: &mut String, known_cabs: Arc<Mutex<HashMap<String, u32>>>, cabs_mutex: Arc<Mutex<Vec<Cab>>>) -> PacketProt {
    let mut cabs = cabs_mutex.lock().unwrap();
    let known_cabs = known_cabs.lock().unwrap();

    let id = known_cabs.get(&msg.address);

    
    let id: u32 = match id {
        Some(i) => *i,
        None => msg.address.parse().unwrap(),
    };

    let mut cab: Vec<&mut Cab> = cabs.iter_mut().filter(|c| c.id == id).collect();
    let mut cab = &mut cab[0];

    cab.speed = msg.speed;
    let speed: i8;

    let direction = match msg.direction {
        Direction::Forward => {speed = msg.speed as i8;1},
        Direction::Reverse => {speed = msg.speed as i8;0},
        Direction::Stopped => {speed = 0;0},
        Direction::Estop => {speed = -1;0}, //Needs to be changed
    };

    cab.direction = msg.direction;
    // println!("Cabs: {:?}", cabs);

    let mut update_packet_temp = format!(r#""type": "state", "updateType": "cab", "cab": "{}", "speed": "{}", "direction": {}, "functions": {:?} "#, id, speed, direction, cab.functions);
    update_packet_temp = format!("{{{}}}", update_packet_temp);
    *update_packet = update_packet_temp;

    PacketProt::new("t 1".to_string(), 
                    Some(id as i32), 
                    Some(speed as i32), 
                    Some(direction))
                    .unwrap()
}

pub fn function(msg: FnMsg, update_packet: &mut String, known_cabs: Arc<Mutex<HashMap<String, u32>>>, cabs_mutex: Arc<Mutex<Vec<Cab>>>) -> PacketProt {
    let mut cabs = cabs_mutex.lock().unwrap();
    let known_cabs = known_cabs.lock().unwrap();

    let id = known_cabs.get(&msg.address);

    let id: u32 = match id {
        Some(i) => *i,
        None => msg.address.parse().unwrap(),
    };

    let mut cab: Vec<&mut Cab> = cabs.iter_mut().filter(|c| c.id == id).collect();
    let cab = &mut cab[0];

    let index  = msg.func_num;
    let last_state = cab.get_function(index.into()).unwrap();
    
    let state = match msg.state {
        FnState::On => 1,
        FnState::Off => 0,
        FnState::Toggle => !last_state as i32,
    };


    cab.set_function(index.into(), state != 0 ).unwrap();

    println!("Cabs: {:?}", cabs);

    PacketProt::new("T".to_string(), Some(id as i32), Some(msg.func_num as i32), Some(state)).unwrap()

}

pub fn power(msg: PowerMsg, update_packet: &mut String, track_power: Arc<Mutex<TrackPower>>) -> PacketProt {
    let mut track_power = track_power.lock().unwrap();

    let state = match msg.state {
        TrackPower::Powered => 1,
        TrackPower::Off => 0,
    };

    *track_power = msg.state;

    println!("{:?}", track_power);

    PacketProt::new(format!("{}", state), None, None, None ).unwrap()
}
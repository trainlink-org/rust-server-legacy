// pub mod parser{
use serde_json::{Value};
use std::error::Error;
use crate::{Direction, FnState, TrackPower};

pub fn parse(input: String) -> Result<Parsed, Box<dyn Error>> {
    let v: Value;

    v = serde_json::from_str(&input[..])?;
    
    // println!("{}", &v["class"].to_string()[..]);
    let result = match &v["class"].to_string()[..] {
        r#""cabControl""# => parse_speed(v), 
        r#""cabFunction""# => parse_function(v),
        r#""power""# => parse_power(v),
        _ => Err("Error".to_string()) 
    };
    
    Ok(result.unwrap())
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Parsed {
    Speed(SpeedMsg),
    Function(FnMsg),
    Power(PowerMsg),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct SpeedMsg{
    pub address: String,
    pub speed: u8,
    pub direction: Direction,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct FnMsg {
    pub address: String,
    pub func_num: u8,
    pub state: FnState,
}

#[derive(Debug, PartialEq)]
pub struct PowerMsg {
    pub state: TrackPower,
}

fn parse_speed(msg: Value) -> Result<Parsed, String> {
    let return_msg: SpeedMsg;
    let address = msg["cabAddress"].to_string();
    let len = address.len();
    let address = address[1..len-1].to_string();
    if msg["action"] == "setSpeed" {
        let direction = match &msg["cabDirection"].to_string()[..] {
            "1" => Direction::Forward,
            "0" => Direction::Reverse,
            _ => Direction::Stopped,
        };
        let speed = msg["cabSpeed"].to_string();
        let len = speed.len();
        let speed: u8 = speed[1..len-1].parse().unwrap();
        return_msg = SpeedMsg{
            address,
            speed,
            direction,
        };
    } else if msg["action"] == "stop" {
        return_msg = SpeedMsg{
            address,
            speed: 0,
            direction: Direction::Stopped,
        }
    } else if msg["action"] == "estop" {
        return_msg = SpeedMsg{
            address,
            speed: 0,
            direction: Direction::Estop,
        }
    } else {
        return Err("Invalid packet".to_string());
    }
    // println!("{:?}", return_msg);
    Ok(Parsed::Speed(return_msg))
}

fn parse_function(msg: Value) -> Result<Parsed, String> {
    let address = msg["cab"].to_string();
    let len = address.len();
    let address = address[1..len-1].to_string();
    let state = match msg["state"].to_string().parse().unwrap() {
        1 => FnState::On,
        0 => FnState::Off,
        -1 => FnState::Toggle,
        _ => FnState::Off,
    };
    let return_msg = FnMsg{
        address,
        func_num: msg["func"].to_string().parse().unwrap(),
        state,
    };
    println!("{:?}", return_msg);
    Ok(Parsed::Function(return_msg))
}

fn parse_power(msg: Value) -> Result<Parsed, String> {
    let state: u8 = msg["state"].to_string().parse().unwrap();
    let state = match state {
        1 => TrackPower::Powered,
        0 => TrackPower::Off,
        _ => TrackPower::Off,
    };
    let return_msg = PowerMsg{state};
    println!("{:?}", return_msg);
    Ok(Parsed::Power(return_msg))
}

#[cfg(test)]
mod tests{
    use super::*;
    // use tungstenite::Message;

    #[test]
    fn test_power() {
        assert_eq!(parse_power(serde_json::from_str(r#"{"class":"power","state":0}"#).unwrap()).unwrap(), Parsed::Power(PowerMsg{state: TrackPower::Off}));
    }
}
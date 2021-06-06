pub fn parse(input: String) -> Parsed {
    Parsed::Err("Not implemented yet".to_string())
}

pub enum Parsed {
    Speed(SpeedMsg),
    Function(FnMsg),
    Power(PowerMsg),
    Err(String),
}

#[derive(Debug)]
pub struct SpeedMsg{

}

pub struct FnMsg {

}

pub struct PowerMsg {

}
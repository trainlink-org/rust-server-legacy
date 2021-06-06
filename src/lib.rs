mod config;
mod parser;
mod update_state;
mod packet_gen;
mod serial;

pub use crate::config::*;
pub use crate::parser::*;
pub use crate::update_state::*;
pub use crate::packet_gen::*;
pub use crate::serial::*;


pub struct Cab {
    id: u32,
    speed: u32,
    pub direction: Direction,
    // pub functions: Functions,
    pub functions: [bool; 28],
}

impl Cab {
    /// Creates a Cab with the default values
    pub fn new(id: u32) -> Cab {
        Cab{
            id,
            speed: 0,
            direction: Direction::Stopped,
            functions: [false; 28],
        }
    }

    /// Gets the cab's ID (address)
    pub fn get_id(&self) -> u32 {
        self.id
    }

    /// Sets the speed of the cab
    pub fn set_speed(&mut self, new_speed: u32) -> Result<(), String>{
        if new_speed <= 126{
            self.speed = new_speed;
            return Ok(());
        }
        Err("New speed greater than 126".to_string())
    }

    /// Gets the speed of the cab
    pub fn get_speed(&self) -> u32 {
        self.speed
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Direction {
    Forward,
    Stopped,
    Reverse,
    Estop,
}

pub enum TrackPower {
    Powered,
    Off,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cab() {
        let cab = Cab::new(1);
        assert_eq!(cab.id, 1);
        assert_eq!(cab.direction, Direction::Stopped);
        assert_eq!(cab.functions, [false; 28]);
        assert_eq!(cab.speed, 0);
    }

    #[test]
    fn set_cab() {
        let mut cab = Cab::new(1);
        assert_eq!(cab.get_id(), 1);
        cab.set_speed(100).unwrap();
        assert_eq!(cab.get_speed(), 100);
        assert_eq!(cab.set_speed(127), Err("New speed greater than 126".to_string()));
    }

    #[test]
    fn set_function() {
        let mut cab = Cab::new(1);
        cab.functions[0] = true;
        let mut expected = [false; 28];
        expected[0] = true;
        assert_eq!(cab.functions, expected);
    }
}
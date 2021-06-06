
use tlserver::{Cab, Direction};

fn main() {
    let mut new_cab = Cab::new(0);

    new_cab.set_speed(126).unwrap();

    println!("{}",new_cab.get_speed());
    println!("{:?}",new_cab.direction);
    new_cab.direction = Direction::Forward;
    println!("{:?}",new_cab.direction);

}

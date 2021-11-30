extern crate serial;

use serial::prelude::*;
use std::error::Error;

use crate::packet_gen::*;
use std::sync::mpsc;

pub fn send_packet(packet: PacketProt, tx: mpsc::Sender<String>) -> Result<(), String> {
    let serial_packet = packet.generate().unwrap();
    tx.send(serial_packet).unwrap();
    Ok(())
}

/*pub fn write_packet<T: SerialPort>(packet: String, port: &mut T) -> Result<(), String> {
    port.write(&packet.into_bytes()[..]).unwrap();
    Ok(())
}*/
// pub fn write_packet<T: SerialPort>(packet: PacketProt, mut port: T) -> Result<(), String> {
pub fn write_packet<T: SerialPort>(packet: PacketProt, mut port: std::sync::MutexGuard<Option<T>>) -> Result<(), Box<dyn Error>> {
    let packet = packet.generate()?;
    match *port {
        Some(ref mut port) => {
            port.write(&packet.into_bytes()[..])?;
            return Ok(());
        }
        None => return Ok(()),
    }
    // port.write(&packet.into_bytes()[..])?;
}

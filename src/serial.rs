use crate::packet_gen::*;
pub fn write_packet(packet: PacketProt) -> Result<(), String> {
    let serial_packet = packet.generate().unwrap();
    Ok(())
}
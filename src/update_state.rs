// pub mod update_state{
    use crate::parser::*;
    use crate::packet_gen::*;
    pub fn speed(msg: SpeedMsg) -> PacketProt {
        PacketProt{..Default::default()} 
    }

    pub fn function(msg: FnMsg) -> PacketProt {
        PacketProt{..Default::default()} 
    }

    pub fn power(msg: PowerMsg) -> PacketProt {
        PacketProt{..Default::default()} 
    }
// }
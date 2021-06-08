pub struct PacketProt{
    pub command: char,
    pub param1: Option<u32>,
    pub param2: Option<u32>,
    pub param3: Option<u32>,
}

impl PacketProt{
    pub fn new(command: char, param1: Option<u32>, param2: Option<u32>, param3: Option<u32>) -> Result<PacketProt, String>{
        // More checks later, for now only returns input in struct
        Ok(PacketProt{
            command,
            param1,
            param2,
            param3,
        })
    }

    pub fn generate(self) -> Result<String, String> {
        Err("Not implemented yet".to_string())
    }
}

impl Default for PacketProt {
    fn default() -> PacketProt {
        PacketProt {
            command: '0',
            param1: None,
            param2: None,
            param3: None,
        }
    }
}
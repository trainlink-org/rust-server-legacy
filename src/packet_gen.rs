pub struct PacketProt{
    command: char,
    param1: Option<u32>,
    param2: Option<u32>,
    param3: Option<u32>,
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
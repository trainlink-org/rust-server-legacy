#[derive(Debug)]
pub struct PacketProt{
    pub command: String,
    pub param1: Option<i32>,
    pub param2: Option<i32>,
    pub param3: Option<i32>,
}

impl PacketProt{
    pub fn new(command: String, param1: Option<i32>, param2: Option<i32>, param3: Option<i32>) -> Result<PacketProt, String>{
        // More checks later, for now only returns input in struct
        Ok(PacketProt{
            command,
            param1,
            param2,
            param3,
        })
    }

    pub fn generate(self) -> Result<String, String> {
        let param1 = match self.param1 {
            Some(p) => p.to_string(),
            None => "".to_string(),
        };

        let param2 = match self.param2 {
            Some(p) => p.to_string(),
            None => "".to_string(),
        };

        let param3 = match self.param3 {
            Some(p) => p.to_string(),
            None => "".to_string(),
        };
        Ok(format!("<{} {} {} {}>", self.command, param1, param2, param3))
    }
}

impl Default for PacketProt {
    fn default() -> PacketProt {
        PacketProt {
            command: "0".to_string(),
            param1: None,
            param2: None,
            param3: None,
        }
    }
}
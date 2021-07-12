use std::io;
// this is a test

pub struct Config {

}

impl Config {
    pub fn load(_filename: String) -> Result<Config, io::Error> { // Need to remove underscore before filename
        Ok(Config{})
    }
}

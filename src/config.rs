use std::io;

pub struct Config {

}

impl Config {
    pub fn load(filename: String) -> Result<Config, io::Error> {
        Ok(Config{})
    }
}
use std::{error::Error, fs::File, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub LISTENER_PORT: String,
    pub BACKEND_PORTS: Vec<String>,
    pub ACCESS_CONTROL_LIST: Vec<String>,
}

impl Config {
    pub fn read_from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }
}

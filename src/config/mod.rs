use std::{error::Error, fs::File, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub listener_port: String,
    pub backend_ports: Vec<String>,
    pub access_control_list: Vec<String>,
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

use serde::Deserialize;

use std::{fs::File, io::Read};

#[derive(Debug, Deserialize)]
pub struct Server {
    port: String,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    level: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: Server,
    pub log: Log,
}

impl Server {
    pub fn get_addr(&self) -> String {
        let addr = format!("0.0.0.0:{}", &self.port);
        return addr;
    }
}

impl Log {
    pub fn get_level(&self) -> &str {
        &self.level
    }
}

impl Settings {
    pub fn new() -> Self {
        let mut file = File::open("conf/default.toml").expect("config not found!");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("config read to string error!");

        toml::from_str(&contents).expect("Config parse Error!")
    }
}

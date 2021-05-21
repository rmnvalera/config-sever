use config::{Config, ConfigError, File};

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
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();

        let config_file = File::with_name("conf/default");
        s.merge(config_file).expect("!!Config not found!!");
        s.try_into()
    }
}

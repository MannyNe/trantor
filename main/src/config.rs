use domain::{serde::Deserialize, thiserror};

#[derive(Deserialize)]
#[serde(crate = "domain::serde")]
pub(crate) struct Config {
    address: std::net::SocketAddr,
    database: String,
    maxminddb: String,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum ConfigError {
    #[error("couldn't read config file: {0}")]
    ReadConfigFile(#[from] std::io::Error),
    #[error("couldn't parse config file: {0}")]
    ParseConfigFile(#[from] toml::de::Error),
    #[error("database path is empty")]
    EmptyDatabasePath,
    #[error("maxminddb path is empty")]
    EmptyMaxminddbPath,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let config = std::fs::read_to_string(path)?;
        let config = toml::from_str::<Self>(&config)?;

        if config.database.is_empty() {
            return Err(ConfigError::EmptyDatabasePath);
        } else if config.maxminddb.is_empty() {
            return Err(ConfigError::EmptyMaxminddbPath);
        }

        Ok(config)
    }

    pub fn database_url(&self) -> &str {
        &self.database
    }

    pub fn address(&self) -> std::net::SocketAddr {
        self.address
    }

    pub fn maxminddb_path(&self) -> &str {
        &self.maxminddb
    }
}

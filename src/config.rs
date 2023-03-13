use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};

pub fn parse(filepath: String) -> Result<Config, String> {
    let mut file = File::open(filepath.as_str()).expect("error opening file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("error reading file");

    let parsed_config: Config = serde_yaml::from_str(&contents).expect("error parsing file");

    Ok(parsed_config)
}

/// The kickable configuration
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// The kickable items
    pub items: Option<Vec<String>>,

    /// The preferred i18n language
    pub lang: Option<String>,

    /// The logging configuration
    pub logging: Option<Logging>,

    /// The server configuration
    pub server: Option<Server>,

    /// The client configuration
    pub client: Option<Client>,
}

/// The logging configuration
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Logging {
    /// The logging level
    pub level: u8,

    /// The logging file
    pub file: String,
}

/// The client configuration
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Client {
    /// The server port
    pub port: u16,

    /// The server address
    pub addr: String,
}

/// The server configuration
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Server {
    /// The server port
    pub port: u16,

    /// The server address
    pub addr: String,

    /// The server TLS configuration
    pub tls: Option<TLS>,
}

/// The server TLS configuration
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TLS {
    /// The TLS certificate
    pub cert: String,

    /// The TLS key
    pub key: String,
}

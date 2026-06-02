use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};

pub fn parse(filepath: String) -> Result<Config, String> {
    let mut file =
        File::open(filepath.as_str()).unwrap_or_else(|_| panic!("error opening file - {filepath}"));

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .unwrap_or_else(|_| panic!("error reading file - {filepath}"));

    let parsed_config: Config = serde_yaml::from_str(&contents)
        .unwrap_or_else(|_| panic!("error parsing file - {filepath}"));

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_reads_kickable_yaml() {
        let cfg = parse("kickable.yaml".to_string()).unwrap();

        let items = cfg.items.expect("items should be present");
        assert!(items.contains(&"it".to_string()));

        assert_eq!(cfg.lang.as_deref(), Some("en-US"));

        let server = cfg.server.expect("server should be present");
        assert_eq!(server.addr, "0.0.0.0");
        assert_eq!(server.port, 8080);

        let client = cfg.client.expect("client should be present");
        assert_eq!(client.addr, "0.0.0.0");
        assert_eq!(client.port, 8080);

        let logging = cfg.logging.expect("logging should be present");
        assert_eq!(logging.level, 1);
    }

    #[test]
    fn deserializes_minimal_config() {
        let cfg: Config = serde_yaml::from_str("items:\n  - it\n").unwrap();

        assert_eq!(cfg.items.unwrap(), vec!["it".to_string()]);
        assert!(cfg.lang.is_none());
        assert!(cfg.server.is_none());
        assert!(cfg.client.is_none());
        assert!(cfg.logging.is_none());
    }

    #[test]
    fn deserializes_empty_config() {
        let cfg: Config = serde_yaml::from_str("{}").unwrap();

        assert!(cfg.items.is_none());
        assert!(cfg.server.is_none());
    }

    #[test]
    fn deserializes_server_tls() {
        let yaml = "server:\n  addr: 127.0.0.1\n  port: 9000\n  tls:\n    cert: /tmp/c.pem\n    key: /tmp/k.pem\n";
        let cfg: Config = serde_yaml::from_str(yaml).unwrap();

        let server = cfg.server.expect("server");
        assert_eq!(server.port, 9000);
        let tls = server.tls.expect("tls");
        assert_eq!(tls.cert, "/tmp/c.pem");
        assert_eq!(tls.key, "/tmp/k.pem");
    }

    #[test]
    fn config_equality() {
        let a: Config = serde_yaml::from_str("items:\n  - it\n").unwrap();
        let b: Config = serde_yaml::from_str("items:\n  - it\n").unwrap();
        assert_eq!(a, b);
    }
}

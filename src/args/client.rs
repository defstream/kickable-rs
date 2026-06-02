use anyhow::Error;

use clap::CommandFactory;
use clap::{ArgGroup, Parser};

use crate::config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(ArgGroup::new("client")))]
pub struct ClientArgs {
    /// The item to check for kick-ability
    pub item: String,

    /// The path to the configuration file
    #[arg(short, long, default_value = crate::args::DEFAULT_CFG)]
    pub config: String,
}

impl ClientArgs {
    pub fn to_config(&self) -> config::Config {
        config::parse(self.config.clone()).unwrap()
    }
}

impl std::fmt::Display for ClientArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let cfg = config::parse(self.config.clone()).unwrap();
        match cfg.client {
            Some(client) => {
                write!(f, "{}:{}", client.addr, client.port)
            }
            None => {
                write!(f, "")
            }
        }
    }
}
fn validate(args: &ClientArgs) -> bool {
    let cfg = config::parse(args.config.clone()).unwrap();
    match cfg.client {
        Some(client) => {
            if client.port == 0 && client.addr.trim().is_empty() {
                return false;
            }
        }
        None => return false,
    }
    if args.item.trim().is_empty() {
        return false;
    }
    true
}
pub fn display_help_and_exit() {
    let mut cmd = ClientArgs::command();
    cmd.print_help().unwrap();
    std::process::exit(exitcode::USAGE);
}
pub fn parse() -> crate::Result<ClientArgs> {
    let args = ClientArgs::parse();
    if !validate(&args) {
        return Err(Error::msg("Arguments addr and port cannot be empty."));
    }

    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[cfg_attr(not(feature = "complete"), ignore)]
    fn test_display() {
        let result = ClientArgs {
            item: "item".to_string(),
            config: "kickable.yaml".to_string(),
        };
        assert_eq!(format!("{result}"), "0.0.0.0:8080",);
    }
    #[test]
    fn test_validate() {
        let result = ClientArgs {
            item: "item".to_string(),
            config: "kickable.yaml".to_string(),
        };
        assert!(validate(&result));
    }
    #[test]
    fn test_to_config() {
        let _result = ClientArgs {
            item: "item".to_string(),
            config: "kickable.yaml".to_string(),
        }
        .to_config();
    }

    fn write_temp_yaml(name: &str, content: &str) -> String {
        let path = std::env::temp_dir().join(name);
        std::fs::write(&path, content).unwrap();
        path.to_string_lossy().into_owned()
    }

    #[test]
    fn validate_returns_false_when_no_client_section() {
        let path = write_temp_yaml("kickable_test_client_none.yaml", "items:\n  - it\n");
        assert!(!validate(&ClientArgs {
            item: "it".to_string(),
            config: path,
        }));
    }

    #[test]
    fn validate_returns_false_for_empty_item() {
        assert!(!validate(&ClientArgs {
            item: "  ".to_string(),
            config: "kickable.yaml".to_string(),
        }));
    }

    #[test]
    fn display_returns_empty_when_no_client_section() {
        let path = write_temp_yaml("kickable_test_client_display_none.yaml", "items:\n  - it\n");
        let args = ClientArgs {
            item: "it".to_string(),
            config: path,
        };
        assert_eq!(format!("{args}"), "");
    }

    #[test]
    fn display_returns_addr_port() {
        let args = ClientArgs {
            item: "it".to_string(),
            config: "kickable.yaml".to_string(),
        };
        assert_eq!(format!("{args}"), "0.0.0.0:8080");
    }
}

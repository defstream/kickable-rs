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

#[cfg(not(tarpaulin_include))]
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
#[cfg(not(tarpaulin_include))]
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
#[cfg(not(tarpaulin_include))]
pub fn display_help_and_exit() {
    let mut cmd = ClientArgs::command();
    cmd.print_help().unwrap();
    std::process::exit(exitcode::USAGE);
}
#[cfg(not(tarpaulin_include))]
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
        assert_eq!(format!("{result}"), "127.0.0.1:8080",);
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
        let result = ClientArgs {
            item: "item".to_string(),
            config: "kickable.yaml".to_string(),
        }.to_config();
    }
}

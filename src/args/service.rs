use anyhow::Error;

use clap::CommandFactory;
use clap::{ArgGroup, Parser};

use crate::config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(ArgGroup::new("service")))]
pub struct ServiceArgs {
    /// The path to the configuration file
    #[arg(short, long, default_value = crate::args::DEFAULT_CFG)]
    pub config: String,
}

impl ServiceArgs {
    pub fn to_config(&self) -> config::Config {
        config::parse(self.config.clone()).unwrap()
    }
}

#[cfg(not(tarpaulin_include))]
impl std::fmt::Display for ServiceArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let cfg = config::parse(self.config.clone()).unwrap();
        match cfg.server {
            Some(server) => {
                write!(f, "{}:{}", server.addr, server.port)
            }
            None => {
                write!(f, "")
            }
        }
    }
}

#[cfg(not(tarpaulin_include))]
fn validate(args: &ServiceArgs) -> bool {
    let cfg = config::parse(args.config.clone()).unwrap();
    match cfg.server {
        Some(server) => {
            if server.port == 0 && server.addr.trim().is_empty() {
                return false;
            }
        }
        None => return false,
    }
    true
}
#[cfg(not(tarpaulin_include))]
#[cfg(not(tarpaulin_include))]
pub fn parse() -> crate::Result<ServiceArgs> {
    let args = ServiceArgs::parse();

    if !validate(&args) {
        return Err(Error::msg("Arguments addr and port cannot be empty."));
    }

    Ok(args)
}
#[cfg(not(tarpaulin_include))]
pub fn display_help_and_exit() {
    let mut cmd = ServiceArgs::command();
    cmd.print_help().unwrap();
    std::process::exit(exitcode::USAGE);
}
#[cfg(not(tarpaulin_include))]
pub fn display_error<T: std::fmt::Display>(args: ServiceArgs, e: T) {
    eprintln!("error {e} {args}");
    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[cfg_attr(not(feature = "complete"), ignore)]
    fn test_display() {
        let result = ServiceArgs {
            config: "kickable.yaml".to_string(),
        };
        assert_eq!(format!("{result}"), "0.0.0.0:8080",);
    }
    #[test]
    fn test_validate() {
        let result = ServiceArgs {
            config: "kickable.yaml".to_string(),
        };
        assert!(validate(&result));
    }
    #[test]
    fn test_to_config() {
        let result = ServiceArgs {
            config: "kickable.yaml".to_string(),
        }.to_config();
    }
}

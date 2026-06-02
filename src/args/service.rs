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
pub fn parse() -> crate::Result<ServiceArgs> {
    let args = ServiceArgs::parse();

    if !validate(&args) {
        return Err(Error::msg("Arguments addr and port cannot be empty."));
    }

    Ok(args)
}
pub fn display_help_and_exit() {
    let mut cmd = ServiceArgs::command();
    cmd.print_help().unwrap();
    std::process::exit(exitcode::USAGE);
}
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
        let _result = ServiceArgs {
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
    fn validate_returns_false_when_no_server_section() {
        let path = write_temp_yaml("kickable_test_service_none.yaml", "items:\n  - it\n");
        assert!(!validate(&ServiceArgs { config: path }));
    }

    #[test]
    fn display_returns_empty_when_no_server_section() {
        let path = write_temp_yaml("kickable_test_service_display_none.yaml", "items:\n  - it\n");
        let args = ServiceArgs { config: path };
        assert_eq!(format!("{args}"), "");
    }

    #[test]
    fn display_returns_addr_port() {
        let args = ServiceArgs {
            config: "kickable.yaml".to_string(),
        };
        assert_eq!(format!("{args}"), "0.0.0.0:8080");
    }
}

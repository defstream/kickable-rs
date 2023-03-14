use crate::config;
use anyhow::Error;
use clap::CommandFactory;
use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
ArgGroup::new("cli")
.required(true)
.args(["item"]),
))]
pub struct CliArgs {
    /// The item to check for kick-ability
    pub item: String,
    /// The path to the configuration file
    #[arg(short, long, default_value = crate::args::DEFAULT_CFG)]
    pub config: String,
}

impl CliArgs {
    pub fn to_config(&self) -> config::Config {
        config::parse(self.config.clone()).unwrap()
    }
}

#[cfg(not(tarpaulin_include))]
impl std::fmt::Display for CliArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let args: Vec<String> = std::env::args().collect();
        write!(f, "{args:?}")
    }
}
#[cfg(not(tarpaulin_include))]
fn validate(args: &CliArgs) -> bool {
    if args.item.trim().is_empty() {
        return false;
    }
    true
}
#[cfg(not(tarpaulin_include))]
fn validate_args() -> bool {
    if std::env::args().len() < 2 {
        return false;
    }
    true
}
#[cfg(not(tarpaulin_include))]
pub fn parse() -> crate::Result<CliArgs> {
    if !validate_args() {
        return Err(Error::msg("No arguments supplied."));
    }

    let args = CliArgs::parse();

    if !validate(&args) {
        return Err(Error::msg("Arguments port and addr cannot be empty."));
    }

    Ok(args)
}
#[cfg(not(tarpaulin_include))]
pub fn display_help_and_exit() {
    let mut cmd = CliArgs::command();
    cmd.print_help().unwrap();
    std::process::exit(exitcode::USAGE);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[cfg_attr(not(feature = "complete"), ignore)]
    fn test_display() {
        let result = CliArgs {
            item: "it".to_string(),
            config: "kickable.yaml".to_string(),
        };
        assert_eq!(
            format!("The origin is: {result:?}"),
            "The origin is: CliArgs { item: \"it\", config: \"kickable.yaml\" }"
        );
    }
    #[test]
    #[cfg_attr(not(feature = "complete"), ignore)]
    fn test_parse_empty_args() {
        let result = parse();
        assert!(result.is_err());
    }
    #[test]
    fn test_validate() {
        let result = CliArgs {
            item: "it".to_string(),
            config: "kickable.yaml".to_string(),
        };
        assert!(validate(&result));
    }

    #[test]
    fn test_to_config() {
        let result = CliArgs {
            item: "it".to_string(),
            config: "kickable.yaml".to_string(),
        }
        .to_config();
    }
    #[test]
    #[cfg_attr(not(feature = "complete"), ignore)]
    fn test_validate_args_empty() {
        assert!(!validate_args());
    }
}

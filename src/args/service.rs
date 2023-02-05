use anyhow::Error;
use clap::CommandFactory;
use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(ArgGroup::new("service")))]
pub struct ServiceArgs {
    /// The port to listen on}
    #[arg(short, long, default_value_t = 31337)]
    pub port: u16,

    /// The IP address to bind to
    #[arg(short, long, default_value = "0.0.0.0")]
    pub addr: String,
}

#[cfg(not(tarpaulin_include))]
impl std::fmt::Display for ServiceArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.addr, self.port)
    }
}
#[cfg(not(tarpaulin_include))]
fn validate(args: &ServiceArgs) -> bool {
    if args.port == 0 && args.addr.trim().is_empty() {
        return false;
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
            port: 8000,
            addr: "test".to_string(),
        };
        assert_eq!(format!("{result}"), "test:8000",);
    }
    #[test]
    fn test_validate() {
        let result = ServiceArgs {
            port: 8000,
            addr: "test".to_string(),
        };
        assert!(validate(&result));
    }
}

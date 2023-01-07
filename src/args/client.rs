use clap::CommandFactory;
use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(ArgGroup::new("client")))]
pub struct ClientArgs {
    /// The item to check for kick-ability
    pub item: String,

    /// The service port to connect to
    #[arg(short, long, default_value_t = 31337)]
    pub port: u16,

    /// The address of the service to connect
    #[arg(short, long, default_value = "127.0.0.1")]
    pub addr: String,
}

#[cfg(not(tarpaulin_include))]
impl std::fmt::Display for ClientArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.addr, self.port)
    }
}

fn validate(args: &ClientArgs) -> bool {
    if args.port == 0 && args.addr.trim().is_empty() {
        return false;
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

#[cfg(not(tarpaulin_include))]
pub fn parse() -> crate::Result<ClientArgs> {
    let args = ClientArgs::parse();
    if !validate(&args) {
        return Err("Arguments addr and port cannot be empty.");
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
            port: 8000,
            addr: "test".to_string(),
        };
        assert_eq!(format!("{result}"), "test:8000",);
    }
    #[test]
    fn test_validate() {
        let result = ClientArgs {
            item: "item".to_string(),
            port: 8000,
            addr: "test".to_string(),
        };
        assert!(validate(&result));
    }
}

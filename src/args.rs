use kickable::Result;

use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
ArgGroup::new("kickable")
.required(true)
.args(["item"]),
))]
pub(crate) struct Args {
    /// The item to check for kick-ability
    pub item: String,
}

#[cfg(not(tarpaulin_include))]
impl std::fmt::Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let args: Vec<String> = std::env::args().collect();
        write!(f, "{:?}", args)
    }
}

fn validate(args: &Args) -> bool {
    if args.item.trim().is_empty() {
        return false;
    }
    true
}

fn validate_args() -> bool {
    if std::env::args().len() < 2 {
        return false;
    }
    true
}

#[cfg(not(tarpaulin_include))]
pub(crate) fn parse() -> Result<Args> {
    if !validate_args() {
        return Err("No arguments supplied.");
    }

    let args = Args::parse();

    if !validate(&args) {
        return Err("Item cannot be empty.");
    }

    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[cfg_attr(not(feature = "complete"), ignore)]
    fn test_display() {
        let result = Args {
            item: "it".to_string(),
        };
        assert_eq!(
            format!("The origin is: {result:?}"),
            "The origin is: Args { item: \"it\" }"
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
        let result = Args {
            item: "it".to_string(),
        };
        assert!(validate(&result));
    }
    #[test]
    #[cfg_attr(not(feature = "complete"), ignore)]
    fn test_validate_args_empty() {
        assert!(!validate_args());
    }
}

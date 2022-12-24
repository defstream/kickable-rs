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

impl std::fmt::Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let args: Vec<String> = std::env::args().collect();
        write!(f, "{:?}", args)
    }
}

pub(crate) fn parse() -> Result<Args> {
    if std::env::args().len() < 2 {
        return Err("No arguments supplied.");
    }
    let args = Args::parse();
    if args.item.trim().is_empty() {
        return Err("Item cannot be empty.");
    }
    Ok(args)
}

mod args;
use clap::CommandFactory;

const NO: &str = "No.";
const YES: &str = "Yes, yes you can.";

fn main() {
    // parse arguments
    match args::parse() {
        Ok(args) => {
            // validate kick-ability
            if kickable::validate(&args.item) {
                println!("{YES}");
                std::process::exit(exitcode::OK);
            }
            println!("{NO}");
            std::process::exit(exitcode::DATAERR);
        }
        Err(_) => {
            // if error, print help
            let mut cmd = args::Args::command();
            cmd.print_help().unwrap();
            std::process::exit(exitcode::USAGE);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        main();
    }
}

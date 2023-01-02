mod args;
use clap::CommandFactory;

const NO: &str = "No.";
const YES: &str = "Yes, yes you can.";

#[cfg(not(tarpaulin_include))]
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
    use assert_cli;

    #[test]
    #[cfg_attr(not(feature = "complete"), ignore)]
    fn test_main_good() {
        assert_cli::Assert::main_binary()
            .with_args(&["it"])
            .unwrap();
    }
    #[test]
    #[cfg_attr(not(feature = "complete"), ignore)]
    fn test_main_bad() {
        assert_cli::Assert::main_binary()
            .with_args(&["bad"])
            .fails()
            .unwrap();
    }
    #[test]
    #[cfg_attr(not(feature = "complete"), ignore)]
    fn test_main_empty_args() {
        assert_cli::Assert::main_binary().fails().unwrap();
    }
}

const NO: &str = "No.";
const YES: &str = "Yes, yes you can.";

#[cfg(not(tarpaulin_include))]
fn main() {
    // parse arguments
    match kickable::args::cli::parse() {
        Ok(args) => {
            // validate kick-ability
            if kickable::validate(&args.item) {
                println!("{YES}");
                std::process::exit(exitcode::OK);
            }
            println!("{NO}");
            std::process::exit(exitcode::DATAERR);
        }
        Err(_) => kickable::args::cli::display_help_and_exit(),
    }
}

#[cfg(test)]
mod tests {
    use assert_cli;
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

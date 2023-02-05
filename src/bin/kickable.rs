#[cfg(not(tarpaulin_include))]
fn main() {
    // parse arguments
    match kickable::args::cli::parse() {
        Ok(args) => {
            // validate kick-ability
            if kickable::validate(&args.item) {
                let yes = kickable::i18n::yes(args.lang);
                println!("{yes}");
                std::process::exit(exitcode::OK);
            }
            let no = kickable::i18n::no(args.lang);
            println!("{no}");
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

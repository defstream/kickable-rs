fn main() {
    // parse arguments
    match kickable::args::cli::parse() {
        Ok(args) => {
            // read configuration
            let cfg = args.to_config();
            // validate kick-ability
            if kickable::validate(&args.item) {
                let response = match cfg.lang {
                    Some(lang) => kickable::i18n::yes(lang),
                    None => String::from("true"),
                };
                println!("{response}");
                std::process::exit(exitcode::OK);
            }
            let response = match cfg.lang {
                Some(lang) => kickable::i18n::no(lang),
                None => String::from("false"),
            };
            println!("{response}");
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

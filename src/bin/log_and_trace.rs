extern crate pretty_env_logger;

use log::{error, info, trace};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn init_log_and_trace() {
    pretty_env_logger::init();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[cfg(not(tarpaulin_include))]
fn main() {
    init_log_and_trace();
    //initialize logger
    info!("kickable has started");
    trace!("parsing cli args");
    // parse arguments
    match kickable::args::cli::parse() {
        Ok(args) => {
            trace!("parsed cli args: {}", args);
            // read configuration
            trace!("parsing config");
            let cfg = args.to_config();
            trace!("parsed config: {:?}", cfg);

            trace!("validating item: {}", args.item);
            // validate kick-ability
            if kickable::validate(&args.item) {
                info!("validated item: {}, kickable = true", args.item);
                let response = match cfg.lang {
                    Some(lang) => {
                        trace!(
                            "parsed lang: {}, returning: {}",
                            lang,
                            kickable::i18n::yes(lang.clone())
                        );
                        kickable::i18n::yes(lang)
                    }
                    None => {
                        trace!("could not parse lang, returning: true");
                        String::from("true")
                    }
                };
                println!("{response}");
                info!("kickable has exited");
                std::process::exit(exitcode::OK);
            }
            info!("validated item: {}, kickable = false", args.item);
            let response = match cfg.lang {
                Some(lang) => {
                    trace!(
                        "parsed lang: {}, returning: {}",
                        lang,
                        kickable::i18n::no(lang.clone())
                    );
                    kickable::i18n::no(lang)
                }
                None => {
                    trace!("could not parse lang, returning: false");
                    String::from("false")
                }
            };
            println!("{response}");
            info!("kickable has exited");
            std::process::exit(exitcode::DATAERR);
        }
        Err(err) => {
            error!("could not parse cli args: {err}");
            kickable::args::cli::display_help_and_exit()
        }
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

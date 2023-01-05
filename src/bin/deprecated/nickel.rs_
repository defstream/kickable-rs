#[macro_use]
extern crate nickel;
use clap::CommandFactory;
use nickel::{HttpRouter, MediaType, Nickel};

fn main() {
    let mut server = Nickel::new();
    server.get(
        "/:it",
        middleware! { |req, mut res|
            res.set(MediaType::Json);

            let item = req.param("it").unwrap();
            let val = kickable::validate(item);

            format!("{val}")
        },
    );
    match kickable::service_args::parse() {
        Ok(args) => {
            server.listen(format!("{args}")).unwrap();
        }
        Err(_) => {
            let mut cmd = kickable::service_args::ServiceArgs::command();
            cmd.print_help().unwrap();
            std::process::exit(exitcode::USAGE);
        }
    }
}

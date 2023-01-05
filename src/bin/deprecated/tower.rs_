#[macro_use]
extern crate tower_web;

use tower_web::ServiceBuilder;
use clap::CommandFactory;

#[derive(Clone, Debug)]
pub struct KickableResource;

impl_web! {
    impl KickableResource {
        #[get("/:item")]
        fn can_i_kick_it(&self, item: String) -> Result<String, ()> {
            let result = kickable::validate(item.as_str());
            Ok(format!("{result}"))
        }
    }
}

pub fn main() {
    match kickable::service_args::parse() {
        Ok(args) => match args.to_string().parse::<std::net::SocketAddr>() {
            Ok(addr) => {
                println!("Listening on http://{}", addr);
                ServiceBuilder::new()
                    .resource(KickableResource)
                    .run(&addr)
                    .unwrap();
            }
            Err(e) => {
                eprintln!("error {} {}", e, args);
                std::process::exit(1);
            }
        },
        Err(_) => {
            let mut cmd = kickable::service_args::ServiceArgs::command();
            cmd.print_help().unwrap();
            std::process::exit(exitcode::USAGE);
        }
    }
}
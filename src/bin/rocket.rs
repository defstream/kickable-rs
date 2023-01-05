use clap::CommandFactory;

#[macro_use]
extern crate rocket;

#[get("/<it>")]
fn can_i_kick_it(it: &str) -> String {
    let val = kickable::validate(it);
    format!("{val}")
}

#[launch]
fn rocket() -> _ {
    match kickable::service_args::parse() {
        Ok(args) => {
            let figment = rocket::Config::figment()
                .merge(("port", args.port))
                .merge(("address", args.addr));
            rocket::custom(figment).mount("/", routes![can_i_kick_it])
        }
        Err(_) => {
            let mut cmd = kickable::service_args::ServiceArgs::command();
            cmd.print_help().unwrap();
            std::process::exit(exitcode::USAGE);
        }
    }
}

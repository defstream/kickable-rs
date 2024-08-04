#[macro_use]
extern crate rocket;
#[get("/<it>")]
fn can_i_kick_it(it: &str) -> String {
    let val = kickable::validate(it);
    format!("{val}")
}
#[launch]
fn rocket() -> _ {
    match kickable::args::service::parse() {
        Ok(args) => {
            let cfg = args.to_config();
            match cfg.server {
                Some(server) => {
                    let figment = rocket::Config::figment()
                        .merge(("port", server.port))
                        .merge(("address", server.addr));
                    rocket::custom(figment).mount("/", routes![can_i_kick_it])
                }
                None => {
                    kickable::args::service::display_help_and_exit();
                    std::process::exit(1);
                }
            }
        }
        Err(_) => {
            kickable::args::service::display_help_and_exit();
            std::process::exit(1);
        }
    }
}

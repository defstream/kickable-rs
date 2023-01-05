extern crate iron;
extern crate router;

use clap::CommandFactory;
use iron::status;
use iron::{Iron, IronResult, Request, Response};
use router::Router;

fn can_i_kick_it(req: &mut Request) -> IronResult<Response> {
    let item = req.extensions.get::<Router>().unwrap().find("it").unwrap();
    let val = kickable::validate(item);
    let res = format!("{val}");
    Ok(Response::with((status::Ok, res)))
}

fn main() {
    let mut router = Router::new();
    router.get("/:it", can_i_kick_it, "can_i_kick_it");
    match kickable::service_args::parse() {
        Ok(args) => {
            Iron::new(router).http(args.to_string().as_str()).unwrap();
        }
        Err(_) => {
            let mut cmd = kickable::service_args::ServiceArgs::command();
            cmd.print_help().unwrap();
            std::process::exit(exitcode::USAGE);
        }
    }
}

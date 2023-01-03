extern crate iron;
extern crate router;

use iron::status;
use iron::{Iron, IronResult, Request, Response};
use kickable;
use router::Router;

fn can_i_kick_it(req: &mut Request) -> IronResult<Response> {
    let ref item = req.extensions.get::<Router>().unwrap().find("it").unwrap();

    let val = kickable::validate(item);
    let res = format!("{val}");
    Ok(Response::with((status::Ok, res)))
}

fn main() {
    let mut router = Router::new();
    router.get("/:it", can_i_kick_it, "can_i_kick_it");
    Iron::new(router).http("localhost:31337").unwrap();
}

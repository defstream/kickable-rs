#[macro_use] extern crate nickel;

use kickable;
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

    server.listen("127.0.0.1:31337").unwrap();
}

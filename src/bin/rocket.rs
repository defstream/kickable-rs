#[macro_use]
extern crate rocket;

#[get("/<it>")]
fn can_i_kick_it(it: &str) -> String {
    let val = kickable::validate(it);
    format!("{val}")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![can_i_kick_it])
}

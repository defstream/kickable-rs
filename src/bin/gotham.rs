use gotham::prelude::*;
use gotham::router::{build_simple_router, Router};
use gotham::state::State;
use serde::Deserialize;

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct PathExtractor {
    item: String,
}
fn can_i_kick_it(state: State) -> (State, String) {
    let message = {
        let req = PathExtractor::borrow_from(&state);
        let val = kickable::validate(req.item.as_str());
        format!("{val}")
    };
    (state, message)
}
fn router() -> Router {
    build_simple_router(|route| {
        route
            .get("/:item")
            .with_path_extractor::<PathExtractor>()
            .to(can_i_kick_it);
    })
}
pub fn main() {
    match kickable::args::service::parse() {
        Ok(args) => {
            gotham::start(args.to_string(), router()).unwrap();
        }
        Err(_) => kickable::args::service::display_help_and_exit(),
    }
}

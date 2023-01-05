use saphir::prelude::*;
async fn can_i_kick_it(req: Request<Body>) -> (u16, String)  {
    let item = req.captures().get("item").unwrap().as_str();
    let result = kickable::validate(item);
    (200, result.to_string())
}
#[tokio::main]
async fn main() -> Result<(), SaphirError> {
    env_logger::init();
    let server = Server::builder()
        .configure_listener(|l| {
            l.interface("127.0.0.1:3000")
        })
        .configure_router(|r| {
            r.route("/{item}", Method::GET, can_i_kick_it)
        })
        .build();

    server.run().await
}
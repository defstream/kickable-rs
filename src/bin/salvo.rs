use salvo::prelude::*;
#[cfg(not(tarpaulin_include))]
#[handler]
async fn can_i_kick_it(req: &mut Request, _: &mut Response) -> String {
    let id = req.param::<String>("item").unwrap();
    let val = kickable::validate(id.as_str());
    format!("{val}")
}
#[cfg(not(tarpaulin_include))]
#[tokio::main]
async fn main() {
    let router = Router::new().push(Router::with_path("/<item>").get(can_i_kick_it));

    match kickable::args::service::parse() {
        Ok(args) => {
            let acceptor = TcpListener::bind(&args.to_string());
            Server::new(acceptor).serve(router).await;
        }
        Err(_) => kickable::args::service::display_help_and_exit(),
    }
}

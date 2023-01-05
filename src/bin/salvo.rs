use clap::CommandFactory;
use salvo::prelude::*;

#[handler]
async fn can_i_kick_it(req: &mut Request, _: &mut Response) -> String {
    let id = req.param::<String>("item").unwrap();

    let val = kickable::validate(id.as_str());
    format!("{val}")
}
#[tokio::main]
async fn main() {
    let router = Router::new().push(Router::with_path("/<item>").get(can_i_kick_it));

    match kickable::service_args::parse() {
        Ok(args) => {
            let acceptor = TcpListener::bind(&args.to_string());
            Server::new(acceptor).serve(router).await;
        }
        Err(_) => {
            let mut cmd = kickable::service_args::ServiceArgs::command();
            cmd.print_help().unwrap();
            std::process::exit(exitcode::USAGE);
        }
    }
}

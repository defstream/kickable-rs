use clap::CommandFactory;
use poem::{
    get, handler, listener::TcpListener, middleware::Tracing, web::Path, EndpointExt, Route, Server,
};

#[handler]
fn can_i_kick_it(Path(item): Path<String>) -> String {
    let val = kickable::validate(item.as_str());
    format!("{val}")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Route::new().at("/:it", get(can_i_kick_it)).with(Tracing);

    match args::service::parse() {
        Ok(args) => {
            Server::new(TcpListener::bind(format!("{args}")))
                .name("kickable")
                .run(app)
                .await
        }
        Err(_) => {
            let mut cmd = args::service::ServiceArgs::command();
            cmd.print_help().unwrap();
            std::process::exit(exitcode::USAGE);
        }
    }
}

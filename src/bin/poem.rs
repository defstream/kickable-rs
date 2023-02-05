use poem::{
    get, handler, listener::TcpListener, middleware::Tracing, web::Path, EndpointExt, Route, Server,
};
#[cfg(not(tarpaulin_include))]
#[handler]
fn can_i_kick_it(Path(item): Path<String>) -> String {
    let val = kickable::validate(item.as_str());
    format!("{val}")
}
#[cfg(not(tarpaulin_include))]
#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();
    let app = Route::new().at("/:it", get(can_i_kick_it)).with(Tracing);
    match kickable::args::service::parse() {
        Ok(args) => {
            dbg!(Server::new(TcpListener::bind(format!("{args}")))
                .name("kickable")
                .run(app)
                .await
                .unwrap());
        }
        Err(_) => {
            kickable::args::service::display_help_and_exit();
        }
    }
}

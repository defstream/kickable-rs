use axum::{
    error_handling::HandleErrorLayer, extract::Path, http::StatusCode, response::IntoResponse,
    routing::get, Router,
};
use clap::CommandFactory;
use std::{borrow::Cow, time::Duration};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;

async fn can_i_kick_it(Path(item): Path<String>) -> Result<String, StatusCode> {
    let result = kickable::validate(item.as_str());
    Ok(format!("{result}"))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/:item", get(can_i_kick_it)).layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(handle_error))
            .load_shed()
            .concurrency_limit(1024)
            .timeout(Duration::from_secs(10))
            .layer(TraceLayer::new_for_http())
            .into_inner(),
    );
    match args::service::parse() {
        Ok(args) => match args.to_string().parse() {
            Ok(addr) => {
                axum::Server::bind(&addr)
                    .serve(app.into_make_service())
                    .await
                    .unwrap();
            }
            Err(e) => {
                eprintln!("error {} {}", e, args);
                std::process::exit(1);
            }
        },
        Err(_) => {
            let mut cmd = args::service::ServiceArgs::command();
            cmd.print_help().unwrap();
            std::process::exit(exitcode::USAGE);
        }
    }
}

async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {}", error)),
    )
}

use axum::{
    error_handling::HandleErrorLayer, extract::Path, http::StatusCode, response::IntoResponse,
    routing::get, Router,
};
use std::{borrow::Cow, time::Duration};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;

#[cfg(not(tarpaulin_include))]
async fn can_i_kick_it(Path(item): Path<String>) -> Result<String, StatusCode> {
    let result = kickable::validate(item.as_str());
    Ok(format!("{result}"))
}

#[cfg(not(tarpaulin_include))]
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
    match kickable::args::service::parse() {
        Ok(args) => {
            let listener = tokio::net::TcpListener::bind(args.to_string())
                .await
                .unwrap();
            axum::serve(listener, app).await.unwrap();
        }
        Err(_) => kickable::args::service::display_help_and_exit(),
    }
}

#[cfg(not(tarpaulin_include))]
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
        Cow::from(format!("Unhandled internal error: {error}")),
    )
}

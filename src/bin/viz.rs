use viz::{serve, types::Params, Request, RequestExt, Result, Router};
async fn can_i_kick_it(mut req: Request) -> Result<String> {
    let item = req.extract::<Params<String>>().await?;
    let result = kickable::validate(item.as_str());
    let response = format!("{result}");
    Ok(response)
}
#[tokio::main]
async fn main() {
    let app = Router::new().get("/:item", can_i_kick_it);

    match kickable::args::service::parse() {
        Ok(args) => match tokio::net::TcpListener::bind(args.to_string()).await {
            Ok(listener) => {
                if let Err(err) = serve(listener, app).await {
                    eprintln!("{err}");
                }
            }
            Err(e) => kickable::args::service::display_error(args, e),
        },
        Err(_) => kickable::args::service::display_help_and_exit(),
    }
}

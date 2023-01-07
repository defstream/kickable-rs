use viz::{types::Params, Request, RequestExt, Result, Router, Server, ServiceMaker};

async fn can_i_kick_it(mut req: Request) -> Result<String> {
    let item = req.extract::<Params<String>>().await?;
    let result = kickable::validate(item.as_str());
    let response = format!("{result}");
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().get("/:item", can_i_kick_it);

    match kickable::args::service::parse() {
        Ok(args) => match args.to_string().parse() {
            Ok(addr) => {
                if let Err(err) = Server::bind(&addr).serve(ServiceMaker::from(app)).await {
                    eprintln!("{err}");
                }
            }
            Err(e) => kickable::args::service::display_error(args, e),
        },
        Err(_) => kickable::args::service::display_help_and_exit(),
    }
    Ok(())
}

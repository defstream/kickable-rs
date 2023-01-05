use clap::CommandFactory;
use tide::Request;

async fn can_i_kick_it(req: Request<()>) -> tide::Result<String> {
    let item = req.param("it")?;
    let val = kickable::validate(item);
    let res = format!("{val}");
    Ok(res)
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    app.at("/:it").get(can_i_kick_it);

    match kickable::service_args::parse() {
        Ok(args) => {
            app.listen(format!("{args}")).await?;
            Ok(())
        }
        Err(_) => {
            let mut cmd = kickable::service_args::ServiceArgs::command();
            cmd.print_help().unwrap();
            std::process::exit(exitcode::USAGE);
        }
    }
}

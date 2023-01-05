use clap::CommandFactory;
use graphul::{http::Methods, Context, Graphul};

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();
    app.get("/:item", |c: Context| async move {
        let item = c.params("item");
        let result = kickable::validate(item.as_str());
        format!("{result}")
    });

    match kickable::service_args::parse() {
        Ok(args) => {
            app.run(args.to_string().as_str()).await;
        }
        Err(_) => {
            let mut cmd = kickable::service_args::ServiceArgs::command();
            cmd.print_help().unwrap();
            std::process::exit(exitcode::USAGE);
        }
    }
}

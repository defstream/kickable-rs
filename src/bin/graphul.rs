use graphul::{http::Methods, Context, Graphul};

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();
    app.get("/:item", |c: Context| async move {
        let item = c.params("item");
        let result = kickable::validate(item.as_str());
        format!("{result}")
    });

    match kickable::args::service::parse() {
        Ok(args) => {
            app.run(args.to_string().as_str()).await;
        }
        Err(_) => kickable::args::service::display_help_and_exit(),
    }
}

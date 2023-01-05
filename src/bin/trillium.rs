use clap::CommandFactory;
use trillium::Conn;
use trillium_router::{Router, RouterConnExt};

pub fn main() {
    match args::service::parse() {
        Ok(args) => {
            trillium_smol::config()
                .with_port(args.port)
                .with_host(args.addr.as_str())
                .run(Router::new().get("/:item", |conn: Conn| async move {
                    let item = conn.param("item").unwrap();
                    let result = kickable::validate(item);
                    conn.ok(format!("{result}"))
                }));
        }
        Err(_) => {
            let mut cmd = args::service::ServiceArgs::command();
            cmd.print_help().unwrap();
            std::process::exit(exitcode::USAGE);
        }
    }
}

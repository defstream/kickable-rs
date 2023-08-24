use trillium::Conn;
use trillium_router::{Router, RouterConnExt};
#[cfg(not(tarpaulin_include))]
pub fn main() {
    if let Ok(args) = kickable::args::service::parse() {
        if let Some(server) = args.to_config().server {
            trillium_smol::config()
                .with_port(server.port)
                .with_host(server.addr.as_str())
                .run(Router::new().get("/:item", |conn: Conn| async move {
                    let item = conn.param("item").unwrap();
                    let result = kickable::validate(item);
                    conn.ok(format!("{result}"))
                }));
            return;
        }
    }
    kickable::args::service::display_help_and_exit();
    std::process::exit(1);
}

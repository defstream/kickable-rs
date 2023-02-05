use trillium::Conn;
use trillium_router::{Router, RouterConnExt};
#[cfg(not(tarpaulin_include))]
pub fn main() {
    match kickable::args::service::parse() {
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
        Err(_) => kickable::args::service::display_help_and_exit(),
    }
}

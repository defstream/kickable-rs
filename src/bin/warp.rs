use warp::Filter;

#[cfg(not(tarpaulin_include))]
#[tokio::main]
async fn main() {
    let can_i_kick_it = warp::get()
        .and(warp::path::param::<String>())
        .map(|item: String| {
            let val = kickable::validate(item.as_str());
            format!("{val}")
        });

    let routes = warp::get().and(can_i_kick_it);

    match kickable::args::service::parse() {
        Ok(args) => match args.to_string().parse::<std::net::SocketAddr>() {
            Ok(addr) => {
                warp::serve(routes).run(addr).await;
            }
            Err(e) => kickable::args::service::display_error(args, e),
        },
        Err(_) => kickable::args::service::display_help_and_exit(),
    }
}

use clap::CommandFactory;
use warp::Filter;

#[tokio::main]
async fn main() {
    let can_i_kick_it = warp::get()
        .and(warp::path::param::<String>())
        .map(|item: String| {
            let val = kickable::validate(item.as_str());
            format!("{val}")
        });

    let routes = warp::get().and(can_i_kick_it);

    match args::service::parse() {
        Ok(args) => match args.to_string().parse::<std::net::SocketAddr>() {
            Ok(addr) => {
                warp::serve(routes).run(addr).await;
            }
            Err(e) => {
                eprintln!("error {} {}", e, args);
                std::process::exit(1);
            }
        },
        Err(_) => {
            let mut cmd = args::service::ServiceArgs::command();
            cmd.print_help().unwrap();
            std::process::exit(exitcode::USAGE);
        }
    }
}

use actix_web::{get, web, App, HttpServer, Responder};
use clap::CommandFactory;

#[get("/{item}")]
async fn can_i_kick_it(item: web::Path<String>) -> impl Responder {
    let val = kickable::validate(item.as_str());
    format!("{val}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match kickable::service_args::parse() {
        Ok(args) => {
            HttpServer::new(|| App::new().service(can_i_kick_it))
                .bind((args.addr, args.port))?
                .run()
                .await
        }
        Err(_) => {
            let mut cmd = kickable::service_args::ServiceArgs::command();
            cmd.print_help().unwrap();
            std::process::exit(exitcode::USAGE);
        }
    }
}

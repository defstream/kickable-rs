use clap::CommandFactory;
use tonic::{transport::Server, Request, Response, Status};

use kickable_proto::kickable_server::{Kickable, KickableServer};
use kickable_proto::{KickableReply, KickableRequest};

pub mod kickable_proto {
    tonic::include_proto!("kickable");
}

#[derive(Default)]
pub struct TonicServer {}

#[tonic::async_trait]
impl Kickable for TonicServer {
    async fn validate(
        &self,
        request: Request<KickableRequest>,
    ) -> Result<Response<KickableReply>, Status> {
        let item = request.into_inner().item;
        let result = kickable::validate(item.as_str());
        let reply = kickable_proto::KickableReply { result };
        println!("{result}");
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match args::service::parse() {
        Ok(args) => match args.to_string().parse() {
            Ok(addr) => {
                let server = TonicServer::default();
                Server::builder()
                    .add_service(KickableServer::new(server))
                    .serve(addr)
                    .await?;
                Ok(())
            }
            Err(e) => {
                eprintln!("error parsing {} - {}", args, e);
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

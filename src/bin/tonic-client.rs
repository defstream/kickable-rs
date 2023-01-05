use clap::CommandFactory;

use kickable_proto::kickable_client::KickableClient;
use kickable_proto::KickableRequest;

pub mod kickable_proto {
    tonic::include_proto!("kickable");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match args::service::parse() {
        Ok(args) => {
            let mut client = KickableClient::connect(format!("http://{}", args)).await?;
            let request = tonic::Request::new(KickableRequest {
                item: "not me".into(),
            });
            let response = client.validate(request).await?;
            println!("RESPONSE={:?}", response);

            Ok(())
        }
        Err(_) => {
            let mut cmd = args::service::ServiceArgs::command();
            cmd.print_help().unwrap();
            std::process::exit(exitcode::USAGE);
        }
    }
}

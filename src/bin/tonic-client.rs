use kickable_proto::kickable_client::KickableClient;
use kickable_proto::KickableRequest;

pub mod kickable_proto {
    tonic::include_proto!("kickable");
}

#[tokio::main]
async fn main() {
    match kickable::args::client::parse() {
        Ok(args) => {
            let mut client = KickableClient::connect(format!("http://{}", args))
                .await
                .unwrap();
            let request = tonic::Request::new(KickableRequest { item: args.item });
            let response = client.validate(request).await.unwrap();
            println!("{response:?}");
        }
        Err(_) => kickable::args::client::display_help_and_exit(),
    }
}

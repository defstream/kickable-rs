use kickable_proto::kickable_client::KickableClient;
use kickable_proto::KickableRequest;
pub mod kickable_proto {
    tonic::include_proto!("kickable");
}
#[tokio::main]
async fn main() {
    if let Ok(args) = kickable::args::client::parse() {
        let cfg = args.to_config();
        if let Some(server) = cfg.client {
            let mut client =
                KickableClient::connect(format!("http://{}:{}", server.addr, server.port))
                    .await
                    .unwrap();
            let request = tonic::Request::new(KickableRequest { item: args.item });
            let is_kickable = client.validate(request).await.unwrap().into_inner().result;
            if is_kickable {
                if let Some(lang) = cfg.lang {
                    println!("{}", kickable::i18n::yes(lang));
                } else {
                    println!("true");
                }
                std::process::exit(exitcode::OK);
            } else {
                if let Some(lang) = cfg.lang {
                    println!("{}", kickable::i18n::no(lang));
                } else {
                    println!("false");
                }
                std::process::exit(exitcode::DATAERR);
            }
        }
    }
    kickable::args::client::display_help_and_exit()
}

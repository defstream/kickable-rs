use clap::CommandFactory;

#[macro_use]
extern crate rouille;

fn main() {
    match args::service::parse() {
        Ok(args) => {
            rouille::start_server(format!("{args}"), move |request| {
                router!(request,
                    (GET) (/{item: String}) => {
                        let val = kickable::validate(item.as_str());
                        rouille::Response::text(format!("{val}"))
                    },
                    _ => rouille::Response::empty_404()
                )
            });
        }
        Err(_) => {
            let mut cmd = args::service::ServiceArgs::command();
            cmd.print_help().unwrap();
            std::process::exit(exitcode::USAGE);
        }
    }
}

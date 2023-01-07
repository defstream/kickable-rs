#[macro_use]
extern crate rouille;

fn main() {
    match kickable::args::service::parse() {
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
        Err(_) => kickable::args::service::display_help_and_exit(),
    }
}

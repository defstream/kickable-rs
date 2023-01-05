use clap::CommandFactory;

use hyper::Body;
use thruster::{
    context::{
        basic_hyper_context::{generate_context, BasicHyperContext as Ctx, HyperRequest},
        context_ext::ContextExt,
    },
    errors::{ErrorSet, ThrusterError},
    hyper_server::HyperServer,
    m, middleware_fn, App, MiddlewareNext, MiddlewareResult, ThrusterServer,
};

#[middleware_fn]
async fn can_i_kick_it(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let item = &context
        .params()
        .get("item")
        .ok_or_else(|| ThrusterError::generic_error(Ctx::default()))?
        .param;
    let result = kickable::validate(item.as_str());

    context.body = Body::from(format!("{result}"));
    Ok(context)
}

#[tokio::main]
async fn main() {
    match kickable::service_args::parse() {
        Ok(args) => {
            HyperServer::new(
                App::<HyperRequest, Ctx, ()>::create(generate_context, ())
                    .get("/:item", m![can_i_kick_it]),
            )
            .build(args.addr.as_str(), args.port)
            .await;
        }
        Err(_) => {
            let mut cmd = kickable::service_args::ServiceArgs::command();
            cmd.print_help().unwrap();
            std::process::exit(exitcode::USAGE);
        }
    }
}

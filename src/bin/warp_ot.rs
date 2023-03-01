use opentelemetry::global::shutdown_tracer_provider;
use opentelemetry::sdk::trace::Config;
use opentelemetry::trace::TraceContextExt;
use opentelemetry::trace::TraceError;
use opentelemetry::{
    global,
    trace::{Tracer},
    Context, Key, KeyValue,
};
use opentelemetry::runtime;

use opentelemetry::sdk::Resource;
use std::{net::SocketAddr};
use warp::Filter;

fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
    opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("kickable-warp")
        .with_trace_config(Config::default().with_resource(Resource::new(vec![
            KeyValue::new("service.name", "kickable-warp"),
            KeyValue::new("exporter", "otlp-jaeger"),
        ])))
        .install_batch(runtime::Tokio)
}

const MAIN_KEY: Key = Key::from_static_str("kickable.rs/main");
const KICKABLE_KEY: Key = Key::from_static_str("kickable.rs/kickable");

#[cfg(not(tarpaulin_include))]
#[tokio::main]
async fn main() {
    let _tracer = init_tracer().expect("failed to initialize tracer");
    let cx = Context::new();

    let tracer = global::tracer("kickable.rs");
    let meter = global::meter("kickable.rs/meter");

    let req_counter = meter
        .u64_observable_counter("req")
        .with_description("Total requests")
        .init();

    let hit_counter = meter
        .u64_observable_counter("hit")
        .with_description("Total hits")
        .init();

    let miss_counter = meter
        .u64_observable_counter("miss")
        .with_description("Total misses")
        .init();

    let can_i_kick_it = warp::get()
        .and(warp::path::param::<String>())
        .map(|item: String| {
            let val: bool;
            tracer.in_span("can_i_kick_it", |cx| {
                let span = cx.span();
                val = kickable::validate(item.as_str());
                span.set_attribute(KICKABLE_KEY.string(item));
                let _ = meter.register_callback(move |cx| {
                    req_counter.observe(cx, 1, &[]);
                    if val {
                        hit_counter.observe(cx, 1, &[]);
                    } else {
                        miss_counter.observe(cx, 1, &[]);
                    }
                });
            });

            format!("{val}")
        });

    let routes = warp::get().and(can_i_kick_it);

    match kickable::args::service::parse() {
        Ok(args) => match args.to_string().parse::<SocketAddr>() {
            Ok(addr) => {
                warp::serve(routes).run(addr).await;
                shutdown_tracer_provider(); // sending remaining spans.
            }
            Err(e) => kickable::args::service::display_error(args, e),
        },
        Err(_) => kickable::args::service::display_help_and_exit(),
    }
}

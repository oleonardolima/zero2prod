use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{
    fmt::MakeWriter, prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Registry,
};

/// Compose multiple layers into a `tracing`'s subscriber.
///
/// It uses the `impl Subscriber` as return type to avoid having to
/// spell out the actual type of the returned subscriber.
///
/// It's explicitly `Send` and `Sync` to make it possible to pass it to `init_subscriber`.
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    // This 'weird'syntax is a higher-ranked trait bound (HRTB)
    Sink: for<'a> MakeWriter<'a> + Send + Sync + Sync + 'static,
{
    // fallback to info-level if the RUST_LOG environment variable is not set
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // redirects all third-party log's events to tracing subscriber
    LogTracer::init().expect("Failed to set LogTracer logger");

    set_global_default(subscriber).expect("Failed to set up tracing subscriber");
}

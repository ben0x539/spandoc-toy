use {
    std::io,
    eyre::{ContextExt},
    tracing::{info, span, Level},
    tracing_subscriber::{Registry, EnvFilter, layer::Layer},
    spandoc::{spandoc},
};

#[spandoc]
fn main() -> eyre::Result<()> {

    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    let fmt = tracing_subscriber::fmt::Layer::builder()
        .with_writer(io::stderr)
        .finish();
    let subscriber = tracing_error::ErrorLayer::default()
        .and_then(filter)
        .and_then(fmt)
        .with_subscriber(Registry::default());

    tracing::subscriber::set_global_default(subscriber)?;

    /// Outer comment
    {
        let span = span!(Level::INFO, "i'm a span");
        let _enter = span.enter();
        /// Inner comment
        info!("just an event");
        Err(eyre::err!("rip")).note("u died")?;
    }

    Ok(())
}

use axum::routing::get;
use axum::Router;
use clap::Parser;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

mod args;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let cli = args::Cli::parse();

    tracing::debug!(?cli);

    let app = Router::new().route("/", get(|| async { "hello world " }));

    let binding_address = format!("{0}:{1}", cli.address, cli.port);
    axum::Server::bind(&binding_address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

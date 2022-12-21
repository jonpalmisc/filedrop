mod handlers;
mod helpers;
mod settings;

use settings::Settings;

use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, put},
    Router, Server,
};

use std::sync::Arc;

#[tokio::main]
async fn main() {
    let trace_sub = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(trace_sub).unwrap();

    let settings = match Settings::load() {
        Ok(s) => Arc::new(s),
        _ => {
            error!("Failed to initialize settings.");
            return;
        }
    };

    let listen_addr = match settings.listen_addr() {
        Ok(a) => a,
        _ => {
            error!("Failed to parse listen address.");
            return;
        }
    };

    let router = Router::new()
        .route("/:name", put(handlers::upload))
        .route("/get/:name", get(handlers::download))
        .layer(DefaultBodyLimit::max(settings.max_request_size())) // 1 GB
        .with_state(settings.clone());

    info!(
        "Listening on {} ({})...",
        settings.listen_string(),
        settings.server_string()
    );
    info!(
        "Files will be served from and saved to '{}'.",
        settings.storage_path().display()
    );

    Server::bind(&listen_addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

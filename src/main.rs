mod handlers;
mod helpers;
mod settings;

use settings::Settings;

use dotenvy::dotenv;

use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, put},
    Router, Server,
};

use std::{env, sync::Arc};

use crate::settings::Variable;

/// Show usage information and exit.
#[rustfmt::skip]
fn show_usage_and_exit() -> ! {
    eprintln!("Minimal, CLI-friendly file transfer service\n");

    eprintln!("Options:");
    eprintln!("  {:<22}IP address to listen on", Variable::IP_KEY);
    eprintln!("  {:<22}Port to listen on", Variable::PORT_KEY);
    eprintln!("  {:<22}Port to use in URLs (defaults to normal port)", Variable::PUBLIC_PORT_KEY);
    eprintln!("  {:<22}Host name to use in URLs", Variable::HOST_KEY);
    eprintln!("  {:<22}Path to upload storage directory", Variable::STORAGE_KEY);
    eprintln!("  {:<22}Upload size limit (in bytes)", Variable::SIZE_LIMIT_KEY);

    eprintln!("\nAll options must be configured through environment variables.");

    std::process::exit(1);
}

#[tokio::main]
async fn main() {
    // Treat any arguments at all as a cry for help.
    if env::args().nth(1).is_some() {
        show_usage_and_exit();
    }

    // Load variables from a `.env` file, if one exists. Result is intentionally
    // ignored via `.ok()` since it doesn't really matter if there isn't one as
    // variables can still be supplied by hand.
    dotenv().ok();

    // Initialize logging subsystem.
    let trace_sub = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(trace_sub).unwrap();

    let settings = Arc::new(Settings::from_env());
    let listen_address = match settings.listen_address() {
        Ok(a) => a,
        _ => {
            error!("Failed to parse listen address.");
            return;
        }
    };

    let router = Router::new()
        .route("/:name", put(handlers::upload))
        .route("/get/:name", get(handlers::download))
        .layer(DefaultBodyLimit::max(settings.size_limit())) // 1 GB
        .with_state(settings.clone());

    info!(
        "Listening on {} ({})...",
        settings.listen_string(),
        settings.host_string()
    );
    info!(
        "Files will be served from and saved to '{}'.",
        settings.storage_path().display()
    );

    Server::bind(&listen_address)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

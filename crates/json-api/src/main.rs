//! Lattice JSON API Server

use std::process;

use salvo::{
    oapi::{OpenApi, swagger_ui::SwaggerUi},
    prelude::*,
    trailing_slash::remove_slash,
};
use tracing_subscriber::EnvFilter;

use crate::{config::ServerConfig, handlers::healthcheck};

mod config;
mod handlers;

/// Lattice JSON API Server entry point
///
/// # Panics
///
/// Panics if the server fails to bind or serve requests
#[tokio::main]
pub async fn main() {
    // Load configuration from .env and CLI arguments
    let config = ServerConfig::load().unwrap_or_else(|e| {
        #[expect(
            clippy::print_stderr,
            reason = "logging not initialized yet, must use eprintln for config errors"
        )]
        {
            eprintln!("Configuration error: {e}");
        }

        process::exit(1);
    });

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.log_level)),
        )
        .init();

    let addr = config.socket_addr();
    tracing::info!("Starting server on {addr}");

    // Bind server
    let listener = TcpListener::new(addr).bind().await;

    // Create router
    let router = Router::new()
        .hoop(CatchPanic::new())
        .hoop(remove_slash())
        .push(Router::with_path("healthcheck").get(healthcheck::handler));

    // Create OpenAPI documentation
    let doc = OpenApi::new("Lattice API", "0.1.0").merge_router(&router);

    // Add documentation routes
    let router = router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(SwaggerUi::new("/api-doc/openapi.json").into_router("docs"));

    tracing::debug!("{router:?}");

    // Start serving requests
    Server::new(listener).serve(router).await;
}

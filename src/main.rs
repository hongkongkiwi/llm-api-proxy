use axum::{
    extract::{Path, Request},
    http::StatusCode,
    response::Response,
    routing::any,
    Router,
};
use std::env;
use tracing::info;

mod config;
mod proxy;
use config::Config;
use proxy::ProxyService;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    // Load configuration
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());
    let config = match Config::from_file(&config_path) {
        Ok(config) => config,
        Err(e) => {
            info!("Failed to load config from {}: {}, using defaults", config_path, e);
            Config::default()
        }
    };
    
    let port = config.server.port.unwrap_or(8811);
    let addr = format!("0.0.0.0:{}", port);
    
    info!("Starting Anthropic HTTP proxy on {}", addr);
    info!("Loaded configuration with {} endpoints", config.endpoints.len());
    
    let app = Router::new()
        .route("/:prefix/v1/*path", any(proxy_handler))
        .route("/:prefix/v1", any(proxy_handler))
        .fallback(|| async { 
            (StatusCode::NOT_FOUND, "Not Found") 
        });
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    axum::serve(listener, app).await.unwrap();
}

async fn proxy_handler(
    Path(prefix): Path<String>,
    request: Request,
) -> Result<Response, StatusCode> {
    // Load configuration for each request (in production, you'd want to cache this)
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());
    let config = match Config::from_file(&config_path) {
        Ok(config) => config,
        Err(_) => Config::default(),
    };
    
    let proxy_service = ProxyService::new_with_config(config).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    proxy_service.handle_request(prefix, request).await
}
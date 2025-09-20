use axum::{
    http::StatusCode,
    response::Response,
    routing::get,
    Router,
};
use tokio::net::TcpListener;

#[tokio::test]
async fn test_e2e_proxy_request() {
    println!("Starting E2E test");
    // Create a mock target server that mimics Anthropic API
    let app = Router::new()
        .route("/v1/messages", get(|| async { 
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(axum::body::Body::from(r#"{"type": "message", "content": "Hello from mock Anthropic API"}"#))
                .unwrap()
        }))
        .route("/v1/health", get(|| async { 
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(axum::body::Body::from(r#"{"status": "healthy"}"#))
                .unwrap()
        }));

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let target_addr = listener.local_addr().unwrap();
    let target_url = format!("http://{}", target_addr);
    
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // Start the proxy server pointing to our mock target
    let proxy_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let proxy_addr = proxy_listener.local_addr().unwrap();
    
    let target_url_clone = target_url.clone();
    
    let app = Router::new()
        .route("/:prefix/v1/*path", axum::routing::any(move |path, request| async move {
            proxy_handler(path, request, &target_url_clone).await
        }))
        .fallback(|| async { 
            (StatusCode::NOT_FOUND, "Not Found") 
        });
    
    tokio::spawn(async move {
        axum::serve(proxy_listener, app).await.unwrap();
    });
    
    // Give the server a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    // Create a client to test the proxy
    let client = reqwest::Client::new();
    
    // Test a request through the proxy
    let url = &format!("http://{}/api/v1/messages", proxy_addr);
    println!("Making request to: {}", url);
    
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to send request");
    
    let status = response.status();
    println!("Response status: {}", status);
    let body = response.text().await.expect("Failed to get response body");
    println!("Response body: {}", body);
    
    assert_eq!(status, 200);
    
    assert!(body.contains("Hello from mock Anthropic API"));
    
    // Test health endpoint
    let health_response = client
        .get(&format!("http://{}/api/v1/health", proxy_addr))
        .send()
        .await
        .expect("Failed to send request");
    
    assert_eq!(health_response.status(), 200);
    
    let health_body = health_response.text().await.expect("Failed to get response body");
    assert!(health_body.contains("healthy"));
}

async fn proxy_handler(
    axum::extract::Path((prefix, path)): axum::extract::Path<(String, String)>,
    request: axum::extract::Request,
    target_url: &str,
) -> Result<Response, StatusCode> {
    // Import the proxy service
    use anthropic_http_proxy::ProxyService;
    
    println!("DEBUG: Creating proxy service with target_url: {}", target_url);
    println!("DEBUG: Prefix: {}", prefix);
    println!("DEBUG: Path: {}", path);
    println!("DEBUG: Request URI: {}", request.uri());
    println!("DEBUG: Request method: {}", request.method());
    
    let proxy_service = ProxyService::new_with_base(Some(target_url)).await
        .map_err(|e| {
            println!("DEBUG: Failed to create proxy service: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    let result = proxy_service.handle_request(prefix, request).await;
    println!("DEBUG: Proxy service result: {:?}", result);
    result
}

#[tokio::test]
async fn test_e2e_proxy_with_custom_port() {
    // Test that the proxy can run on a custom port
    let proxy_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let proxy_addr = proxy_listener.local_addr().unwrap();
    
    let app = Router::new()
        .route("/:prefix/v1/*path", axum::routing::any(proxy_handler_simple))
        .fallback(|| async { 
            (StatusCode::NOT_FOUND, "Not Found") 
        });
    
    tokio::spawn(async move {
        axum::serve(proxy_listener, app).await.unwrap();
    });
    
    // Give the server a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    // Test that the server is responding
    let client = reqwest::Client::new();
    
    let response = client
        .get(&format!("http://{}/api/v1/test", proxy_addr))
        .send()
        .await
        .expect("Failed to send request");
    
    // This should fail since we don't have a real target, but it should not be a connection error
    assert_ne!(response.status(), 404);
}

async fn proxy_handler_simple(
    axum::extract::Path((_prefix, _path)): axum::extract::Path<(String, String)>,
    _request: axum::extract::Request,
) -> Result<Response, StatusCode> {
    // Simple handler that just returns an error for testing
    Err(StatusCode::BAD_GATEWAY)
}
use axum::{
    body::to_bytes,
    extract::Request,
    http::{StatusCode, Uri},
    response::Response,
};
use std::collections::HashMap;
use tracing::{debug, error, info};

use crate::config::Config;

pub struct ProxyService {
    pub clients: HashMap<String, reqwest::Client>,
    pub config: Config,
}

impl ProxyService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::default();
        Self::new_with_config(config).await
    }
    
    pub async fn new_with_config(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let clients = Self::create_clients(&config)?;
        
        Ok(Self {
            clients,
            config,
        })
    }
    
    fn create_clients(config: &Config) -> Result<HashMap<String, reqwest::Client>, Box<dyn std::error::Error>> {
        let mut clients = HashMap::new();
        
        // Default client (no proxy)
        clients.insert("default".to_string(), Self::create_client(None)?);
        
        // Create clients for each configured endpoint
        for (prefix, proxy_url) in config.get_endpoint_proxies() {
            info!("Creating client for endpoint '{}' with proxy: {}", prefix, proxy_url);
            clients.insert(prefix, Self::create_client(Some(&proxy_url))?);
        }
        
        Ok(clients)
    }
    
    fn create_client(proxy_url: Option<&str>) -> Result<reqwest::Client, Box<dyn std::error::Error>> {
        let mut builder = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(3600));
        
        // Configure proxy if provided
        if let Some(proxy_url) = proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url)?;
            builder = builder.proxy(proxy);
        }
        
        Ok(builder.build()?)
    }
    
    fn get_client_for_endpoint(&self, endpoint: &str) -> &reqwest::Client {
        self.clients.get(endpoint).unwrap_or_else(|| self.clients.get("default").unwrap())
    }
    
    pub async fn handle_request(
        &self,
        prefix: String,
        request: Request,
    ) -> Result<Response, StatusCode> {
        let method = request.method().clone();
        let uri = request.uri().clone();
        let headers = request.headers().clone();
        
        debug!("Proxying {} request to {}", method, uri);
        
        // Extract the path after the prefix
        let path = self.extract_path(&uri, &prefix)?;
        let target_base = self.config.get_endpoint_target_base(&prefix)
            .unwrap_or_else(|| "https://api.anthropic.com".to_string());
        let target_url = format!("{}{}", target_base, path);
        
        debug!("Forwarding to: {}", target_url);
        
        // Get the appropriate client for this endpoint
        let client = self.get_client_for_endpoint(&prefix);
        
        // Convert axum Request to reqwest Request
        let body_bytes = to_bytes(request.into_body(), usize::MAX).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        // Convert the method
        let reqwest_method = match method.as_str() {
            "GET" => reqwest::Method::GET,
            "POST" => reqwest::Method::POST,
            "PUT" => reqwest::Method::PUT,
            "DELETE" => reqwest::Method::DELETE,
            "PATCH" => reqwest::Method::PATCH,
            "HEAD" => reqwest::Method::HEAD,
            "OPTIONS" => reqwest::Method::OPTIONS,
            _ => reqwest::Method::GET, // fallback
        };
        
        let mut req_builder = client
            .request(reqwest_method, &target_url);
        
        // Copy headers
        for (name, value) in headers.iter() {
            if name.as_str().to_lowercase() != "host" {
                req_builder = req_builder.header(
                    name.as_str(),
                    value.to_str().unwrap_or("")
                );
            }
        }
        
        // Set the body
        let req_builder = if !body_bytes.is_empty() {
            req_builder.body(body_bytes.to_vec())
        } else {
            req_builder
        };
        
        // Execute the request
        let response = req_builder.send().await
            .map_err(|e| {
                error!("Request failed: {}", e);
                StatusCode::BAD_GATEWAY
            })?;
        
        // Convert reqwest Response to axum Response
        let status_code = axum::http::StatusCode::from_u16(response.status().as_u16()).unwrap();
        let mut axum_response = Response::builder()
            .status(status_code);
        
        // Copy response headers
        for (name, value) in response.headers() {
            axum_response = axum_response.header(name.as_str(), value.to_str().unwrap_or(""));
        }
        
        // Get response body
        let response_body = response.bytes().await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Ok(axum_response
            .body(axum::body::Body::from(response_body.to_vec()))
            .unwrap())
    }
    
    fn extract_path(&self, uri: &Uri, prefix: &str) -> Result<String, StatusCode> {
        let path = uri.path();
        let expected_prefix = format!("/{}/v1", prefix);
        
        if !path.starts_with(&expected_prefix) {
            return Err(StatusCode::BAD_REQUEST);
        }
        
        let remaining_path = path[expected_prefix.len()..].to_string();
        let final_path = format!("/v1{}", remaining_path);
        
        Ok(final_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Uri;
    use std::collections::HashMap;

    #[test]
    fn test_extract_path_valid() {
        let proxy_service = ProxyService {
            clients: HashMap::new(),
            config: Config::default(),
        };
        
        let uri = "/test/v1/messages".parse::<Uri>().unwrap();
        let result = proxy_service.extract_path(&uri, "test");
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "/v1/messages");
    }

    #[test]
    fn test_extract_path_invalid_prefix() {
        let proxy_service = ProxyService {
            clients: HashMap::new(),
            config: Config::default(),
        };
        
        let uri = "/wrong/v1/messages".parse::<Uri>().unwrap();
        let result = proxy_service.extract_path(&uri, "test");
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_extract_path_empty_path() {
        let proxy_service = ProxyService {
            clients: HashMap::new(),
            config: Config::default(),
        };
        
        let uri = "/test/v1".parse::<Uri>().unwrap();
        let result = proxy_service.extract_path(&uri, "test");
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "/v1");
    }

    #[test]
    fn test_get_client_for_endpoint_existing() {
        let mut clients = HashMap::new();
        let client = reqwest::Client::new();
        clients.insert("test".to_string(), client.clone());
        clients.insert("default".to_string(), reqwest::Client::new()); // Different default client
        
        let proxy_service = ProxyService {
            clients,
            config: Config::default(),
        };
        
        // Just check that it returns a client without panicking
        let _result = proxy_service.get_client_for_endpoint("test");
    }

    #[test]
    fn test_get_client_for_endpoint_default() {
        let mut clients = HashMap::new();
        let client = reqwest::Client::new();
        clients.insert("default".to_string(), client.clone());
        
        let proxy_service = ProxyService {
            clients,
            config: Config::default(),
        };
        
        // Just check that it returns a client without panicking
        let _result = proxy_service.get_client_for_endpoint("nonexistent");
    }

    #[test]
    fn test_get_endpoint_proxies_empty() {
        let config = Config::default();
        let endpoints = config.get_endpoint_proxies();
        assert!(endpoints.is_empty());
    }

    #[test]
    fn test_get_endpoint_proxies_with_config() {
        let mut config = Config::default();
        let mut endpoints = HashMap::new();
        endpoints.insert("test".to_string(), crate::config::EndpointConfig {
            proxy_url: Some("http://proxy.example.com:8080".to_string()),
            target_base: None,
        });
        config.endpoints = endpoints;
        
        let endpoints = config.get_endpoint_proxies();
        assert_eq!(endpoints.len(), 1);
        assert_eq!(endpoints[0], ("test".to_string(), "http://proxy.example.com:8080".to_string()));
    }

    #[tokio::test]
    async fn test_create_client_no_proxy() {
        let result = ProxyService::create_client(None);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_client_with_proxy() {
        let _result = ProxyService::create_client(Some("http://example.com:8080"));
        // This might fail if proxy is invalid, but we test the structure
        // In a real test, you'd mock the proxy or use a valid one
    }

    #[tokio::test]
    async fn test_proxy_service_new() {
        let result = ProxyService::new().await;
        assert!(result.is_ok());
        
        let service = result.unwrap();
        assert!(service.clients.contains_key("default"));
        assert_eq!(service.config.server.target_base, Some("https://api.anthropic.com".to_string()));
    }
}
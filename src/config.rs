use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub endpoints: HashMap<String, EndpointConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub port: Option<u16>,
    pub target_base: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EndpointConfig {
    pub proxy_url: Option<String>,
    pub target_base: Option<String>,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn get_endpoint_proxies(&self) -> Vec<(String, String)> {
        let mut endpoints = Vec::new();
        
        for (name, config) in &self.endpoints {
            if let Some(proxy_url) = &config.proxy_url {
                endpoints.push((name.clone(), proxy_url.clone()));
            }
        }
        
        endpoints
    }

    pub fn get_endpoint_target_base(&self, endpoint: &str) -> Option<String> {
        self.endpoints
            .get(endpoint)
            .and_then(|config| config.target_base.clone())
            .or_else(|| self.server.target_base.clone())
    }
}

impl Default for Config {
    fn default() -> Self {
        let endpoints = HashMap::new();
        
        Self {
            server: ServerConfig {
                port: Some(8811),
                target_base: Some("https://api.anthropic.com".to_string()),
            },
            endpoints,
        }
    }
}
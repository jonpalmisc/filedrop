use std::{net::SocketAddr, path::PathBuf, str::FromStr};

use config::{Config, ConfigError};
use serde::Deserialize;

/// Program configuration.
#[derive(Debug, Deserialize)]
pub struct Settings {
    /// Address to listen on.
    bind_address: String,

    /// Port to listen on.
    bind_port: u32,

    /// Domain name this server is accessible from.
    server_name: String,

    /// Path to store uploaded files in.
    storage_path: String,

    /// Maximum request size.
    max_request_size: usize,
}

impl Settings {
    /// Load the config from disk or use the default.
    pub fn load() -> Result<Self, ConfigError> {
        match Config::builder()
            .add_source(config::File::with_name("filedrop"))
            .add_source(config::Environment::with_prefix("FILEDROP"))
            .build()
        {
            Ok(config) => config.try_deserialize(),
            _ => Ok(Self::default()),
        }
    }

    /// Get the listen address string (with port).
    pub fn listen_string(&self) -> String {
        format!("{}:{}", self.bind_address, self.bind_port)
    }

    /// Get the server name string (with port if non-standard).
    pub fn server_string(&self) -> String {
        if self.bind_port == 80 {
            self.server_name.clone()
        } else {
            format!("{}:{}", self.server_name, self.bind_port)
        }
    }

    /// Get the configured listening address.
    pub fn listen_addr(&self) -> Result<SocketAddr, <SocketAddr as FromStr>::Err> {
        self.listen_string().parse()
    }

    /// Get the storage directory as a `Path`.
    pub fn storage_path(&self) -> PathBuf {
        PathBuf::from(&self.storage_path)
    }

    pub fn max_request_size(&self) -> usize {
        self.max_request_size
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1".to_string(),
            bind_port: 8000,
            storage_path: "storage".to_string(),
            server_name: "localhost".to_string(),
            max_request_size: 104857600,
        }
    }
}

use std::{env, net::SocketAddr, path::PathBuf, str::FromStr};

use serde::Deserialize;

/// Program configuration.
#[derive(Debug, Deserialize)]
pub struct Settings {
    /// IP address to listen on.
    ip: String,

    /// Port to listen on.
    port: u32,

    /// Domain name this server is accessible from.
    host: String,

    /// Path to store uploaded files in.
    storage_path: String,

    /// Maximum request size.
    size_limit: usize,
}

/// Grouped environment variable name constants.
pub struct Variable;

impl Variable {
    pub const IP_KEY: &str = "FILEDROP_IP";
    pub const PORT_KEY: &str = "FILEDROP_PORT";
    pub const HOST_KEY: &str = "FILEDROP_HOST";
    pub const STORAGE_KEY: &str = "FILEDROP_STORAGE";
    pub const SIZE_LIMIT_KEY: &str = "FILEDROP_SIZE_LIMIT";
}

/// Get the value of an environment variable as a string.
macro_rules! env_str {
    ($name:expr, $default:expr) => {
        env::var($name).unwrap_or($default.to_string())
    };
}

/// Get the value of an environment variable as a number.
macro_rules! env_num {
    ($name:expr, $default:expr) => {
        env::var($name)
            .unwrap_or("".to_string())
            .parse()
            .unwrap_or($default)
    };
}

impl Settings {
    /// Load the config from the environment; missing or unspecified options are
    /// set to their defaults.
    pub fn from_env() -> Self {
        Self {
            ip: env_str!(Variable::IP_KEY, "127.0.0.1"),
            port: env_num!(Variable::PORT_KEY, 8000),
            host: env_str!(Variable::HOST_KEY, "localhost"),
            storage_path: env_str!(Variable::STORAGE_KEY, "storage"),
            size_limit: env_num!(Variable::SIZE_LIMIT_KEY, 50 * 1024 * 1024 /* 50 MiB */),
        }
    }

    /// Get the listen address string (with port).
    pub fn listen_string(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    /// Get the host name string (with port if non-standard).
    pub fn host_string(&self) -> String {
        if self.port == 80 {
            self.host.clone()
        } else {
            format!("{}:{}", self.host, self.port)
        }
    }

    /// Get the configured listening address.
    pub fn listen_address(&self) -> Result<SocketAddr, <SocketAddr as FromStr>::Err> {
        self.listen_string().parse()
    }

    /// Get the storage directory as a `Path`.
    pub fn storage_path(&self) -> PathBuf {
        PathBuf::from(&self.storage_path)
    }

    /// Get the request size limit.
    pub fn size_limit(&self) -> usize {
        self.size_limit
    }
}

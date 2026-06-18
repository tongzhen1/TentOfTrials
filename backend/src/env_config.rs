use std::env;

const DEFAULT_BACKEND_HOST: &str = "0.0.0.0";
const DEFAULT_BACKEND_PORT: u16 = 8080;
const DEFAULT_LOG_LEVEL: &str = "info";
const DEFAULT_ENABLE_EXPERIMENTAL: bool = false;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub log_level: String,
    pub enable_experimental: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("TOT_BACKEND_PORT must be a valid TCP port between 1 and 65535, got `{0}`")]
    InvalidPort(String),
    #[error("TOT_ENABLE_EXPERIMENTAL must be `true` or `false`, got `{0}`")]
    InvalidBool(String),
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let host = env::var("TOT_BACKEND_HOST")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| DEFAULT_BACKEND_HOST.to_string());
        let port = env::var("TOT_BACKEND_PORT")
            .map(|value| parse_port(&value))
            .unwrap_or(Ok(DEFAULT_BACKEND_PORT))?;
        let log_level = env::var("TOT_LOG_LEVEL")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| DEFAULT_LOG_LEVEL.to_string());
        let enable_experimental = env::var("TOT_ENABLE_EXPERIMENTAL")
            .map(|value| parse_bool(&value))
            .unwrap_or(Ok(DEFAULT_ENABLE_EXPERIMENTAL))?;

        Ok(Self {
            host,
            port,
            log_level,
            enable_experimental,
        })
    }
}

fn parse_port(value: &str) -> Result<u16, ConfigError> {
    match value.parse::<u16>() {
        Ok(0) | Err(_) => Err(ConfigError::InvalidPort(value.to_string())),
        Ok(port) => Ok(port),
    }
}

fn parse_bool(value: &str) -> Result<bool, ConfigError> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(ConfigError::InvalidBool(value.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());
    const ENV_KEYS: [&str; 4] = [
        "TOT_BACKEND_HOST",
        "TOT_BACKEND_PORT",
        "TOT_LOG_LEVEL",
        "TOT_ENABLE_EXPERIMENTAL",
    ];

    fn with_clean_env(test: impl FnOnce()) {
        let _guard = ENV_LOCK.lock().expect("env lock poisoned");
        let saved: Vec<_> = ENV_KEYS
            .iter()
            .map(|key| (*key, env::var(key).ok()))
            .collect();

        for key in ENV_KEYS {
            env::remove_var(key);
        }

        test();

        for (key, value) in saved {
            match value {
                Some(value) => env::set_var(key, value),
                None => env::remove_var(key),
            }
        }
    }

    #[test]
    fn from_env_uses_safe_defaults() {
        with_clean_env(|| {
            let config = Config::from_env().expect("default config should load");

            assert_eq!(config.host, DEFAULT_BACKEND_HOST);
            assert_eq!(config.port, DEFAULT_BACKEND_PORT);
            assert_eq!(config.log_level, DEFAULT_LOG_LEVEL);
            assert!(!config.enable_experimental);
        });
    }

    #[test]
    fn from_env_accepts_valid_overrides() {
        with_clean_env(|| {
            env::set_var("TOT_BACKEND_HOST", "127.0.0.1");
            env::set_var("TOT_BACKEND_PORT", "9090");
            env::set_var("TOT_LOG_LEVEL", "debug");
            env::set_var("TOT_ENABLE_EXPERIMENTAL", "true");

            let config = Config::from_env().expect("overridden config should load");

            assert_eq!(config.host, "127.0.0.1");
            assert_eq!(config.port, 9090);
            assert_eq!(config.log_level, "debug");
            assert!(config.enable_experimental);
        });
    }

    #[test]
    fn from_env_rejects_invalid_port() {
        with_clean_env(|| {
            env::set_var("TOT_BACKEND_PORT", "70000");

            let err = Config::from_env().expect_err("invalid port should fail");

            assert!(err.to_string().contains("TOT_BACKEND_PORT"));
            assert!(err.to_string().contains("70000"));
        });
    }

    #[test]
    fn from_env_rejects_invalid_bool() {
        with_clean_env(|| {
            env::set_var("TOT_ENABLE_EXPERIMENTAL", "yes");

            let err = Config::from_env().expect_err("invalid bool should fail");

            assert!(err.to_string().contains("TOT_ENABLE_EXPERIMENTAL"));
            assert!(err.to_string().contains("yes"));
        });
    }
}

use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            database_url: required("DATABASE_URL")?,
            port: optional("PORT", 8080)?,
        })
    }
}

fn required(key: &'static str) -> Result<String, ConfigError> {
    env::var(key).map_err(|_| ConfigError::Missing(key))
}

fn optional<T>(key: &'static str, default: T) -> Result<T, ConfigError>
where
    T: std::str::FromStr,
{
    match env::var(key) {
        Ok(value) => value.parse::<T>().map_err(|_| ConfigError::Invalid(key)),
        Err(env::VarError::NotPresent) => Ok(default),
        Err(_) => Err(ConfigError::Invalid(key)),
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Missing(&'static str),
    Invalid(&'static str),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Missing(key) => {
                write!(f, "missing required environment variable: {key}")
            }
            ConfigError::Invalid(key) => {
                write!(f, "invalid value for environment variable: {key}")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

#![allow(dead_code)]
use serde::Deserialize;
use once_cell::sync::OnceCell;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct CratisConfig {
    pub client: ClientConfig,
    pub backup: BackupConfig,
    pub server: ServerConfig,
    pub advanced: Option<AdvancedConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ClientConfig {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct BackupConfig {
    pub mode: BackupMode,
    pub watch_directories: Vec<String>,
    pub exclude: Option<Vec<String>>,
    pub interval_seconds: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BackupMode {
    Full,
    Incremental,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub auth_token: String
}

#[derive(Debug, Deserialize)]
pub struct AdvancedConfig {
    pub max_file_size_mb: Option<u64>,
    pub retry_attempts: Option<u32>,
    pub retry_delay_seconds: Option<u64>,
    pub enable_notifications: Option<bool>
}

static CONFIG: OnceCell<CratisConfig> = OnceCell::new();

/// Loads the application configuration from a YAML file and initializes the global configuration.
///
/// Reads the configuration file at the specified path, parses its contents as YAML into a `CratisConfig` instance, and stores it in the global configuration container. Panics if the file cannot be read, the YAML is invalid, or the configuration has already been initialized.
///
/// # Examples
///
/// ```ignore
/// load_config("config.yaml");
/// let config = get_config();
/// assert_eq!(config.client.name, "example-client");
/// ```
pub fn load_config(path: &str) {
    let contents = fs::read_to_string(path).expect("Failed to read config file");
    let parsed: CratisConfig = serde_yaml::from_str(&contents).expect("Invalid config format");
    CONFIG.set(parsed).expect("Config already initialized");
}

/// Returns a reference to the globally loaded application configuration.
///
/// Panics if the configuration has not been initialized with `load_config`.
///
/// # Examples
///
/// ```ignore
/// // Initialize configuration once at startup
/// load_config("config.yaml");
///
/// // Access configuration anywhere after initialization
/// let config = get_config();
/// assert_eq!(config.client.name, "example-client");
/// ```
pub fn get_config() -> &'static CratisConfig {
    CONFIG.get().expect("Config not initialized")
}
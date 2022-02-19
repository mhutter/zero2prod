use config::{Config, ConfigError, Environment, File};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::{
    postgres::{PgConnectOptions, PgSslMode},
    ConnectOptions,
};
use tracing::log::LevelFilter;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub name: String,
    pub username: String,
    pub password: Secret<String>,
    pub require_ssl: bool,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = match self.require_ssl {
            true => PgSslMode::Require,
            false => PgSslMode::Prefer,
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
    pub fn with_db(&self) -> PgConnectOptions {
        let mut options = self.without_db().database(&self.name);
        options.log_statements(LevelFilter::Trace);
        options
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let base_path = std::env::current_dir().expect("Determine current directory");
        let config_dir = base_path.join("configuration");

        // Detect runtime environment. Default to `local`.
        let environment = std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "local".into());

        let s = Config::builder()
            // Read the "default" configuration file
            .add_source(File::from(config_dir.join("base")).required(true))
            // Load env-specific config
            .add_source(File::from(config_dir.join(environment)).required(true))
            // Add settings from ENV vars
            .add_source(Environment::with_prefix("app").separator("__"))
            .build()?;

        s.try_deserialize()
    }
}

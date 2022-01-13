use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub username: String,
    pub password: Secret<String>,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
        ))
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Determine current directory");
    let config_dir = base_path.join("configuration");

    // Read the "default" configuration file
    settings.merge(config::File::from(config_dir.join("base")).required(true))?;

    // Detect runtime environment. Default to `local`.
    let environment = std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "local".into());

    // Load env-specific config
    settings.merge(config::File::from(config_dir.join(environment)).required(true))?;

    settings.try_into()
}

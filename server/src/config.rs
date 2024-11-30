use config::builder::DefaultState;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub url: String,
}

pub fn config() -> Result<AppConfig, config::ConfigError> {
    config::ConfigBuilder::<DefaultState>::default()
        .set_default("port", 8080)?
        .set_default("url", "sqlite::memory:")?
        .add_source(config::File::with_name("config"))
        .build()?
        .try_deserialize()
}
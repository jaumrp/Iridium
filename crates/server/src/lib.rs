use std::env;

pub use log;
pub use tokio;

use time::macros::format_description;

#[derive(Debug)]
pub struct Config {
    pub address: String,
    pub port: u16,
    pub id: String,
}

impl Config {
    pub fn load() -> Self {
        let _ = dotenvy::dotenv();

        Self {
            address: env::var("IRIDIUM_ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("IRIDIUM_PORT")
                .unwrap_or_else(|_| "25565".to_string())
                .parse()
                .unwrap_or(25565),
            id: env::var("SERVER_ID").unwrap_or_else(|_| "iridium".to_string()),
        }
    }
}

pub fn init_logging() {
    simple_logger::SimpleLogger::new()
        .with_colors(true)
        .with_level(log::LevelFilter::Debug)
        .with_timestamp_format(format_description!("[year]-[month]-[day] [hour]:[minute]"))
        .init()
        .expect("failed to initialize logging");
}

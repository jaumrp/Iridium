pub mod iridium_server;
use std::{env, fs, path::Path, sync::Arc};

use ahash::AHashMap;
use events::EventBus;
pub use log;
use serde::{Deserialize, Serialize};
pub use tokio;

use time::macros::format_description;

#[derive(Serialize, Clone)]
pub struct ServerContext {
    pub path: String,
    pub config: ServerConfig,

    #[serde(skip)]
    pub event_bus: Arc<events::EventBus<ServerContext>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub file_path: String,
    pub config: AHashMap<String, serde_yaml::Value>,
}

impl ServerConfig {
    pub fn new(file_path: String) -> Self {
        let config = AHashMap::new();
        Self { file_path, config }
    }
    pub async fn load_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !Path::new(&self.file_path).exists() {
            self.set("server.host", "0.0.0.0");
            self.set("server.port", 25565);
            self.save()?;
            return Ok(());
        }
        let content = fs::read_to_string(&self.file_path)?;
        let parse_config: serde_yaml::Value = serde_yaml::from_str(&content)?;

        let mut new_config = AHashMap::new();

        Self::flattern_value("", &parse_config, &mut new_config);
        self.config = new_config;

        Ok(())
    }

    pub fn flattern_value(
        prefix: &str,
        value: &serde_yaml::Value,
        map: &mut AHashMap<String, serde_yaml::Value>,
    ) {
        if let Some(mapping) = value.as_mapping() {
            for (key, value) in mapping {
                if let Some(key_str) = key.as_str() {
                    let new_key = if prefix.is_empty() {
                        key_str.to_string()
                    } else {
                        format!("{}.{}", prefix, key_str)
                    };
                    Self::flattern_value(&new_key, value, map);
                }
            }
        } else {
            map.insert(prefix.to_string(), value.clone());
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut root = serde_yaml::Mapping::new();

        for (key, value) in &self.config {
            let parts: Vec<&str> = key.split('.').collect();
            let mut current_map = &mut root;

            for i in 0..parts.len() {
                let part = parts[i];
                if i == parts.len() - 1 {
                    current_map.insert(serde_yaml::Value::String(part.to_string()), value.clone());
                } else {
                    let key_value = serde_yaml::Value::String(part.to_string());

                    if !current_map.contains_key(&key_value)
                        || !current_map.get(&key_value).unwrap().is_mapping()
                    {
                        current_map.insert(
                            key_value.clone(),
                            serde_yaml::Value::Mapping(serde_yaml::Mapping::new()),
                        );
                    }
                    current_map = current_map
                        .get_mut(&key_value)
                        .and_then(|v| v.as_mapping_mut())
                        .unwrap();
                }
            }
        }

        let content = serde_yaml::to_string(&root)?;
        fs::write(&self.file_path, content)?;

        Ok(())
    }

    pub fn set(&mut self, key: &str, value: impl Into<serde_yaml::Value>) {
        self.config.insert(key.to_string(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&serde_yaml::Value> {
        self.config.get(key)
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.config.get(key).and_then(|v| v.as_str())
    }

    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.config.get(key).and_then(|v| v.as_i64())
    }
}

impl ServerContext {
    pub fn new() -> Self {
        let path = env::current_dir()
            .unwrap_or_else(|_| ".".into())
            .to_string_lossy()
            .into_owned();

        let mut config_path = path.clone();
        config_path.push_str("/server.yml");

        let config = ServerConfig::new(config_path.to_owned());

        let event_bus = Arc::new(EventBus::new());

        Self {
            path,
            config,
            event_bus: event_bus.clone(),
        }
    }
}

pub fn init_logging() {
    simple_logger::SimpleLogger::new()
        .with_colors(true)
        .with_level(log::LevelFilter::Info)
        .with_timestamp_format(format_description!("[year]-[month]-[day] [hour]:[minute]"))
        .init()
        .expect("failed to initialize logging");
}

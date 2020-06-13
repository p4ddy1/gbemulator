use crate::config::config::Config;
use std::error::Error;
use std::fs;
use std::io::ErrorKind;
use std::sync::{Arc, RwLock};

pub struct ConfigStorage {
    pub config: Arc<RwLock<Config>>,
}

impl ConfigStorage {
    pub fn create_empty() -> Self {
        ConfigStorage {
            config: Arc::new(RwLock::new(Config::default())),
        }
    }

    pub fn create_from_file(filename: &str) -> Result<Self, String> {
        let content = match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                if let ErrorKind::NotFound = e.kind() {
                    let empty_storage = Self::create_empty();
                    empty_storage.save_to_file(filename)?;
                    return Ok(empty_storage);
                }

                return Err(format!("Error loading config file: {:?}", e));
            }
        };

        let config = match Config::create_from_toml(content.as_str()) {
            Ok(config) => config,
            Err(e) => return Err(format!("Error parsing toml config: {:?}", e)),
        };

        Ok(ConfigStorage {
            config: Arc::new(RwLock::new(config)),
        })
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), String> {
        let toml = match self.config.read().unwrap().to_toml() {
            Ok(toml) => toml,
            Err(e) => return Err(format!("Error serializing config: {:?}", e)),
        };

        match fs::write(filename, toml) {
            Ok(_) => {}
            Err(e) => return Err(format!("Error saving config to file: {:?}", e)),
        };

        Ok(())
    }
}

use crate::core::app_model::AppModel;

use serde::{Deserialize, Serialize};

use std::fs;
use std::path::Path;

pub const CONFIG_FILE: &str = "config/dock.json";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DockConfig {
    pub apps: Vec<AppModel>,
}

pub struct ConfigManager;

impl ConfigManager {
    pub fn load(
        path: &str,
    ) -> Result<DockConfig, Box<dyn std::error::Error>> {
        if !Path::new(path).exists() {
            return Ok(DockConfig::default());
        }

        let content = fs::read_to_string(path)?;

        let config: DockConfig =
            serde_json::from_str(&content)?;

        Ok(config)
    }

    pub fn save(
        path: &str,
        config: &DockConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(path);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json =
            serde_json::to_string_pretty(config)?;

        fs::write(path, json)?;

        Ok(())
    }

    pub fn load_default(
    ) -> Result<DockConfig, Box<dyn std::error::Error>> {
        Self::load(CONFIG_FILE)
    }

    pub fn save_default(
        config: &DockConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Self::save(CONFIG_FILE, config)
    }
}
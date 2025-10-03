// SPDX-License-Identifier: GPL-3.0-only

use std::path::Path;
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub gitlab: GitLabConfig,
    pub app: AppConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitLabConfig {
    pub url: String,
    pub access_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub refresh_interval: u64,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        
        if !path.exists() {
            anyhow::bail!(
                "Configuration file '{}' not found. Please copy config.yaml.example to config.yaml and customize it",
                path.display()
            );
        }

        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        
        let config: Config = serde_yaml::from_str(&content)
            .with_context(|| "Failed to parse YAML configuration")?;

        // Validate configuration
        if config.gitlab.access_token == "your-token-here" || config.gitlab.access_token.is_empty() {
            anyhow::bail!("Please set a valid GitLab access token in config.yaml");
        }

        if config.gitlab.url.is_empty() {
            anyhow::bail!("Please set a valid GitLab URL in config.yaml");
        }

        Ok(config)
    }

    pub fn default_path() -> &'static str {
        "config.yaml"
    }
}
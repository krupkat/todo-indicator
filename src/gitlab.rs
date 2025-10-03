// SPDX-License-Identifier: GPL-3.0-only

use anyhow::{Result, Context};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct GitLabResponse {
    // We don't need the actual TODO data, just the headers
}

#[derive(Clone)]
pub struct GitLabClient {
    client: Client,
    base_url: String,
    access_token: String,
}

impl GitLabClient {
    pub fn new(base_url: String, access_token: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url,
            access_token,
        }
    }

    pub async fn fetch_todo_count(&self) -> Result<u32> {
        let url = format!("{}/api/v4/todos", self.base_url);
        
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .query(&[("state", "pending"), ("per_page", "1")])
            .send()
            .await
            .with_context(|| "Failed to send request to GitLab API")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "GitLab API request failed with status: {} - {}",
                response.status(),
                response.status().canonical_reason().unwrap_or("Unknown error")
            );
        }

        // Get total count from headers
        let total_count = response
            .headers()
            .get("X-Total")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);

        Ok(total_count)
    }

    pub fn get_todos_url(&self) -> String {
        format!("{}/dashboard/todos", self.base_url)
    }
}
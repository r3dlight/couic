use async_trait::async_trait;
use chrono::Utc;
use reqwest::Client;
use std::time::Duration;
use tracing::{error, info};

use crate::{
    config::Thresholds,
    notifier::{Notifier, NotifyError, NotifyResult},
    stats::Statistics,
};

const CLIENT_TIMEOUT: u64 = 5;

pub struct DiscordNotifier {
    webhook_url: String,
    client: Client,
    server_name: String,
    batch_interval_secs: u64,
    thresholds: Thresholds,
}

impl DiscordNotifier {
    pub fn new(
        webhook_url: String,
        server_name: String,
        batch_interval_secs: u64,
        thresholds: Thresholds,
    ) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(CLIENT_TIMEOUT))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            webhook_url,
            client,
            server_name,
            batch_interval_secs,
            thresholds,
        }
    }

    fn format_period(&self) -> String {
        let secs = self.batch_interval_secs;
        if secs.is_multiple_of(3600) {
            format!("{}-hour", secs / 3600)
        } else if secs.is_multiple_of(60) {
            format!("{}-minute", secs / 60)
        } else {
            format!("{}-second", secs)
        }
    }

    fn get_color(&self, count: usize) -> u32 {
        if count < self.thresholds.orange {
            0x00FF00 // Green
        } else if count < self.thresholds.red {
            0xFF9500 // Orange
        } else {
            0xFF0000 // Red
        }
    }
}

#[async_trait]
impl Notifier for DiscordNotifier {
    async fn send_statistics(&self, stats: &Statistics) -> NotifyResult<()> {
        let top_tag_text = if let Some((tag, count)) = &stats.top_tag {
            format!("**{}** (×{})", tag, count)
        } else {
            "N/A".to_string()
        };

        let timestamp = Utc::now().to_rfc3339();
        let period = self.format_period();
        let color = self.get_color(stats.total_count);

        let embeds = vec![serde_json::json!({
            "title": format!("📊 {} Report Summary", period),
            "timestamp": timestamp,
            "color": color,
            "author": { "name": format!("Server: {}", self.server_name) },
            "thumbnail": {
                "url": format!("https://couic.net/images/{}",if color == 0xFF0000 {"notif-red-fire.png"} else if color == 0xFF9500 {"notif-orange.png"} else {"notif-green.png"})
            },
            "footer": {
                "text": "Couic Report",
                "icon_url": "https://couic.net/images/white-150.png"
            },
            "fields": [
                { "name": "Total CIDRs", "value": format!(":shield: **{}**", stats.total_count), "inline": true },
                { "name": "Distinct CIDRs", "value": format!(":mag: **{}**", stats.distinct_cidrs), "inline": true },
                { "name": "Top Tag", "value": format!(":satellite: {}", top_tag_text), "inline": true }
            ]
        })];

        let body = serde_json::json!({
            "username": "Couic",
            "embeds": embeds
        });

        let resp = self
            .client
            .post(&self.webhook_url)
            .json(&body)
            .send()
            .await
            .map_err(|e| NotifyError::Http(e.to_string()))?;

        if !resp.status().is_success() {
            error!("discord returned {:?}", resp.status());
            return Err(NotifyError::Http(format!(
                "discord returned {:?}",
                resp.status()
            )));
        }

        info!("Successful Discord notification sent");
        Ok(())
    }

    fn name(&self) -> &'static str {
        "discord"
    }
}

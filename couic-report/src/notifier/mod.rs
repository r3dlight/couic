use async_trait::async_trait;
use std::sync::Arc;

use crate::stats::Statistics;

pub mod discord;

#[derive(Debug, thiserror::Error)]
pub enum NotifyError {
    #[error("HTTP error: {0}")]
    Http(String),
}

pub type NotifyResult<T> = Result<T, NotifyError>;

#[async_trait]
pub trait Notifier: Send + Sync {
    async fn send_statistics(&self, stats: &Statistics) -> NotifyResult<()>;
    fn name(&self) -> &'static str;
}

pub struct NotificationDispatcher {
    notifiers: Vec<Arc<dyn Notifier>>,
}

impl NotificationDispatcher {
    pub fn new(notifiers: Vec<Arc<dyn Notifier>>) -> Self {
        Self { notifiers }
    }

    pub async fn dispatch(&self, stats: Statistics) {
        for notifier in &self.notifiers {
            if let Err(e) = notifier.send_statistics(&stats).await {
                tracing::error!("[{}] statistics failed: {}", notifier.name(), e);
            } else {
                tracing::info!("[{}] statistics sent successfully", notifier.name());
            }
        }
    }
}

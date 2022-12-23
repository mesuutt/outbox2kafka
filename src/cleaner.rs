use chrono::Utc;
use log::{error, info, warn};
use std::future::Future;
use std::ops::Sub;
use std::sync::Arc;
use std::time;
use std::time::Duration;

use crate::{AppError, AppResult, Repo};

pub struct OutboxCleaner {
    repo: Arc<Repo>,
    run_interval: time::Duration,
    retention: chrono::Duration, // processed record retention
}

impl OutboxCleaner {
    pub fn new(repo: Arc<Repo>, run_interval: time::Duration, retention: time::Duration) -> AppResult<Self> {
        if run_interval < Duration::from_secs(60) {
            warn!("outbox table cleaner run interval might be >= 1m");
        }

        let retention = chrono::Duration::from_std(retention)
            .map_err(|_| AppError::DurationParseError("outbox data retention period parse failed".to_string()))?;

        Ok(Self {
            repo,
            run_interval,
            retention,
        })
    }

    pub async fn run(&self, shutdown: impl Future) {
        tokio::select! {
            _ = self.run_forever() => {}
            _ = shutdown => {
                info!("cleaner shutting down");
            }
        }
    }

    async fn run_forever(&self) {
        let mut interval = tokio::time::interval(self.run_interval);
        loop {
            interval.tick().await;

            if let Err(e) = self.repo.delete_older_than(Utc::now().sub(self.retention)).await {
                error!("error occurred while deletion of old processed records: {}", e);
            }
        }
    }
}

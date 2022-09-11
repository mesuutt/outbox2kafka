use std::future::Future;
use std::ops::Sub;
use std::sync::Arc;
use std::time;
use std::time::Duration;
use chrono::Utc;
use log::{debug, error, info};
use tokio::time::sleep;

use crate::{AppError, AppResult, Repo};

pub struct OutboxCleaner {
    repo: Arc<Repo>,
    run_interval: time::Duration,
    retention: chrono::Duration, // processed record retention
}

impl OutboxCleaner {
    pub fn new(repo: Arc<Repo>, run_interval: time::Duration, retention: time::Duration) -> AppResult<Self> {
        if run_interval < Duration::from_secs(60) {
            return Err(AppError::DurationParseError("outbox table cleaner run interval must be >= 1m".to_string()));
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
        loop {
            if let Err(e) = self.repo.delete_older_than(Utc::now().sub(self.retention)).await {
                error!("error occurred while deletion of old processed records: {}", e);
            }
            debug!("old processed items deleted");
            sleep(self.run_interval).await;
        }
    }
}

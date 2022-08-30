use std::time::Duration;
use crate::db::DbPool;
use crate::Repo;

pub struct OutboxCleaner {
    repo: Repo,
    run_interval: Duration,
    retention: Duration, // processed record retention
}

impl OutboxCleaner {
    pub fn new(repo: Repo, run_interval: Duration, retention: Duration) -> Self {
        Self { repo, run_interval, retention }
    }

    pub fn run(&self) {
        todo!()
    }
}
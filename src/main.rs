extern crate core;

use std::sync::Arc;
use futures::future::join_all;
use log::{error, info};
use structopt::StructOpt;

mod cleaner;
mod cli;
mod db;
mod error;
mod model;
mod producer;
mod repo;

use crate::cli::Opt;
use crate::db::create_pool;
use crate::producer::{Producer};
use crate::repo::Repo;
use error::{AppError, AppResult};
use crate::cleaner::OutboxCleaner;
use tokio::signal;

#[tokio::main]
async fn main() -> AppResult<()> {
    env_logger::init();

    let opts = Opt::from_args();

    let db_pool = create_pool(opts.db_url, opts.max_db_connection).await?;
    let repo = Arc::new(Repo::new(db_pool, opts.processed_data_retention));

    let mut tasks = vec![];

    if !opts.processed_data_retention.is_zero() {
        let cleaner = OutboxCleaner::new(repo.clone(), opts.cleaner_run_interval, opts.processed_data_retention)?;
        let task = tokio::spawn(async move {
            info!("outbox table cleaner starting");
            cleaner.run(signal::ctrl_c()).await;
        });

        tasks.push(task);
    }

    for i in 0..opts.concurrency {
        let producer = Producer::new(opts.brokers.clone(), opts.topic.clone(), repo.clone(), opts.outbox_check_interval)?;
        let task = tokio::spawn(async move {
            info!("{}. producer worker starting", i);
            producer.run(signal::ctrl_c()).await;
        });

        tasks.push(task);
    }

    join_all(tasks).await;

    Ok(())
}
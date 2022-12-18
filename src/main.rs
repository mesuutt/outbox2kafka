extern crate core;

use env_logger::Env;
use futures::future::join_all;
use log::info;
use std::sync::Arc;
use structopt::StructOpt;

mod args;
mod cleaner;
mod db;
mod error;
mod model;
mod producer;
mod repo;

use crate::args::Args;
use crate::cleaner::OutboxCleaner;
use crate::db::create_pool;
use crate::producer::Producer;
use crate::repo::Repo;
use error::{AppError, AppResult};
use tokio::signal;

#[tokio::main]
async fn main() -> AppResult<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info,sqlx=error"));

    let args = Args::from_args();

    let db_pool = create_pool(args.db_url, args.max_db_connection).await?;
    let repo = Arc::new(Repo::new(db_pool, args.table_name, args.processed_data_retention));

    let mut tasks = vec![];

    if !args.processed_data_retention.is_zero() {
        let cleaner = OutboxCleaner::new(repo.clone(), args.cleaner_run_interval, args.processed_data_retention)?;

        let task = tokio::spawn(async move {
            info!("outbox table cleaner starting");
            cleaner.run(signal::ctrl_c()).await;
        });

        tasks.push(task);
    }

    for i in 0..args.concurrency {
        let producer =
            Producer::new(args.brokers.clone(), repo.clone(), args.outbox_check_interval)?;

        let task = tokio::spawn(async move {
            info!("{}. producer worker starting", i);
            producer.run(signal::ctrl_c()).await;
        });

        tasks.push(task);
    }

    join_all(tasks).await;

    Ok(())
}

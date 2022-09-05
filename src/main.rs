use std::sync::Arc;
use futures::future::join_all;
use log::error;
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
use crate::producer::Producer;
use crate::repo::Repo;
use error::{AppError, AppResult};

#[tokio::main]
async fn main() -> AppResult<()> {
    env_logger::init();

    let opts = Opt::from_args();

    let db_pool = create_pool(opts.db_url, opts.max_db_connection).await?;
    let repo = Arc::new(Repo::new(db_pool, opts.retention));

    let mut tasks = vec![];
    for _n in 0..opts.concurrency {
        let repo = repo.clone();
        let brokers = opts.brokers.clone();
        let topic = opts.topic.clone();

        let task = tokio::spawn(async move {
            match Producer::new(brokers, topic, repo, opts.db_check_interval) {
                Ok(p) => {
                    p.start().await;
                }
                Err(e) => error!("Producer start failed: {}", e),
            }
        });

        tasks.push(task);
    }

    join_all(tasks).await;

    Ok(())
}
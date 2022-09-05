use log::{debug, error, info, log_enabled, Level};
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

    let db_pool = create_pool(opts.db_url, opts.threads).await?;
    let repo = Repo::new(db_pool, opts.retention);

    let brokers = opts.brokers.split(",").into_iter().map(|x| x.to_string()).collect();
    let producer = Producer::new(brokers, opts.topic, repo, opts.db_check_interval)?;
    producer.start().await;

    Ok(())
}

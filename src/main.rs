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
    println!("{:?}", opts.clone());
    let db_pool = create_pool(opts.db_url).await?;
    let repo = Repo::new(db_pool, opts.retention);

    let producer = match Producer::new(opts.brokers, opts.topic, repo, opts.db_check_interval) {
        Ok(p) => p,
        Err(e) => return Err(e),
    };

    producer.start().await?;

    Ok(())
}

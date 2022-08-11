use std::env;
use std::io::Result;
use sqlx::postgres::PgPoolOptions;
use crate::error::AppError;
use structopt::StructOpt;

mod error;
mod outbox;
mod db;
mod kafka;
mod cli;

use error::AppResult;
use crate::cli::Opt;
use crate::db::create_pool;
use crate::outbox::Producer;

#[tokio::main]
async fn main() -> AppResult<()> {
    let opts = Opt::from_args();
    println!("{:?}", opts.clone());
    let db_pool = create_pool(opts.db_url).await?;
    let producer = Producer::new(db_pool, opts.brokers, opts.topic, opts.check_interval, opts.clean_after);
    producer.start().await?;

    // let consumer = kafka::
    Ok(())
}


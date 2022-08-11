use std::env;
use std::io::Result;
use sqlx::postgres::PgPoolOptions;
use crate::error::AppError;
use structopt::StructOpt;

mod error;
mod outbox;
mod db;
mod kafka;
mod args;

use error::AppResult;
use crate::args::Args;
use crate::db::create_pool;
use crate::outbox::Producer;

#[tokio::main]
async fn main() -> AppResult<()> {
    let args = Args::from_args();
    println!("{:?}", args.clone());
    let db_pool = create_pool(args.db_url).await?;
    let producer = Producer::new(db_pool);
    producer.start().await?;

    // let consumer = kafka::
    Ok(())
}


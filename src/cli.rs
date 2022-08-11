use std::error::Error;
use std::num::ParseIntError;
use structopt::StructOpt;
use std::time::Duration;
use crate::{AppError, AppResult};

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "outbox2kafka", about = "Read outbox table and send events to kafka")]
pub struct Opt {
    #[structopt(short, long, about = "the topic the messages were send")]
    pub topic: String,

    #[structopt(short, long, env = "DATABASE_URL")]
    pub db_url: String,

    #[structopt(short, long, default_value = "localhost:9092")]
    pub brokers: String,

    #[structopt(short, long, parse(try_from_str = parse_duration), default_value = "1s", about = "outbox table check interval, time units: mon,w,d,h,m,s,ms")]
    pub check_interval: Duration,

    #[structopt(long,  parse(try_from_str = parse_duration), default_value = "0ms", about = "clean sent events from outbox after time passed. 0 means never. If duration <= 1ms the events will be removed immediately. Supported time units: mon,w,d,h,m,s,ms")]
    pub clean_after: Duration,
}

fn parse_duration(src: &str) -> AppResult<Duration> {
    duration_str::parse(src).map_err(|_| AppError::CLIParseError("duration parse failed".to_string()))
}

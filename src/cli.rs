use crate::{AppError, AppResult};
use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "outbox2kafka", about = "Read outbox table and send events to kafka")]
pub struct Opt {
    #[structopt(short, long, env = "DATABASE_URL")]
    pub db_url: String,

    #[structopt(short, long, default_value = "localhost:9092", about = "Comma separated broker list")]
    pub brokers: String,

    #[structopt(long, about = "The topic the messages were send")]
    pub topic: String,

    #[structopt(short, long, default_value = "1", about = "number of workers to use")]
    pub concurrency: u32,


    #[structopt(short, long, default_value = "5", about = "max db connection to open")]
    pub max_db_connection: u32,

    #[structopt(short = "i", long, parse(try_from_str = parse_duration), default_value = "10ms", about = "outbox table check interval, time units: mon,w,d,h,m,s,ms")]
    pub db_check_interval: Duration,

    #[structopt(long, parse(try_from_str = parse_duration), default_value = "0ms", about = "Retention period of processed records. 0 means never. Supported time units: mon,w,d,h,m,s,ms")]
    pub retention: Duration,
}

fn parse_duration(src: &str) -> AppResult<Duration> {
    duration_str::parse(src).map_err(|_| AppError::CLIParseError("duration parse failed".to_string()))
}

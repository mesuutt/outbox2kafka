use crate::{AppError, AppResult};
use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "outbox2kafka", about = "Read events from outbox table and send to kafka")]
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

    #[structopt(long, parse(try_from_str = parse_duration), default_value = "10ms", about = "interval of fetching new records from outbox table, time units: mon,w,d,h,m,s,ms")]
    pub outbox_check_interval: Duration,

    #[structopt(long, parse(try_from_str = parse_cleaner_run_interval), default_value = "10m", about = "interval of deleting old processed records from outbox table. 0 means never delete. Supported time units: mon,w,d,h,m")]
    pub cleaner_run_interval: Duration,

    #[structopt(long, parse(try_from_str = parse_duration), default_value = "0ms", about = "Retention period of processed records in outbox table. 0 means never. Supported time units: mon,w,d,h,m,s,ms")]
    pub processed_data_retention: Duration,
}

fn parse_duration(src: &str) -> AppResult<Duration> {
    duration_str::parse(src).map_err(|_| AppError::DurationParseError(src.to_string()))
}

fn parse_cleaner_run_interval(src: &str)  -> AppResult<Duration> {
    let duration = parse_duration(src)?;
    if duration < Duration::from_secs(60) {
        return Err(AppError::DurationParseError("should be at least 1m".to_string()));
    }

    Ok(duration)
}

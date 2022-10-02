use crate::{AppError, AppResult};
use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "outbox2kafka", about = "Read events from outbox table and send to kafka")]
pub struct Opt {
    #[structopt(short, long, default_value = "localhost:9092")]
    pub brokers: String,

    #[structopt(long)]
    pub topic: String,

    #[structopt(short, long, default_value = "1")]
    pub concurrency: u32,

    #[structopt(short, long, env = "DATABASE_URL", hide_env_values = true)]
    pub db_url: String,

    #[structopt(long)]
    pub table_name: String,

    #[structopt(long, default_value = "2")]
    pub max_db_connection: u32,

    #[structopt(long, parse(try_from_str = parse_duration), default_value = "10ms")]
    pub outbox_check_interval: Duration,

    #[structopt(long, parse(try_from_str = parse_cleaner_run_interval), default_value = "10m")]
    pub cleaner_run_interval: Duration,

    #[structopt(long, parse(try_from_str = parse_duration), default_value = "1h")]
    pub processed_data_retention: Duration,
}

fn parse_duration(src: &str) -> AppResult<Duration> {
    duration_str::parse(src).map_err(|_| AppError::DurationParseError(src.to_string()))
}

fn parse_cleaner_run_interval(src: &str) -> AppResult<Duration> {
    let duration = parse_duration(src)?;
    if duration < Duration::from_secs(60) {
        return Err(AppError::DurationParseError("should be at least 1m".to_string()));
    }

    Ok(duration)
}

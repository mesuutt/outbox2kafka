use crate::{AppError, AppResult};
use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "outbox2kafka", about = "Read events from outbox table and send to kafka")]
pub struct Args {
    #[structopt(long, default_value = "localhost:9092", env = "OUTBOX2KAFKA_BROKERS", hide_env_values = true)]
    pub brokers: String,

    #[structopt(long, env = "OUTBOX2KAFKA_TOPIC", hide_env_values = true)]
    pub topic: String,

    #[structopt(long, default_value = "1", env = "OUTBOX2KAFKA_CONCURRENCY", hide_env_values = true)]
    pub concurrency: u32,

    #[structopt(long, env = "OUTBOX2KAFKA_DB_URL", hide_env_values = true)]
    pub db_url: String,

    #[structopt(long, env = "OUTBOX2KAFKA_TABLE_NAME", hide_env_values = true)]
    pub table_name: String,

    #[structopt(long, default_value = "2", env = "OUTBOX2KAFKA_MAX_DB_CONNECTION", hide_env_values = true)]
    pub max_db_connection: u32,

    #[structopt(long, parse(try_from_str = parse_duration), default_value = "10ms", env = "OUTBOX2KAFKA_OUTBOX_CHECK_INTERVAL", hide_env_values = true)]
    pub outbox_check_interval: Duration,

    #[structopt(long, parse(try_from_str = parse_cleaner_run_interval), default_value = "10m", env = "OUTBOX2KAFKA_CLEANER_RUN_INTERVAL", hide_env_values = true)]
    pub cleaner_run_interval: Duration,

    #[structopt(long, parse(try_from_str = parse_duration), default_value = "1d", env = "OUTBOX2KAFKA_PROCESSED_DATA_RETENTION", hide_env_values = true)]
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

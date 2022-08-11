use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "outbox", about = "description offf")]
pub struct Args {
    #[structopt(env = "DATABASE_URL")]
    pub db_url: String,

    #[structopt(default_value = "localhost:9092")]
    pub brokers: String,

    // TODO: accept values like 1days/10ms etc
    #[structopt(default_value = "1000", about = "outbox table check interval")]
    pub check_interval: usize,

    // TODO: accept values like 1days/10ms etc
    #[structopt(default_value="-1", about = "remove sent events from outbox after time passed. 0 means immediately, -1 never")]
    pub remove_after: i64,
}
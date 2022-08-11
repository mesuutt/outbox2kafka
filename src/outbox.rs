use sqlx::types::Uuid;
use tokio::time::{sleep, Duration};
use crate::AppResult;
use crate::db::DbPool;


#[derive(Debug)]
struct Record {
    pub id: Uuid,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub event_type: String,
    pub payload: serde_json::Value,
}

pub struct Producer {
    pool: DbPool,
    topic: String,
    brokers: String,
    check_interval: Duration,
    clean_after: Duration,
}

impl Producer {
    pub fn new(pool: DbPool, brokers: String, topic: String, check_interval: Duration, clean_after: Duration) -> Self {
        println!("Duration: {:?}", check_interval.as_secs());
        Self { pool, brokers, topic, check_interval, clean_after: clean_after }
    }

    pub async fn start(&self) -> AppResult<()> {
        loop {
            let mut tx = self.pool.begin().await?;
            let record = match self.get_message().await {
                Err(x) => {
                    println!("db error: {:?}", x);
                    continue;
                }
                Ok(x) => x
            };

            println!("{:?}", record);
            sleep(Duration::from_millis(2 * 1000)).await;
        }
    }

    async fn get_message(&self) -> AppResult<Record> {
        let q = sqlx::query_as!(
        Record,
        r#"Select
                id, aggregate_type, aggregate_id,
                event_type, payload
            from messaging_outbox
            where processed_at is null
            order by created
            FOR UPDATE SKIP LOCKED
        "#);
        let event = q.fetch_one(&self.pool).await?;

        Ok(event)
    }
}

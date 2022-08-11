use sqlx::types::Uuid;
use tokio::time::{sleep, Duration};
use crate::AppResult;
use crate::db::DbPool;


#[derive(Debug)]
struct OutboxRecord {
    pub id: Uuid,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub event_type: String,
    pub payload: serde_json::Value,
}

pub struct Producer {
    pool: DbPool,
}

impl Producer {
    pub fn new(pool: DbPool) -> Self{
        Self{pool}
    }

    pub async fn start(&self) -> AppResult<()> {
        loop {
            let mut tx = self.pool.begin().await?;
            let event = match self.fetch_and_send().await {
                Err(x) => {
                    println!("producing error: {:?}", x);
                    continue
                },
                Ok(x) => x
            };

            println!("{:?}", event);
            sleep(Duration::from_millis(2 * 1000)).await;
        }
    }

    async fn fetch_and_send(&self) -> AppResult<OutboxRecord> {
        let q = sqlx::query_as!(
        OutboxRecord,
        r#"Select
                id, aggregate_type, aggregate_id,
                event_type, payload
            from messaging_outbox
            where processed_at is null
            FOR UPDATE SKIP LOCKED
        "#);
        let event = q.fetch_one(&self.pool).await?;

        Ok(event)
    }

}

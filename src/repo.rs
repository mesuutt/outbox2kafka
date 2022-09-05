use std::future::Future;

use crate::db::DbPool;
use crate::model::Record;
use crate::{AppResult};
use std::time::Duration;

use uuid::Uuid;


pub struct Repo {
    pool: DbPool,
    retention: Duration,
}

impl Repo {
    pub fn new(pool: DbPool, retention: Duration) -> Self {
        Self { pool, retention }
    }

    pub async fn get_for_process<F, T>(&self, func: F) -> AppResult<()>
        where
            F: Fn(Record) -> T,
            T: Future<Output = AppResult<()>> + Send,
    {
        // We are creating a db transaction.
        // If not error occurred the tx will commit.
        self.pool.begin().await?;

        if let Some(record) = self.get_one_record().await? {
            let record_id = record.id;
            func(record).await?;
            if self.retention.is_zero() {
                self.delete_record(record_id).await?
            } else {
                self.mark_as_processed(record_id).await?;
            }
        }

        Ok(())
    }

    async fn get_one_record(&self) -> AppResult<Option<Record>> {
        let q = sqlx::query_as!(
            Record,
            r#"Select
                id, aggregate_type, aggregate_id,
                event_type, payload, metadata
            from messaging_outbox
            where processed_at is null
            order by created
            FOR UPDATE SKIP LOCKED
        "#
        );
        let record = q.fetch_optional(&self.pool).await?;

        Ok(record)
    }

    async fn mark_as_processed(&self, id: Uuid) -> AppResult<()> {
        let q = sqlx::query!("Update messaging_outbox set processed_at=now() where id=$1", id);
        q.execute(&self.pool).await?;
        Ok(())
    }

    async fn delete_record(&self, id: Uuid) -> AppResult<()> {
        let q = sqlx::query!("Delete from messaging_outbox where id=$1", id);
        q.execute(&self.pool).await?;
        Ok(())
    }

    /*async fn delete_older_than(&self, time: Uuid) -> AppResult<()> {
        let q = sqlx::query!("Delete from messaging_outbox where processed_at <$1", time);
        q.execute(&self.pool).await?;
        Ok(())
    }*/
}

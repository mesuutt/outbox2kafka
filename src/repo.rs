use std::future::Future;

use crate::db::DbPool;
use crate::model::Record;
use crate::AppResult;
use chrono::{DateTime, Utc};
use once_cell::sync::OnceCell;
use std::time::Duration;

use sqlx::Row;
use uuid::Uuid;

static SQL_GET_ONE_RECORD: OnceCell<String> = OnceCell::new();
static SQL_MARK_RECORD_AS_PROCESSED: OnceCell<String> = OnceCell::new();
static SQL_DELETE_ONE_RECORD: OnceCell<String> = OnceCell::new();
static SQL_DELETE_OLD_RECORDS: OnceCell<String> = OnceCell::new();

pub struct Repo {
    pool: DbPool,
    retention: Duration,
    table_name: String,
}

impl Repo {
    pub fn new(pool: DbPool, table_name: String, retention: Duration) -> Self {
        Self {
            pool,
            retention,
            table_name: table_name.to_string(),
        }
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
        let sql = SQL_GET_ONE_RECORD.get_or_init(|| {
            format!(
                r#"Select
                id,
                aggregate_id, event_type,
                payload, metadata
            from {}
            where processed_date is null
            order by occurred_on
            FOR UPDATE SKIP LOCKED
            "#,
                &self.table_name
            )
        });

        let row = sqlx::query(&sql).fetch_optional(&self.pool).await?;
        match row {
            None => Ok(None),
            Some(r) => Ok(Some(Record {
                id: r.get(0),
                aggregate_id: r.get(1),
                event_type: r.get(2),
                payload: r.get(3),
                metadata: r.get(4),
            })),
        }
    }

    async fn mark_as_processed(&self, id: Uuid) -> AppResult<()> {
        let sql = SQL_MARK_RECORD_AS_PROCESSED
            .get_or_init(|| format!("Update {} set processed_date=$1 where id=$2", &self.table_name));
        sqlx::query(sql).bind(Utc::now()).bind(id).execute(&self.pool).await?;

        Ok(())
    }

    async fn delete_record(&self, id: Uuid) -> AppResult<()> {
        let sql = SQL_DELETE_ONE_RECORD.get_or_init(|| format!("Delete from {} where id=$1", &self.table_name));

        sqlx::query(sql).bind(id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn delete_older_than(&self, time: DateTime<Utc>) -> AppResult<()> {
        let sql =
            SQL_DELETE_OLD_RECORDS.get_or_init(|| format!("Delete from {} where processed_date <$1", &self.table_name));

        sqlx::query(sql).bind(time).execute(&self.pool).await?;

        Ok(())
    }
}

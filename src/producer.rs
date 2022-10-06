use log::{debug, error, info};
use std::future::Future;
use std::sync::Arc;
use std::time;
use std::time::Duration;

use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord, Producer as ProducerTrait};
use serde_json::Value;

use crate::model::Record;
use crate::repo::Repo;
use crate::{AppError, AppResult};

pub struct Producer {
    topic: String,
    repo: Arc<Repo>,
    check_interval: Duration,
    producer: FutureProducer,
}

impl Producer {
    pub fn new(brokers: String, topic: String, repo: Arc<Repo>, check_interval: Duration) -> AppResult<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()?;

        Ok(Self {
            topic,
            repo,
            check_interval,
            producer,
        })
    }

    pub async fn run(&self, shutdown: impl Future) {
        tokio::select! {
            _ = self.run_forever() => {}
            _ = shutdown => {
                self.producer.flush(time::Duration::from_secs(5));
                info!("producer shutting down");
            }
        }
    }

    async fn run_forever(&self) {
        let mut interval = tokio::time::interval(self.check_interval);
        loop {
            interval.tick().await;

            let result = self
                .repo
                .get_for_process(|record| async move {
                    debug!("record sending to kafka: {}", record);
                    self.send(record).await
                })
                .await;

            if let Err(e) = result {
                error!("sending record to kafka failed: {}", e)
            }
        }
    }

    async fn send(&self, record: Record) -> AppResult<()> {
        let future_record = FutureRecord::to(&self.topic)
            .key(&record.aggregate_id)
            .headers(Producer::build_headers(&record)?)
            .payload(&record.payload);

        self.producer
            .send(future_record, Duration::from_secs(0))
            .await
            .map_err(|(x, _)| AppError::KafkaError(x))?;

        debug!("record sent to kafka: {}", record);

        Ok(())
    }

    fn build_headers(record: &Record) -> AppResult<OwnedHeaders> {
        let mut headers = OwnedHeaders::new()
            .add("event_type", &record.event_type)
            .add("aggregate_id", &record.aggregate_id);

        let metadata = if let Some(x) = &record.metadata { x } else {
            return Ok(headers);
        };

        let json_val = if let Ok(x) = serde_json::from_str::<Value>(metadata) { x } else {
            return Err(AppError::InvalidMetadataError(record.id));
        };

        let kv_map = if let Some(x) = json_val.as_object() { x } else {
            return Err(AppError::InvalidMetadataError(record.id));
        };

        for (k, v) in kv_map {
            if let Some(x) = v.as_str() {
                headers = headers.add(k, x);
            } else if let Ok(x) = serde_json::to_string(v) {
                headers = headers.add(k, &x);
            }
        }

        Ok(headers)
    }
}

use std::future::Future;
use std::sync::Arc;
use std::time;
use std::time::Duration;
use log::{debug, error, info};

use rdkafka::producer::{FutureProducer, FutureRecord, Producer as ProducerTrait};
use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;

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

            let result = self.repo.get_for_process(|record: Record| async move {
                self.send(record).await
            }).await;

            if let Err(e) = result {
                error!("producer error: {:?}", e)
            }
        }
    }

    async fn send(&self, record: Record) -> AppResult<()> {
        let mut headers = OwnedHeaders::new();
        if let Some(ref value) = record.metadata {
            if let Some(map) = value.as_object() {
                for (k, v) in map {
                    headers = headers.add(k, &serde_json::to_vec(v).unwrap());
                }
            } else {
                error!("metadata should be key value map")
            }
        }

        self.producer.send(
            FutureRecord::to(&self.topic)
                .payload(&record.payload)
                .headers(headers)
                .key(&record.key()
                ), Duration::from_secs(0))
            .await
            .map_err(|(x, _)| AppError::KafkaError(x))?;

        debug!("record sent to kafka: {:?}", record.key());

        Ok(())
    }
}

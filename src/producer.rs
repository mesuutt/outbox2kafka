use std::sync::Arc;
use std::time::Duration;
use tokio::time::{sleep};
use log::{error, info};

use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::config::ClientConfig;

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

    pub async fn start(&self) {
        loop {
            let result = self.repo.get_for_process(|record: Record| async move {
                self.producer.send(
                    FutureRecord::to(&self.topic)
                        .payload(&record.payload)
                        .key(&record.key()
                        ), Duration::from_secs(0))
                    .await
                    .map_err(|(x, _)| AppError::KafkaError(x))?;

                info!("record sent to kafka: {:?}", record.key());

                Ok(())
            }).await;

            if let Err(e) = result {
                error!("sending event to kafka failed with error: {:?}", e)
            }

            sleep(self.check_interval).await;
        }
    }
}

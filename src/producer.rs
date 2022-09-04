use std::cell::RefCell;
use tokio::time::{sleep, Duration};
use log::info;
use kafka::producer::{Producer as KafkaProducer, Record as KafkaRecord, RequiredAcks};

use crate::model::Record;
use crate::repo::Repo;
use crate::AppResult;

pub struct Producer {
    topic: String,
    repo: Repo,
    check_interval: Duration,
    // KafkaProducer::send needs mutable ref
    // RefCell was used for get rid of necessity of mutable ref
    producer: RefCell<KafkaProducer>,
}

impl Producer {
    pub fn new(brokers: String, topic: String, repo: Repo, check_interval: Duration) -> AppResult<Self> {
        let broker_list = brokers
            .replace(" ", "")
            .split(",")
            .into_iter()
            .map(|x| x.to_string())
            .collect();

        let producer = KafkaProducer::from_hosts(broker_list)
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()?;

        Ok(Self {
            topic,
            repo,
            check_interval,
            producer: RefCell::new(producer),
        })
    }

    pub async fn start(&self) -> AppResult<()> {
        loop {
            self.repo
                .get_for_process(|record: &Record| {
                    let message = &KafkaRecord {
                        key: record.key(),
                        value: record.payload.as_bytes(),
                        topic: self.topic.as_str(),
                        partition: -1,
                    };

                    self.producer.borrow_mut().send(message)?;

                    info!("record sent to kafka: {:?}", record.key());
                    Ok(())
                })
                .await?;

            sleep(self.check_interval).await;
        }
    }
}

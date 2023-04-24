use anyhow::{anyhow, Result};
use futures::Future;
use rdkafka::{
    error::KafkaError,
    message::OwnedMessage,
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};
use std::time::Duration;

use super::config::KafkaConfig;

#[derive(Debug)]
pub struct KafkaMsgToSend {
    pub key: String,
    pub value: String,
}

pub struct KafkaProducer {
    producer: FutureProducer,
    topic_name: String,
}

impl KafkaProducer {
    pub fn new(config: KafkaConfig) -> Result<Self> {
        let topic_name = config.topic.clone();
        let client_config: ClientConfig = config.into();
        let producer = client_config.create().map_err(|e| anyhow!(e))?;

        Ok(KafkaProducer {
            producer,
            topic_name,
        })
    }

    pub fn send_message<'a>(
        &'a self,
        msg: &'a KafkaMsgToSend,
    ) -> impl Future<Output = Result<(i32, i64), (KafkaError, OwnedMessage)>> + 'a {
        self.producer.send(
            FutureRecord::to(&self.topic_name)
                .key(&msg.key)
                .payload(&msg.value),
            Duration::from_millis(0),
        )
    }
}

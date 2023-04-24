use super::config::KafkaConfig;
use anyhow::{anyhow, Result};
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    message::BorrowedMessage,
    util::Timeout,
    ClientConfig, TopicPartitionList,
};

pub struct KafkaConsumer {
    consumer: StreamConsumer,
    topic_name: String,
    partitions: i32,
}

impl KafkaConsumer {
    pub fn new(config: KafkaConfig) -> Result<Self> {
        let topic_name = config.topic.clone();
        let partitions = config.partitions;

        let config: ClientConfig = config.into();
        let consumer = config
            .create::<StreamConsumer>()
            .expect("Consumer creation failed");

        consumer.subscribe(&[&topic_name]).map_err(|e| anyhow!(e))?;

        Ok(Self {
            consumer,
            topic_name,
            partitions,
        })
    }

    pub fn store_offset(&self, partition: i32, offset: i64) -> Result<()> {
        let mut tpl = TopicPartitionList::new();
        tpl.add_partition_offset(&self.topic_name, partition, rdkafka::Offset::Offset(offset))?;

        Ok(self.consumer.assign(&tpl)?)
    }

    pub fn is_empty(&self) -> Result<bool> {
        log::info!("Checking topic emptyness...");

        let mut tpl = TopicPartitionList::new();
        for i in 0..self.partitions {
            tpl.add_partition(&self.topic_name, i);
        }

        let offsets = self.consumer.offsets_for_times(tpl, Timeout::Never)?;

        log::info!("Offsets for topic {}: {:#?}", self.topic_name, offsets);

        for p in offsets.elements_for_topic(&self.topic_name).into_iter() {
            match p.offset() {
                rdkafka::Offset::End => continue,
                _ => return Ok(false),
            }
        }

        Ok(true)
    }

    pub async fn read_msg(&self) -> Result<BorrowedMessage> {
        let msg = self.consumer.recv().await.map_err(|e| anyhow!(e))?;

        self.consumer.store_offset_from_message(&msg)?;

        Ok(msg)
    }
}

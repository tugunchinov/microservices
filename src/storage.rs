use anyhow::{anyhow, bail, Result};
use rdkafka::Message;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    collections::HashMap,
    hash::Hash,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::kafka::{
    config::KafkaConfig,
    consumer::KafkaConsumer,
    producer::{KafkaMsgToSend, KafkaProducer},
};

enum StorageState {
    Opened,
    Closed,
}

pub struct KeyValueStorage<K: Eq + Hash + Serialize, V: Serialize> {
    config: KafkaConfig,
    storage: HashMap<K, V>,
    state: StorageState,
    consumer: KafkaConsumer,
}

impl<K: Eq + Hash + Serialize, V: Serialize> KeyValueStorage<K, V> {
    pub async fn new(config: KafkaConfig) -> Result<Self>
    where
        K: DeserializeOwned,
        V: DeserializeOwned,
    {
        let consumer = KafkaConsumer::new(config.clone()).map_err(|e| anyhow!(e))?;

        let storage = if consumer.is_empty()? {
            log::info!("Saved storage is empty");

            HashMap::new()
        } else {
            log::info!("Reading state from Kafka...");
            let msg = consumer.read_msg().await?.detach();

            log::info!("Decoding message...");

            let Some(payload) = msg.payload() else { bail!("No payload in message") };
            let kv_vec: Vec<(K, V)> = serde_json::from_slice(payload)?;
            kv_vec.into_iter().collect()
        };

        Ok(Self {
            config,
            storage,
            state: StorageState::Opened,
            consumer,
        })
    }

    pub fn read(&self, key: &K) -> Result<Option<&V>> {
        match self.state {
            StorageState::Opened => Ok(self.storage.get(key)),
            StorageState::Closed => {
                bail!("An attempt to accesss closed storage!")
            }
        }
    }

    pub fn exists(&self, key: &K) -> Result<bool> {
        match self.state {
            StorageState::Opened => Ok(self.storage.contains_key(key)),
            StorageState::Closed => {
                bail!("An attempt to accesss closed storage!")
            }
        }
    }

    pub fn write(&mut self, key: K, value: V) -> Result<Option<V>> {
        match self.state {
            StorageState::Opened => Ok(self.storage.insert(key, value)),
            StorageState::Closed => {
                bail!("An attempt to accesss closed storage!")
            }
        }
    }

    pub fn delete(&mut self, key: &K) -> Result<Option<V>> {
        match self.state {
            StorageState::Opened => Ok(self.storage.remove(key)),
            StorageState::Closed => {
                bail!("An attempt to accesss closed storage!")
            }
        }
    }

    pub fn read_keys(&self) -> Result<impl Iterator<Item = &K>> {
        match self.state {
            StorageState::Opened => Ok(self.storage.keys()),
            StorageState::Closed => {
                bail!("An attempt to accesss closed storage!")
            }
        }
    }

    pub fn size(&self) -> usize {
        self.storage.len()
    }

    pub async fn flush(&mut self) -> Result<()> {
        if self.storage.is_empty() {
            return Ok(());
        }

        log::info!("Flushing storage state to Kafka...");

        let producer = KafkaProducer::new(self.config.clone())?;

        log::info!("Serializing...");
        let key =
            serde_json::to_string(&SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis())?;
        let kv_vec: Vec<(&K, &V)> = self.storage.iter().collect();
        let value = serde_json::to_string(&kv_vec)?;

        log::info!("Sending message...");
        let (partition, offset) = producer
            .send_message(&KafkaMsgToSend { key, value })
            .await
            .map_err(|(e, _)| anyhow!(e))?;

        self.consumer.store_offset(partition, offset)?;

        Ok(())
    }

    pub async fn close(&mut self) -> Result<()> {
        log::info!("Closing storage...");

        self.state = StorageState::Closed;

        self.flush().await
    }
}

impl<K: Eq + Hash + Serialize, V: Serialize> Drop for KeyValueStorage<K, V> {
    fn drop(&mut self) {
        futures::executor::block_on(self.close()).unwrap();
    }
}

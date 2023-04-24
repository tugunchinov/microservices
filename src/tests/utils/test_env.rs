use crate::{kafka::config::KafkaConfig, storage::KeyValueStorage};
use anyhow::{bail, Result};
use futures::Future;
use rand::{distributions::Alphanumeric, Rng};
use rdkafka::{
    admin::{AdminClient, AdminOptions, NewTopic, TopicReplication},
    ClientConfig,
};
use serde::Serialize;
use std::hash::Hash;

use super::{
    builders::{build_numbers_storage, build_pojo_storage, build_strings_storage},
    types::{Student, StudentKey},
};

pub async fn create_topic(name: &str, partitions: i32) {
    let config: ClientConfig = KafkaConfig::new().into();
    let client: AdminClient<_> = config.create().unwrap();
    client
        .create_topics(
            &[NewTopic::new(name, partitions, TopicReplication::Fixed(1))],
            &AdminOptions::new(),
        )
        .await
        .unwrap();
}

pub async fn do_in_temp_topic<F, Fut, E>(callback: F) -> Result<E>
where
    Fut: Future<Output = Result<E>>,
    F: FnOnce(KafkaConfig) -> Fut,
{
    const PREFIX: &str = "test_task_2";

    let suffix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(15)
        .map(char::from)
        .collect();

    let temp_topic = format!("{PREFIX}_{suffix}");

    let mut config = KafkaConfig::new();
    config.topic = temp_topic;

    create_topic(&config.topic, config.partitions).await;

    log::info!("temp topic: {}", config.topic);

    callback(config).await
}

pub fn assert_fully_match<'a, T, I>(it: I, items: &'a [&T]) -> Result<()>
where
    T: Ord,
    I: Iterator<Item = &'a T>,
{
    let expected = std::collections::BTreeSet::from_iter(items.iter().copied());
    let actual = std::collections::BTreeSet::from_iter(it);

    if actual == expected {
        Ok(())
    } else {
        bail!("Collections don't match")
    }
}

pub async fn storage_callback<K: Eq + Hash + Serialize, V: Serialize, F, FutF, B, FutB>(
    config: KafkaConfig,
    callback: F,
    builder: B,
) -> Result<()>
where
    FutF: Future<Output = Result<()>>,
    FutB: Future<Output = KeyValueStorage<K, V>>,
    F: FnOnce(KeyValueStorage<K, V>) -> FutF,
    B: FnOnce(KafkaConfig) -> FutB,
{
    let storage = builder(config).await;

    callback(storage).await?;

    Ok(())
}

pub fn do_with_strings<F, Fut>(config: KafkaConfig, callback: F) -> impl Future<Output = Result<()>>
where
    Fut: Future<Output = Result<()>>,
    F: FnOnce(KeyValueStorage<String, String>) -> Fut,
{
    storage_callback(config, callback, build_strings_storage)
}

pub fn do_with_numbers<F, Fut>(config: KafkaConfig, callback: F) -> impl Future<Output = Result<()>>
where
    Fut: Future<Output = Result<()>>,
    F: FnOnce(KeyValueStorage<i32, f64>) -> Fut,
{
    storage_callback(config, callback, build_numbers_storage)
}

pub fn do_with_pojo<F, Fut>(config: KafkaConfig, callback: F) -> impl Future<Output = Result<()>>
where
    Fut: Future<Output = Result<()>>,
    F: FnOnce(KeyValueStorage<StudentKey, Student>) -> Fut,
{
    storage_callback(config, callback, build_pojo_storage)
}

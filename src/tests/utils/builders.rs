use super::types::{Student, StudentKey};
use crate::{kafka::config::KafkaConfig, storage::KeyValueStorage};

pub async fn build_strings_storage(config: KafkaConfig) -> KeyValueStorage<String, String> {
    KeyValueStorage::new(config).await.unwrap()
}

pub async fn build_numbers_storage(config: KafkaConfig) -> KeyValueStorage<i32, f64> {
    KeyValueStorage::new(config).await.unwrap()
}

pub async fn build_pojo_storage(config: KafkaConfig) -> KeyValueStorage<StudentKey, Student> {
    KeyValueStorage::new(config).await.unwrap()
}

use std::path::PathBuf;

use crate::storage::KeyValueStorage;

use super::types::{Student, StudentKey};

pub fn build_strings_storage(path: PathBuf) -> KeyValueStorage<String, String> {
    KeyValueStorage::new(path).unwrap()
}

pub fn build_numbers_storage(path: PathBuf) -> KeyValueStorage<i32, f64> {
    KeyValueStorage::new(path).unwrap()
}

pub fn build_pojo_storage(path: PathBuf) -> KeyValueStorage<StudentKey, Student> {
    KeyValueStorage::new(path).unwrap()
}

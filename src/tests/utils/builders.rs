use std::path::Path;

use crate::storage::KeyValueStorage;

use super::types::{Student, StudentKey};

pub fn build_strings_storage(path: &Path) -> KeyValueStorage<String, String> {
    KeyValueStorage::new(path).unwrap()
}

pub fn build_numbers_storage(path: &Path) -> KeyValueStorage<i32, f64> {
    KeyValueStorage::new(path).unwrap()
}

pub fn build_pojo_storage(path: &Path) -> KeyValueStorage<StudentKey, Student> {
    KeyValueStorage::new(path).unwrap()
}

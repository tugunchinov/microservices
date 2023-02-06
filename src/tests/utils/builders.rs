use crate::storage::KeyValueStorage;

use super::types::{Student, StudentKey};

pub fn build_strings_storage(path: String) -> KeyValueStorage<String, String> {
    todo!()
}

pub fn build_numbers_storage(path: String) -> KeyValueStorage<i32, f64> {
    todo!()
}

pub fn build_pojo_storage(path: String) -> KeyValueStorage<StudentKey, Student> {
    todo!()
}

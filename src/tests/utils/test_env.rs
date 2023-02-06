use crate::storage::KeyValueStorage;
use anyhow::{bail, Result};
use std::fs::{remove_file, File};

use super::{
    builders::{build_numbers_storage, build_pojo_storage, build_strings_storage},
    types::{Student, StudentKey},
};

pub fn do_in_temp_directory<F, E>(callback: F) -> Result<E>
where
    F: FnOnce(String) -> Result<E>,
{
    const PATH: &str = "test_task_2";

    File::create(PATH)?;

    let callback_res = callback(PATH.to_string());

    remove_file(PATH)?;

    callback_res
}

pub fn assert_fully_match<'a, T, I>(it: I, items: &'a [T]) -> Result<()>
where
    T: Ord,
    I: Iterator<Item = &'a T>,
{
    let expected = std::collections::BTreeSet::from_iter(items.into_iter());
    let actual = std::collections::BTreeSet::from_iter(it);

    if actual == expected {
        Ok(())
    } else {
        bail!("Collections don't match")
    }
}

pub fn storage_callback<K, V, F, B>(
    path: String,
    callback: F,
    builder: B,
) -> Result<KeyValueStorage<K, V>>
where
    F: FnOnce(&mut KeyValueStorage<K, V>) -> Result<()>,
    B: FnOnce(String) -> KeyValueStorage<K, V>,
{
    let mut storage = builder(path);

    callback(&mut storage)?;

    storage.close()?;

    Ok(storage)
}

pub fn do_with_strings<F>(path: String, callback: F) -> Result<KeyValueStorage<String, String>>
where
    F: FnOnce(&mut KeyValueStorage<String, String>) -> Result<()>,
{
    storage_callback(path, callback, build_strings_storage)
}

pub fn do_with_numbers<F>(path: String, callback: F) -> Result<KeyValueStorage<i32, f64>>
where
    F: FnOnce(&mut KeyValueStorage<i32, f64>) -> Result<()>,
{
    storage_callback(path, callback, build_numbers_storage)
}

pub fn do_with_pojo<F>(path: String, callback: F) -> Result<KeyValueStorage<StudentKey, Student>>
where
    F: FnOnce(&mut KeyValueStorage<StudentKey, Student>) -> Result<()>,
{
    storage_callback(path, callback, build_pojo_storage)
}

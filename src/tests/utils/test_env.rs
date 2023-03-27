use crate::storage::KeyValueStorage;
use anyhow::{bail, Result};
use serde::Serialize;
use std::{hash::Hash, path::Path};
use tempdir::TempDir;

use super::{
    builders::{build_numbers_storage, build_pojo_storage, build_strings_storage},
    types::{Student, StudentKey},
};

pub fn do_in_temp_directory<F, E>(callback: F) -> Result<E>
where
    F: FnOnce(&Path) -> Result<E>,
{
    const PATH: &str = "test_task_2";

    let tmp_dir = TempDir::new(PATH)?;

    println!("temp path: {}", tmp_dir.path().display());

    let callback_res = callback(tmp_dir.path());

    callback_res
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

pub fn storage_callback<K: Eq + Hash + Serialize, V: Serialize, F, B>(
    path: &Path,
    callback: F,
    builder: B,
) -> Result<()>
where
    F: FnOnce(KeyValueStorage<K, V>) -> Result<()>,
    B: FnOnce(&Path) -> KeyValueStorage<K, V>,
{
    let storage = builder(path);

    callback(storage)?;

    Ok(())
}

pub fn do_with_strings<F>(path: &Path, callback: F) -> Result<()>
where
    F: FnOnce(KeyValueStorage<String, String>) -> Result<()>,
{
    storage_callback(path, callback, build_strings_storage)
}

pub fn do_with_numbers<F>(path: &Path, callback: F) -> Result<()>
where
    F: FnOnce(KeyValueStorage<i32, f64>) -> Result<()>,
{
    storage_callback(path, callback, build_numbers_storage)
}

pub fn do_with_pojo<F>(path: &Path, callback: F) -> Result<()>
where
    F: FnOnce(KeyValueStorage<StudentKey, Student>) -> Result<()>,
{
    storage_callback(path, callback, build_pojo_storage)
}

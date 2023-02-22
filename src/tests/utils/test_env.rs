use crate::storage::KeyValueStorage;
use anyhow::{bail, Result};
use std::path::PathBuf;
use tempdir::TempDir;

use super::{
    builders::{build_numbers_storage, build_pojo_storage, build_strings_storage},
    types::{Student, StudentKey},
};

pub fn do_in_temp_directory<F, E>(callback: F) -> Result<E>
where
    F: FnOnce(PathBuf) -> Result<E>,
{
    const PATH: &str = "test_task_2";

    let tmp_dir = TempDir::new(PATH)?;

    let callback_res = callback(tmp_dir.path().to_owned());

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

pub fn storage_callback<K, V, F, B>(path: PathBuf, callback: F, builder: B) -> Result<()>
where
    F: FnOnce(KeyValueStorage<K, V>) -> Result<()>,
    B: FnOnce(PathBuf) -> KeyValueStorage<K, V>,
{
    let storage = builder(path);

    callback(storage)?;

    Ok(())
}

pub fn do_with_strings<F>(path: PathBuf, callback: F) -> Result<()>
where
    F: FnOnce(KeyValueStorage<String, String>) -> Result<()>,
{
    storage_callback(path, callback, build_strings_storage)
}

pub fn do_with_numbers<F>(path: PathBuf, callback: F) -> Result<()>
where
    F: FnOnce(KeyValueStorage<i32, f64>) -> Result<()>,
{
    storage_callback(path, callback, build_numbers_storage)
}

pub fn do_with_pojo<F>(path: PathBuf, callback: F) -> Result<()>
where
    F: FnOnce(KeyValueStorage<StudentKey, Student>) -> Result<()>,
{
    storage_callback(path, callback, build_pojo_storage)
}

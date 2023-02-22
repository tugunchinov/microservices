use fs_extra::dir::CopyOptions;
use std::path::Path;

use crate::tests::utils::test_env::{
    self, assert_fully_match, do_in_temp_directory, do_with_numbers, do_with_pojo, do_with_strings,
};

use super::{KEY_1, KEY_2, VALUE_1, VALUE_2};

#[test]
fn test_read_write() {
    test_env::do_in_temp_directory(|path| {
        do_with_strings(path, |mut storage| {
            storage.write(&"foo".to_string(), "bar".to_string())?;
            assert_eq!(
                "bar".to_string(),
                storage.read(&"foo".to_string())?.unwrap()
            );
            assert_eq!(1, storage.size());

            assert_fully_match(storage.read_keys(), &[&"foo".to_string()])
        })
    })
    .ok();
}

#[test]
fn test_persistence() {
    test_env::do_in_temp_directory(|path| {
        do_with_pojo(path.clone(), |mut storage| {
            storage.write(&KEY_1, VALUE_1.clone())
        })
        .ok();
        do_with_pojo(path, |storage| {
            assert_eq!(VALUE_1, storage.read(&KEY_1)?.unwrap());
            assert_eq!(1, storage.size());
            assert_fully_match(storage.read_keys(), &[&KEY_1])
        })
    })
    .ok();

    test_env::do_in_temp_directory(|path| {
        do_with_pojo(path, |storage| {
            assert_eq!(storage.read(&KEY_1)?, None);
            Ok(())
        })
    })
    .ok();
}

#[test]
fn test_missing_key() {
    test_env::do_in_temp_directory(|path| {
        do_with_numbers(path, |mut storage| {
            storage.write(&4, 3.0)?;
            assert_eq!(storage.read(&4)?.unwrap(), 3.0);
            assert_eq!(storage.read(&5)?, None);
            assert_eq!(1, storage.size());
            assert_fully_match(storage.read_keys(), &[&4])
        })
    })
    .ok();
}

#[test]
fn test_multiple_modifications() {
    do_in_temp_directory(|path| {
        do_with_strings(path.clone(), |mut storage| {
            storage.write(&"foo".to_string(), "bar".to_string())?;
            storage.write(&"bar".to_string(), "foo".to_string())?;
            storage.write(&"yammy".to_string(), "nooo".to_string())?;
            assert_eq!(
                "bar".to_string(),
                storage.read(&"foo".to_string())?.unwrap()
            );
            assert_eq!(
                "foo".to_string(),
                storage.read(&"bar".to_string())?.unwrap()
            );
            assert_eq!(
                "nooo".to_string(),
                storage.read(&"yammy".to_string())?.unwrap()
            );
            assert!(storage.exists(&"foo".to_string())?);
            assert_eq!(3, storage.size());
            assert_fully_match(
                storage.read_keys(),
                &[&"bar".to_string(), &"foo".to_string(), &"yammy".to_string()],
            )
        })
        .ok();

        do_with_strings(path.clone(), |storage| {
            assert_eq!(
                "bar".to_string(),
                storage.read(&"foo".to_string())?.unwrap()
            );
            assert_eq!(
                "foo".to_string(),
                storage.read(&"bar".to_string())?.unwrap()
            );
            assert_eq!(
                "nooo".to_string(),
                storage.read(&"yammy".to_string())?.unwrap()
            );
            assert!(storage.exists(&"bar".to_string())?);
            assert!(!storage.exists(&"yep".to_string())?);
            assert_eq!(3, storage.size());
            assert_fully_match(
                storage.read_keys(),
                &[&"bar".to_string(), &"foo".to_string(), &"yammy".to_string()],
            )
        })
        .ok();

        do_with_strings(path.clone(), |mut storage| {
            storage.delete(&"bar".to_string())?;
            storage.write(&"yammy".to_string(), "yeahs".to_string())?;
            assert!(!storage.exists(&"bar".to_string())?);
            assert!(!storage.exists(&"yep".to_string())?);
            assert_eq!(2, storage.size());
            assert_fully_match(
                storage.read_keys(),
                &[&"foo".to_string(), &"yammy".to_string()],
            )
        })
        .ok();

        do_with_strings(path.clone(), |storage| {
            assert_eq!(
                "bar".to_string(),
                storage.read(&"foo".to_string())?.unwrap()
            );
            assert_eq!(storage.read(&"bar".to_string())?, None);
            assert_eq!(
                "yeahs".to_string(),
                storage.read(&"yammy".to_string())?.unwrap()
            );
            assert_eq!(2, storage.size());
            assert_fully_match(
                storage.read_keys(),
                &[&"foo".to_string(), &"yammy".to_string()],
            )
        })
    })
    .ok();
}

#[test]
fn test_persist_and_copy() {
    do_in_temp_directory(|path1| {
        do_with_pojo(path1.clone(), |mut storage| {
            storage.write(&KEY_1, VALUE_1.clone())?;
            storage.write(&KEY_2, VALUE_2.clone())
        })
        .ok();

        do_in_temp_directory(|mut path2| {
            path2.push(Path::new("trololo"));
            fs_extra::dir::copy(&path1, &path2, &CopyOptions::new())?;

            do_with_pojo(path2, |storage| {
                assert_eq!(VALUE_1, storage.read(&KEY_1)?.unwrap());
                assert_eq!(VALUE_2, storage.read(&KEY_2)?.unwrap());
                assert_eq!(2, storage.size());
                assert_fully_match(storage.read_keys(), &[&KEY_1, &KEY_2])
            })
        })
    })
    .ok();
}

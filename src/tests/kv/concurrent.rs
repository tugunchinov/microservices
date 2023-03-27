use crate::tests::utils::test_env::{do_in_temp_directory, do_with_numbers, do_with_strings};

#[test]
#[cfg(no_compile)]
fn test_iterator_with_concurrent_keys_modification() {
    do_in_temp_directory(|path| {
        do_with_pojo(path, |mut storage| {
            storage.write(KEY_1.clone(), VALUE_1.clone())?;
            storage.write(KEY_2.clone(), VALUE_2.clone())?;
            storage.write(KEY_3.clone(), VALUE_3.clone())?;

            let mut iter = storage.read_keys()?;
            assert!(&[KEY_1.clone(), KEY_2.clone(), KEY_3.clone()].contains(iter.next().unwrap()));
            assert!(&[KEY_1.clone(), KEY_2.clone(), KEY_3.clone()].contains(iter.next().unwrap()));

            storage.delete(&KEY_2)?;

            iter.next();

            Ok(())
        })
    })
    .ok();
}

#[test]
#[cfg(no_compile)]
fn test_iterator_with_concurrent_values_modification() {
    do_in_temp_directory(|path| {
        do_with_pojo(path, |mut storage| {
            storage.write(KEY_1.clone(), VALUE_1.clone())?;
            storage.write(KEY_2.clone(), VALUE_2.clone())?;
            storage.write(KEY_3.clone(), VALUE_3.clone())?;

            let mut iter = storage.read_keys()?;

            assert!(&[KEY_1.clone(), KEY_2.clone(), KEY_3.clone()].contains(iter.next().unwrap()));
            assert!(&[KEY_1.clone(), KEY_2.clone(), KEY_3.clone()].contains(iter.next().unwrap()));

            storage.write(KEY_3.clone(), VALUE_2.clone())?;

            assert!(&[KEY_1.clone(), KEY_2.clone(), KEY_3.clone()].contains(iter.next().unwrap()));

            Ok(())
        })
    })
    .ok();
}

#[test]
fn test_do_not_write_in_closed_state() {
    do_in_temp_directory(|path| {
        do_with_numbers(path, |mut storage| {
            assert_eq!(storage.read(&4)?, None);
            storage.write(4, 5.0)?;
            assert_eq!(5.0, *storage.read(&4)?.unwrap());
            storage.close()?;
            storage.write(3, 5.0).map(|_| {})
        })
    })
    .expect_err("Succeeded in writing to the closed storage!");
}

#[test]
fn test_do_not_read_in_closed_state() {
    do_in_temp_directory(|path| {
        do_with_strings(path, |mut storage| {
            assert_eq!(storage.read(&"trololo".to_string())?, None);
            storage.write("trololo".to_string(), "yarr".to_string())?;
            assert_eq!(
                "yarr".to_string(),
                *storage.read(&"trololo".to_string())?.unwrap()
            );
            storage.close()?;
            storage.read_keys()?.count();
            Ok(())
        })
    })
    .expect_err("Succeeded in reading from the closed storage!");
}

use std::cell::RefCell;

use crate::tests::utils::test_env::{
    do_in_temp_directory, do_with_numbers, do_with_pojo, do_with_strings,
};

use super::{KEY_1, KEY_2, KEY_3, VALUE_1, VALUE_2, VALUE_3};

#[test]
#[should_panic(expected = "Concurrent modification")]
fn test_iterator_with_concurrent_keys_modification() {
    do_in_temp_directory(|path| {
        do_with_pojo(path, |storage| {
            let storage = RefCell::new(storage);

            storage.borrow_mut().write(&KEY_1, VALUE_1.clone())?;
            storage.borrow_mut().write(&KEY_2, VALUE_2.clone())?;
            storage.borrow_mut().write(&KEY_3, VALUE_3.clone())?;

            let borrowed_storage = storage.borrow();

            let mut iter = borrowed_storage.read_keys();
            assert!(&[KEY_1.clone(), KEY_2.clone(), KEY_3.clone()].contains(iter.next().unwrap()));
            assert!(&[KEY_1.clone(), KEY_2.clone(), KEY_3.clone()].contains(iter.next().unwrap()));

            storage.borrow_mut().delete(&KEY_2)?;

            iter.next();

            Ok(())
        })
    })
    .ok();
}

#[test]
fn test_iterator_with_concurrent_values_modification() {
    do_in_temp_directory(|path| {
        do_with_pojo(path, |storage| {
            let storage = RefCell::new(storage);

            storage.borrow_mut().write(&KEY_1, VALUE_1.clone())?;
            storage.borrow_mut().write(&KEY_1, VALUE_1.clone())?;
            storage.borrow_mut().write(&KEY_1, VALUE_1.clone())?;

            let borrowed_storage = storage.borrow();

            let mut iter = borrowed_storage.read_keys();

            assert!(&[KEY_1.clone(), KEY_2.clone(), KEY_3.clone()].contains(iter.next().unwrap()));
            assert!(&[KEY_1.clone(), KEY_2.clone(), KEY_3.clone()].contains(iter.next().unwrap()));

            storage.borrow_mut().write(&KEY_3, VALUE_2.clone())?;

            assert!(&[KEY_1.clone(), KEY_2.clone(), KEY_3.clone()].contains(iter.next().unwrap()));

            Ok(())
        })
    })
    .ok();
}

#[test]
#[should_panic(expected = "An attempt to accesss closed storage!")]
fn test_do_not_write_in_closed_state() {
    do_in_temp_directory(|path| {
        do_with_numbers(path, |mut storage| {
            assert_eq!(storage.read(&4)?, None);
            storage.write(&4, 5.0)?;
            assert_eq!(5.0, storage.read(&4)?.unwrap());
            storage.close()?;
            storage.write(&3, 5.0)
        })
    })
    .ok();
}

#[test]
#[should_panic(expected = "An attempt to accesss closed storage!")]
fn test_do_not_read_in_closed_state() {
    do_in_temp_directory(|path| {
        do_with_strings(path, |mut storage| {
            assert_eq!(storage.read(&"trololo".to_string())?, None);
            storage.write(&"trololo".to_string(), "yarr".to_string())?;
            assert_eq!(
                "yarr".to_string(),
                storage.read(&"trololo".to_string())?.unwrap()
            );
            storage.close()?;
            storage.read_keys();
            Ok(())
        })
    })
    .ok();
}

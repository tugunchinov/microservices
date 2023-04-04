#[allow(unused_imports)]
use super::{KEY_1, KEY_2, KEY_3, VALUE_1, VALUE_2, VALUE_3};
use crate::tests::utils::test_env::{
    do_in_temp_topic, do_with_numbers, do_with_pojo, do_with_strings,
};

#[tokio::test]
#[cfg(no_compile)]
async fn test_iterator_with_concurrent_keys_modification() {
    do_in_temp_topic(|config| {
        do_with_pojo(config, |mut storage| {
            storage
                .write(KEY_1.clone(), VALUE_1.clone())
                .expect("failed writing");
            storage
                .write(KEY_2.clone(), VALUE_2.clone())
                .expect("failed writing");
            storage
                .write(KEY_3.clone(), VALUE_3.clone())
                .expect("failed writing");

            let mut iter = storage.read_keys().expect("failed reading");
            assert!(&[KEY_1.clone(), KEY_2.clone(), KEY_3.clone()].contains(iter.next().unwrap()));
            assert!(&[KEY_1.clone(), KEY_2.clone(), KEY_3.clone()].contains(iter.next().unwrap()));

            storage.delete(&KEY_2).expect("failed deleting");

            iter.next();

            futures::future::ready(Ok(()))
        })
    })
    .await
    .unwrap();
}

#[tokio::test]
#[cfg(no_compile)]
async fn test_iterator_with_concurrent_values_modification() {
    do_in_temp_topic(|config| {
        do_with_pojo(config, |mut storage| {
            storage
                .write(KEY_1.clone(), VALUE_1.clone())
                .expect("failed writing");
            storage
                .write(KEY_2.clone(), VALUE_2.clone())
                .expect("failed writing");
            storage
                .write(KEY_3.clone(), VALUE_3.clone())
                .expect("failed writing");

            let mut iter = storage.read_keys().expect("failed reading");

            assert!(&[KEY_1.clone(), KEY_2.clone(), KEY_3.clone()].contains(iter.next().unwrap()));
            assert!(&[KEY_1.clone(), KEY_2.clone(), KEY_3.clone()].contains(iter.next().unwrap()));

            storage
                .write(KEY_3.clone(), VALUE_2.clone())
                .expect("failed writing");

            assert!(&[KEY_1.clone(), KEY_2.clone(), KEY_3.clone()].contains(iter.next().unwrap()));

            futures::future::ready(Ok(()))
        })
    })
    .await
    .unwrap();
}

#[tokio::test]
#[should_panic(expected = "closed")]
async fn test_do_not_write_in_closed_state() {
    do_in_temp_topic(|config| {
        do_with_numbers(config, |mut storage| {
            assert_eq!(storage.read(&4).expect("failed reading"), None);
            storage.write(4, 5.0).expect("failed writing");
            assert_eq!(5.0, *storage.read(&4).expect("failed reading").unwrap());
            futures::executor::block_on(storage.close()).unwrap();
            storage.write(3, 5.0).map(|_| {}).expect("failed writing");

            futures::future::ready(Ok(()))
        })
    })
    .await
    .unwrap();
}

#[tokio::test]
#[should_panic(expected = "closed")]
async fn test_do_not_read_in_closed_state() {
    do_in_temp_topic(|config| {
        do_with_strings(config, |mut storage| {
            assert_eq!(
                storage
                    .read(&"trololo".to_string())
                    .expect("failed reading"),
                None
            );
            storage
                .write("trololo".to_string(), "yarr".to_string())
                .expect("failed writing");
            assert_eq!(
                "yarr".to_string(),
                *storage
                    .read(&"trololo".to_string())
                    .expect("failed reading")
                    .unwrap()
            );
            futures::executor::block_on(storage.close()).unwrap();
            storage.read_keys().expect("faield reading").count();

            futures::future::ready(Ok(()))
        })
    })
    .await
    .unwrap();
}

use futures::FutureExt;
use std::{
    process::{Command, Stdio},
    time::Duration,
};

use crate::tests::{
    kv::setup,
    utils::test_env::{
        self, assert_fully_match, do_in_temp_topic, do_with_numbers, do_with_pojo, do_with_strings,
    },
};

use super::{KEY_1, KEY_2, VALUE_1, VALUE_2};

#[tokio::test]
async fn test_read_write() {
    setup();

    tokio::time::timeout(
        Duration::from_secs(2),
        test_env::do_in_temp_topic(|config| {
            do_with_strings(config, |mut storage| {
                storage
                    .write("foo".to_string(), "bar".to_string())
                    .expect("Failed writing");
                assert_eq!(
                    "bar".to_string(),
                    *storage
                        .read(&"foo".to_string())
                        .expect("Failed reading")
                        .unwrap()
                );
                assert_eq!(1, storage.size());

                assert_fully_match(storage.read_keys().unwrap(), &[&"foo".to_string()])
                    .expect("assertion failed");

                futures::future::ready(Ok(()))
            })
        }),
    )
    .await
    .unwrap()
    .unwrap();
}

#[tokio::test]
async fn test_persistence() {
    setup();

    test_env::do_in_temp_topic(|config| {
        let f1 = do_with_pojo(config.clone(), |mut storage| {
            storage
                .write(KEY_1.clone(), VALUE_1.clone())
                .map(|_| {})
                .expect("failed writing");

            futures::future::ready(Ok(()))
        });

        let f2 = do_with_pojo(config, |storage| {
            assert_eq!(
                *VALUE_1,
                *storage.read(&KEY_1).expect("failed reading").unwrap()
            );
            assert_eq!(1, storage.size());
            assert_fully_match(storage.read_keys().unwrap(), &[&*KEY_1]).expect("assertion failed");

            futures::future::ready(Ok(()))
        });

        f1.then(|_| f2)
    })
    .await
    .unwrap();

    test_env::do_in_temp_topic(|config| {
        do_with_pojo(config, |storage| {
            assert_eq!(storage.read(&KEY_1).expect("failed reading"), None);
            futures::future::ready(Ok(()))
        })
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn test_missing_key() {
    setup();

    test_env::do_in_temp_topic(|config| {
        do_with_numbers(config, |mut storage| {
            storage.write(4, 3.0).expect("failed writing");
            assert_eq!(*storage.read(&4).expect("failed reading").unwrap(), 3.0);
            assert_eq!(storage.read(&5).expect("failed reading"), None);
            assert_eq!(1, storage.size());
            assert_fully_match(storage.read_keys().expect("failed reading keys "), &[&4])
                .expect("assertion failed");

            futures::future::ready(Ok(()))
        })
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn test_multiple_modifications() {
    setup();

    do_in_temp_topic(|config| {
        let f1 = do_with_strings(config.clone(), |mut storage| {
            storage
                .write("foo".to_string(), "bar".to_string())
                .expect("failed writing");
            storage
                .write("bar".to_string(), "foo".to_string())
                .expect("failed writing");
            storage
                .write("yammy".to_string(), "nooo".to_string())
                .expect("failed writing");
            assert_eq!(
                "bar".to_string(),
                *storage
                    .read(&"foo".to_string())
                    .expect("failed reading")
                    .unwrap()
            );
            assert_eq!(
                "foo".to_string(),
                *storage
                    .read(&"bar".to_string())
                    .expect("failed reading")
                    .unwrap()
            );
            assert_eq!(
                "nooo".to_string(),
                *storage
                    .read(&"yammy".to_string())
                    .expect("failed reading")
                    .unwrap()
            );
            assert!(storage.exists(&"foo".to_string()).expect("failed reading"));
            assert_eq!(3, storage.size());
            assert_fully_match(
                storage.read_keys().unwrap(),
                &[&"bar".to_string(), &"foo".to_string(), &"yammy".to_string()],
            )
            .expect("assertion failed");

            futures::future::ready(Ok(()))
        });

        let f2 = do_with_strings(config.clone(), |storage| {
            assert_eq!(
                "bar".to_string(),
                *storage
                    .read(&"foo".to_string())
                    .expect("failed reading")
                    .unwrap()
            );
            assert_eq!(
                "foo".to_string(),
                *storage
                    .read(&"bar".to_string())
                    .expect("failed reading")
                    .unwrap()
            );
            assert_eq!(
                "nooo".to_string(),
                *storage
                    .read(&"yammy".to_string())
                    .expect("failed reading")
                    .unwrap()
            );
            assert!(storage.exists(&"bar".to_string()).expect("failed reading"));
            assert!(!storage.exists(&"yep".to_string()).expect("failed reading"));
            assert_eq!(3, storage.size());
            assert_fully_match(
                storage.read_keys().unwrap(),
                &[&"bar".to_string(), &"foo".to_string(), &"yammy".to_string()],
            )
            .expect("assertion failed");

            futures::future::ready(Ok(()))
        });

        let f3 = do_with_strings(config.clone(), |mut storage| {
            storage.delete(&"bar".to_string()).expect("failed deleting");
            storage
                .write("yammy".to_string(), "yeahs".to_string())
                .expect("failed writing");
            assert!(!storage.exists(&"bar".to_string()).expect("failed reading"));
            assert!(!storage.exists(&"yep".to_string()).expect("failed reading"));
            assert_eq!(2, storage.size());
            assert_fully_match(
                storage.read_keys().unwrap(),
                &[&"foo".to_string(), &"yammy".to_string()],
            )
            .expect("assertion failed");

            futures::future::ready(Ok(()))
        });

        let f4 = do_with_strings(config.clone(), |storage| {
            assert_eq!(
                "bar".to_string(),
                *storage
                    .read(&"foo".to_string())
                    .expect("faield reading")
                    .unwrap()
            );
            assert_eq!(
                storage.read(&"bar".to_string()).expect("failed reading"),
                None
            );
            assert_eq!(
                "yeahs".to_string(),
                *storage
                    .read(&"yammy".to_string())
                    .expect("failed reading")
                    .unwrap()
            );
            assert_eq!(2, storage.size());
            assert_fully_match(
                storage.read_keys().unwrap(),
                &[&"foo".to_string(), &"yammy".to_string()],
            )
            .expect("assertion failed");

            futures::future::ready(Ok(()))
        });

        f1.then(|_| f2.then(|_| f3.then(|_| f4)))
    })
    .await
    .unwrap()
}

#[tokio::test]
async fn test_persist_and_copy() {
    do_in_temp_topic(|config1| {
        let f1 = do_with_pojo(config1.clone(), |mut storage| {
            storage
                .write(KEY_1.clone(), VALUE_1.clone())
                .expect("failed writing");
            storage
                .write(KEY_2.clone(), VALUE_2.clone())
                .map(|_| {})
                .expect("failed writing");

            futures::future::ready(Ok(()))
        });

        let kcat_ps1 = Command::new("kcat")
            .args([
                "-b",
                &config1.brokers,
                "-C",
                "-t",
                &config1.topic,
                "-K:",
                "-e",
                "-o",
                "beginning",
            ])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let f2 = do_in_temp_topic(|config2| {
            let mut kcat_ps2 = Command::new("kcat")
                .args(["-b", &config2.brokers, "-P", "-t", &config2.topic, "-K:"])
                .stdin(Stdio::from(kcat_ps1.stdout.unwrap()))
                .spawn()
                .unwrap();

            kcat_ps2.wait().unwrap();

            do_with_pojo(config2, |storage| {
                assert_eq!(
                    *VALUE_1,
                    *storage.read(&KEY_1).expect("failed reading").unwrap()
                );
                assert_eq!(
                    *VALUE_2,
                    *storage.read(&KEY_2).expect("failed reading").unwrap()
                );
                assert_eq!(2, storage.size());
                assert_fully_match(storage.read_keys().unwrap(), &[&*KEY_1, &*KEY_2])
                    .expect("assertion failed");

                futures::future::ready(Ok(()))
            })
        });

        f1.then(|_| f2)
    })
    .await
    .unwrap();
}

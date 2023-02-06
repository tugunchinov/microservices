use crate::tests::utils::test_env::assert_fully_match;

use super::utils::test_env::{self, do_with_strings};

#[test]
fn test_read_write() {
    test_env::do_in_temp_directory(|path| {
        do_with_strings(path, |storage| {
            storage.write(&"foo".to_string(), "bar".to_string())?;
            assert_eq!(
                "bar".to_string(),
                storage.read(&"foo".to_string())?.unwrap()
            );
            assert_eq!(1, storage.size());

            //assert_fully_match(storage.read_keys(), ["foo".to_string()])

            Ok(())
        })
    })
    .ok();
}

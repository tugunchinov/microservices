use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    hash::Hash,
    path::{Path, PathBuf},
};

enum StorageState {
    Opened,
    Closed,
}

pub struct KeyValueStorage<K: Eq + Hash + Serialize, V: Serialize> {
    persistent_storage_path: PathBuf,
    storage: HashMap<K, V>,
    state: StorageState,
}

const PERSISTENT_STORAGE_FILE_NAME: &str = "db";

impl<K: Eq + Hash + Serialize, V: Serialize> KeyValueStorage<K, V> {
    pub fn new<'de>(path: &Path) -> Result<Self>
    where
        K: Deserialize<'de>,
        V: Deserialize<'de>,
    {
        let mut persistent_storage_path = path.to_path_buf();
        persistent_storage_path.push(PERSISTENT_STORAGE_FILE_NAME);

        let storage = if persistent_storage_path.exists() {
            let mut deserializer = rmp_serde::Deserializer::new(
                File::options().read(true).open(&persistent_storage_path)?,
            );

            HashMap::deserialize(&mut deserializer)?
        } else {
            HashMap::new()
        };

        Ok(Self {
            persistent_storage_path,
            storage,
            state: StorageState::Opened,
        })
    }

    pub fn read(&self, key: &K) -> Result<Option<&V>> {
        match self.state {
            StorageState::Opened => Ok(self.storage.get(key)),
            StorageState::Closed => {
                bail!("An attempt to accesss closed storage!")
            }
        }
    }

    pub fn exists(&self, key: &K) -> Result<bool> {
        match self.state {
            StorageState::Opened => Ok(self.storage.contains_key(key)),
            StorageState::Closed => {
                bail!("An attempt to accesss closed storage!")
            }
        }
    }

    pub fn write(&mut self, key: K, value: V) -> Result<Option<V>> {
        match self.state {
            StorageState::Opened => Ok(self.storage.insert(key, value)),
            StorageState::Closed => {
                bail!("An attempt to accesss closed storage!")
            }
        }
    }

    pub fn delete(&mut self, key: &K) -> Result<Option<V>> {
        match self.state {
            StorageState::Opened => Ok(self.storage.remove(key)),
            StorageState::Closed => {
                bail!("An attempt to accesss closed storage!")
            }
        }
    }

    pub fn read_keys(&self) -> Result<impl Iterator<Item = &K>> {
        match self.state {
            StorageState::Opened => Ok(self.storage.keys()),
            StorageState::Closed => {
                bail!("An attempt to accesss closed storage!")
            }
        }
    }

    pub fn size(&self) -> usize {
        self.storage.len()
    }

    pub fn flush(&mut self) -> Result<()> {
        let mut serializer = rmp_serde::Serializer::new(
            File::options()
                .write(true)
                .create(true)
                .open(&self.persistent_storage_path)?,
        );
        self.storage
            .serialize(&mut serializer)
            .map_err(|e| anyhow!(e))
    }

    pub fn close(&mut self) -> Result<()> {
        self.state = StorageState::Closed;

        self.flush()
    }
}

impl<K: Eq + Hash + Serialize, V: Serialize> Drop for KeyValueStorage<K, V> {
    fn drop(&mut self) {
        self.close().unwrap()
    }
}

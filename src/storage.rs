use anyhow::Result;
use std::{marker::PhantomData, path::PathBuf};

pub struct KeyValueStorage<K, V> {
    phantom_key: PhantomData<K>,
    phantom_value: PhantomData<V>,
}

pub struct KeyIterator<'a, K> {
    _phantom_data: &'a PhantomData<K>,
}

impl<K, V> KeyValueStorage<K, V> {
    pub fn new(_p: PathBuf) -> Result<Self> {
        Ok(Self {
            phantom_key: PhantomData::default(),
            phantom_value: PhantomData::default(),
        })
    }

    pub fn read(&self, _key: &K) -> Result<Option<V>> {
        unimplemented!()
    }

    pub fn exists(&self, _key: &K) -> Result<bool> {
        unimplemented!()
    }

    pub fn write(&mut self, _key: &K, _value: V) -> Result<()> {
        unimplemented!()
    }

    pub fn delete(&mut self, _key: &K) -> Result<()> {
        unimplemented!()
    }

    pub fn read_keys(&self) -> KeyIterator<K> {
        unimplemented!()
    }

    pub fn size(&self) -> usize {
        unimplemented!()
    }

    pub fn flush(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub fn close(&mut self) -> Result<()> {
        unimplemented!()
    }
}

// TODO:
// impl<K, V> Drop for KeyValueStorage<K, V> {
//     fn drop(&mut self) {
//         self.close().unwrap()
//     }
// }

impl<'a, K> Iterator for KeyIterator<'a, K> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

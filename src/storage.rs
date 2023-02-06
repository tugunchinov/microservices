use anyhow::Result;
use std::{marker::PhantomData, path::PathBuf};

pub struct KeyValueStorage<K, V> {
    phantom_key: PhantomData<K>,
    phantom_value: PhantomData<V>,
}

impl<K, V> KeyValueStorage<K, V> {
    pub fn new(p: PathBuf) -> Result<Self> {
        todo!()
    }

    pub fn read(&self, key: &K) -> Result<Option<K>> {
        todo!()
    }

    pub fn exists(&self, key: &K) -> Result<bool> {
        todo!()
    }

    pub fn write(&mut self, key: &K, value: V) -> Result<()> {
        todo!()
    }

    pub fn delete(&mut self, key: &K) -> Result<()> {
        todo!()
    }

    // TODO: iter

    pub fn size(&self) -> usize {
        todo!()
    }

    pub fn flush(&mut self) -> Result<()> {
        todo!()
    }

    // TODO: close

    pub fn close(&mut self) -> Result<()> {
        todo!()
    }
}

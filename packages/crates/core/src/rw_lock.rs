use crate::prelude::*;



// ==============
// === RwLock ===
// ==============

#[derive(Debug, Default)]
pub struct RwLock<T> {
    inner: sync::RwLock<T>,
}

impl<T> RwLock<T> {
    pub(crate) fn new(value: T) -> Self {
        Self { inner: sync::RwLock::new(value) }
    }

    pub fn write(&self) -> sync::RwLockWriteGuard<'_, T> {
        self.inner.write().expect("Write")
    }

    pub fn read(&self) -> sync::RwLockReadGuard<'_, T> {
        self.inner.read().expect("Read")
    }
}

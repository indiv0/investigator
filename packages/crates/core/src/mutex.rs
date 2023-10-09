use crate::prelude::*;

#[cfg(not(test))]
type MutexInner<T> = sync::Mutex<T>;
#[cfg(test)]
type MutexInner<T> = parking_lot::Mutex<T>;

#[cfg(not(test))]
type MutexGuard<'a, T> = sync::MutexGuard<'a, T>;
#[cfg(test)]
type MutexGuard<'a, T> = parking_lot::MutexGuard<'a, T>;

#[derive(Debug, Default)]
pub(crate) struct Mutex<T> {
    inner: MutexInner<T>,
}

impl<T> Mutex<T> {
    pub(crate) fn new(value: T) -> Self {
        Self { inner: MutexInner::new(value) }
    }

    pub(crate) fn lock(&self) -> MutexGuard<'_, T> {
        #[cfg(not(test))]
        {
            self.inner.lock().expect("Lock")
        }
        #[cfg(test)]
        {
            self.inner.lock()
        }
    }
}

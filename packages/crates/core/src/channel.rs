use crate::prelude::*;



// ===============
// === Channel ===
// ===============

pub fn channel<T>() -> (Sender<T>, Receiver<T>)
where
    T: Send + 'static,
{
    let (sender, receiver) = mpsc::channel();
    let sender = Mutex::new(sender);
    let sender = Arc::new(sender);
    let sender = Sender { inner: sender };
    let receiver = Mutex::new(receiver);
    let receiver = Receiver { inner: receiver };
    (sender, receiver)
}



// ===================
// === SenderInner ===
// ===================

trait SenderInner<T> {
    fn send(&self, value: T);
}

impl<T> SenderInner<T> for Mutex<mpsc::Sender<T>> {
    fn send(&self, value: T) {
        self.lock().send(value).expect("Send value")
    }
}



// ==============
// === Sender ===
// ==============

#[must_use]
pub struct Sender<T> {
    inner: Arc<dyn SenderInner<T> + Send + Sync>,
}

static_assertions::assert_impl_all!(Sender<()>: Clone, Send, Sync);

impl<T> Sender<T> {
    pub fn send(&self, value: T) {
        self.inner.send(value)
    }
}

impl<T> Sender<T>
where
    T: 'static,
{
    pub(crate) fn map_input<U, F>(&self, func: F) -> Sender<U>
    where
        F: Fn(U) -> T + Send + Sync + 'static,
    {
        Sender {
            inner: Arc::new(MappedInner {
                sender: Arc::clone(&self.inner),
                func,
            }),
        }
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> Debug for Sender<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sender").finish()
    }
}



// ===================
// === MappedInner ===
// ===================

#[derive(Clone)]
#[must_use]
pub(crate) struct MappedInner<T, F> {
    sender: Arc<dyn SenderInner<T> + Send + Sync>,
    func: F,
}

impl<T, U, F> SenderInner<U> for MappedInner<T, F>
where
    F: Fn(U) -> T,
{
    fn send(&self, value: U) {
        let value = (self.func)(value);
        self.sender.send(value)
    }
}

impl<T, F> Debug for MappedInner<T, F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("SenderInner").finish()
    }
}



// ================
// === Receiver ===
// ================

#[derive(Debug)]
#[must_use]
pub struct Receiver<T> {
    inner: Mutex<mpsc::Receiver<T>>,
}

static_assertions::assert_impl_all!(Receiver<PathBuf>: Send, Sync);

impl<T> Receiver<T> {
    pub(crate) fn try_recv(&self) -> Result<T, mpsc::TryRecvError> {
        self.inner.lock().try_recv()
    }

    pub fn recv(&self) -> Option<T> {
        match self.try_recv() {
            Ok(value) => Some(value),
            Err(mpsc::TryRecvError::Empty) => None,
            Err(mpsc::TryRecvError::Disconnected) => panic!("Receive"),
        }
    }

    pub(crate) fn drain(&self) -> Drain<'_, T> {
        Drain { receiver: self }
    }
}



// =============
// === Drain ===
// =============

#[must_use]
pub(crate) struct Drain<'a, T> {
    receiver: &'a Receiver<T>,
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.receiver.recv()
    }
}

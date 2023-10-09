use crate::prelude::*;

use crate::executor;
use crate::request;
use crate::shell::app;

mod shell_request;



// =================
// === Operation ===
// =================

pub trait Operation: Debug + Send + 'static {
    type Output: Debug + Send;
}



// ===================
// === WithContext ===
// ===================

pub(crate) trait WithContext<E, V> {
    fn new_with_context(context: ProtoContext<E, V>) -> app::Capabilities;
}



// ====================
// === ProtoContext ===
// ====================

#[derive(Debug)]
#[must_use]
pub(crate) struct ProtoContext<E, V> {
    task_spawner: executor::Spawner,
    shell_spawner: Sender<E>,
    app_sender: Sender<V>,
}

impl<E, V> ProtoContext<E, V>
where
    E: 'static,
{
    pub(crate) fn new(
        task_spawner: executor::Spawner,
        shell_spawner: Sender<E>,
        app_sender: Sender<V>,
    ) -> Self {
        Self { task_spawner, shell_spawner, app_sender }
    }

    /// Specialize the CapabilityContext to a specific capability, wrapping its operations into an
    /// Effect `E`. The `func` argument will typically be an Effect variant constructor, but can be
    /// any function taking the capability's operation type and returning the effect type.
    ///
    /// This will likely only be called from the implementation of [`WithContext`] for the app's
    /// `Capabilities` type. You should not need to call this function directly.
    pub fn specialize<O, F>(&self, func: F) -> CapabilityContext<O, V>
    where
        F: Fn(request::Request<O>) -> E + Send + Sync + 'static,
        O: Operation,
    {
        CapabilityContext::new(
            self.task_spawner.clone(),
            self.shell_spawner.map_input(func),
            self.app_sender.clone(),
        )
    }
}



// =========================
// === CapabilityContext ===
// =========================

#[derive(Debug)]
#[must_use]
pub(crate) struct CapabilityContext<O, E>
where
    O: Operation,
{
    inner: Arc<ContextInner<O, E>>,
}

static_assertions::assert_impl_all!(CapabilityContext<Unclone, Unclone>: Clone, Debug, Sync);

impl<O, E> CapabilityContext<O, E>
where
    O: Operation,
{
    pub(crate) fn new(
        task_spawner: executor::Spawner,
        shell_spawner: Sender<request::Request<O>>,
        app_sender: Sender<E>,
    ) -> Self {
        let inner = ContextInner { task_spawner, shell_spawner, app_sender };
        let inner = Arc::new(inner);
        Self { inner }
    }

    /// Send an event to the app. The event will be processed on the next run of the update loop.
    /// You can call `update_app` several times, the events will be queued up and processed
    /// sequentially after your async task either `await`s or finishes.
    pub(crate) fn update_app(&self, event: E) {
        self.inner.app_sender.send(event);
    }

    pub(crate) fn spawn(&self, f: impl Future<Output = ()> + Send + 'static) {
        self.inner.task_spawner.spawn(f)
    }

    /// Send an effect request to the shell in a fire and forget fashion. The provided `operation`
    /// does not expect anything to be returned back.
    pub(crate) async fn notify_shell(&self, operation: O) {
        // This function might look like it doesn't need to be async but it's important that it is.
        // It forces all capabilities to spawn onto the executor which keeps the ordering of effects
        // consistent with their function calls.
        self.inner
            .shell_spawner
            .send(request::Request::resolves_never(operation));
    }

    fn send_request(&self, request: request::Request<O>) {
        self.inner.shell_spawner.send(request)
    }
}

impl<O, E> Clone for CapabilityContext<O, E>
where
    O: Operation,
{
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

#[derive(Debug)]
#[must_use]
struct ContextInner<O, E>
where
    O: Operation,
{
    task_spawner: executor::Spawner,
    shell_spawner: Sender<request::Request<O>>,
    app_sender: Sender<E>,
}

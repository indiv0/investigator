use crate::prelude::*;

use crate::context;
use crate::request;



// ===================
// === SharedState ===
// ===================

#[derive(Debug)]
#[must_use]
struct SharedState<T> {
    inner: Arc<Mutex<SharedStateInner<T>>>,
}

static_assertions::assert_impl_all!(SharedState<Unclone>: Clone);
static_assertions::assert_impl_all!(SharedState<Unsync>: Sync);

impl<T> Clone for SharedState<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

#[derive(Debug)]
#[must_use]
struct SharedStateInner<T> {
    result: Option<T>,
    waker: Option<Waker>,
}



// ====================
// === ShellRequest ===
// ====================

#[must_use]
pub(crate) struct ShellRequest<T> {
    shared_state: SharedState<T>,
    send_request: Option<Box<dyn FnOnce() + Send>>,
}

impl<T> Future for ShellRequest<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // If there's still a request to send, take it and send it
        if let Some(send_request) = self.send_request.take() {
            send_request();
        }

        let mut shared_state = self.shared_state.inner.lock();

        // If a result has been delivered, we're ready to continue.
        // Else we're pending with the waker from context.
        match shared_state.result.take() {
            Some(result) => Poll::Ready(result),
            None => {
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

impl<T> Unpin for ShellRequest<T> {}



// =========================
// === CapabilityContext ===
// =========================

impl<O, E> crate::CapabilityContext<O, E>
where
    O: context::Operation,
    E: 'static,
{
    pub(crate) fn request_from_shell(&self, operation: O) -> ShellRequest<O::Output> {
        let result = None;
        let waker = None;

        let shared_state = SharedStateInner { result, waker };
        let shared_state = Mutex::new(shared_state);
        let shared_state = Arc::new(shared_state);
        let shared_state = SharedState { inner: shared_state };

        // FIXME [NP]: remove
        // Our callback holds a weak pointer to avoid circular references
        // from shared_state -> send_request -> request -> shared_state
        //let callback_shared_state = Arc::downgrade(&shared_state);
        let callback_shared_state = shared_state.clone();

        let request = request::Request::resolves_once(operation, move |result| {
            // FIXME [NP]: remove
            //let Some(shared_state) = callback_shared_state.upgrade() else {
            //    // The ShellRequest was dropped before we were called, so just
            //    // do nothing.
            //    return;
            //};

            let mut shared_state = callback_shared_state.inner.lock();

            // Attach the result to the shared state of the future
            shared_state.result = Some(result);
            // Signal the executor to wake the task holding this future
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        let context = self.clone();
        let send_request = move || context.send_request(request);
        let send_request = Box::new(send_request) as _;
        let send_request = Some(send_request);
        ShellRequest { shared_state, send_request }
    }
}



// =============
// === Tests ===
// =============

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use crate::channel;
    use crate::context;
    use crate::executor;

    #[derive(Clone, Copy, Debug)]
    struct TestOperation;

    impl context::Operation for TestOperation {
        type Output = ();
    }

    #[test]
    fn test_effect_future() {
        let (request_sender, requests) = channel::channel();
        let (event_sender, events) = channel::channel();
        let (spawner, executor) = executor::spawner_and_executor();
        let capability_context = crate::CapabilityContext::new(
            spawner.clone(),
            request_sender,
            event_sender.clone(),
        );

        let future = capability_context.request_from_shell(TestOperation);

        // The future hasn't been awaited so we shouldn't have any requests.
        assert_matches!(requests.recv(), None);
        assert_matches!(events.recv(), None);

        // It also shouldn't have spawned anything so check that
        executor.run_all();
        assert_matches!(requests.recv(), None);
        assert_matches!(events.recv(), None);

        spawner.spawn(async move {
            future.await;
            event_sender.send(());
        });

        // We still shouldn't have any requests
        assert_matches!(requests.recv(), None);
        assert_matches!(events.recv(), None);

        executor.run_all();
        let mut request = requests.recv().expect("we should have a request here");
        assert_matches!(requests.recv(), None);
        assert_matches!(events.recv(), None);

        // FIXME [NP]: Uncomment?
        request.resolve(()).expect("request should resolve");

        assert_matches!(requests.recv(), None);
        assert_matches!(events.recv(), None);

        executor.run_all();
        assert_matches!(requests.recv(), None);
        assert_matches!(events.recv(), Some(()));
        // FIXME [NP]: Uncomment?
        //assert_matches!(events.recv(), None);
    }
}

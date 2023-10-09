use crate::prelude::*;

use crate::channel;
use crate::context;
use crate::executor;
use crate::request;
use crate::resolve;



// ==============
// === Update ===
// ==============

/// Update test helper holds the result of running an app update using [`AppTester::update`] or
/// resolving a request with [`AppTester::resolve`].
#[derive(Clone, Debug, Default)]
pub struct Update<E, V> {
    /// Effects requested from the update run.
    pub(crate) effects: Vec<E>,
    /// Events dispatched from the update run.
    pub events: Vec<V>,
}

impl<E, V> Update<E, V> {
    pub fn into_effects(self) -> impl Iterator<Item = E> {
        self.effects.into_iter()
    }

    pub fn effects(&self) -> impl Iterator<Item = &E> {
        self.effects.iter()
    }

    pub fn effects_mut(&mut self) -> impl Iterator<Item = &mut E> {
        self.effects.iter_mut()
    }
}



// ===================
// === App Context ===
// ===================

#[derive(Debug)]
#[must_use]
struct AppContext<E, V> {
    commands: Receiver<E>,
    events: Receiver<V>,
    executor: executor::Executor,
}

impl<E, V> AppContext<E, V> {
    fn updates(self: &Rc<Self>) -> Update<E, V> {
        self.executor.run_all();
        let effects = self.commands.drain().collect();
        let events = self.events.drain().collect();

        Update { effects, events }
    }
}



// ==================
// === App Tester ===
// ==================

#[derive(Debug)]
#[must_use]
pub struct AppTester<V> {
    app: App,
    capabilities: Capabilities,
    context: Rc<AppContext<V, Event>>,
}

impl<E> AppTester<E> {
    /// Run the app's `update` function with an event and a model state
    ///
    /// You can use the resulting [`Update`] to inspect the effects which were requested and
    /// potential further events dispatched by capabilities.
    ///
    /// [`Update`]: crate::testing::Update
    pub fn update(&self, event: Event, model: &mut Model) -> Update<E, Event> {
        self.app.update(event, model, &self.capabilities);
        self.context.updates()
    }

    /// Resolve an effect `request` from previous update with an operation output.
    ///
    /// This potentially runs the app's `update` function if the effect is completed, and produce
    /// another `Update`.
    pub fn resolve<O: context::Operation>(
        &self,
        request: &mut request::Request<O>,
        value: O::Output,
    ) -> Result<Update<E, Event>, resolve::ResolveError> {
        request.resolve(value)?;
        Ok(self.context.updates())
    }
}

impl Default for AppTester<Effect> {
    fn default() -> Self {
        let (shell_spawner, requests) = channel::channel();
        let (app_sender, events) = channel::channel();
        let (task_spawner, executor) = executor::spawner_and_executor();
        let context = crate::ProtoContext::new(
            task_spawner.clone(),
            shell_spawner,
            app_sender.clone(),
        );

        Self {
            app: App::default(),
            capabilities: Capabilities::new_with_context(context),
            context: Rc::new(AppContext {
                commands: requests,
                events,
                executor,
            })
        }
    }
}

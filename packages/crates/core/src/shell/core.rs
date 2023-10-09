use crate::prelude::*;

use crate::channel;
use crate::context;
use crate::executor;
use crate::hash;
use crate::key_value;
use crate::render;
use crate::request;
use crate::shell::app;
use crate::walkdir;



// ==============
// === Effect ===
// ==============

#[derive(Debug)]
#[must_use]
pub enum Effect {
    KeyValue(request::Request<key_value::protocol::KeyValueOperation>),
    Walkdir(request::Request<walkdir::protocol::WalkdirRequest>),
    Render(request::Request<render::RenderOperation>),
    Hash(request::Request<hash::protocol::HashRequest>),
}



// ============
// === Core ===
// ============

#[derive(Debug)]
#[must_use]
pub struct Core {
    pub model: RwLock<app::Model>,
    app: app::App,
    capabilities: app::Capabilities,
    executor: executor::Executor,
    capability_events: Receiver<app::Event>,
    requests: Receiver<Effect>,
}

impl Core {
    pub(crate) fn new() -> Self {
        let (task_spawner, executor) = executor::spawner_and_executor();
        let (shell_spawner, shell_receiver) = channel::channel::<Effect>();
        let (event_sender, event_receiver) = channel::channel();
        let model = <app::Model as Default>::default();
        let model = RwLock::new(model);
        let app = app::App::default();
        let context = ProtoContext::new(task_spawner, shell_spawner, event_sender);
        let capabilities = app::Capabilities::new_with_context(context);
        Self {
            model,
            app,
            executor,
            capabilities,
            capability_events: event_receiver,
            requests: shell_receiver
        }
    }

    pub fn process_event(&self, event: app::Event) -> Vec<Effect> {
        println!("Core process event: {event:?}.");
        let mut model = self.model.write();
        self.app.update(event, &mut model, &self.capabilities);
        self.process()
    }

    /// Resolve an effect `request` for operation `O` with the corresponding result.
    ///
    /// Note that the `request` is borrowed mutably. When a request that is expected to only be
    /// resolved once is passed in, it will be consumed and changed to a request which can no longer
    /// be resolved.
    pub fn resolve<O>(&self, request: &mut request::Request<O>, result: O::Output) -> Vec<Effect>
    where
        O: context::Operation,
    {
        //println!("Core resolve: {request:?}");
        // FIXME [NP]: clean
        //println!("Core resolve: {request:?}, {result:?}.");
        let resolve_result = request.resolve(result);
        debug_assert!(resolve_result.is_ok());
        self.process()
    }

    fn process(&self) -> Vec<Effect> {
        //println!("Core process.");
        self.executor.run_all();
        while let Some(e) = self.capability_events.recv() {
            let mut model = self.model.write();
            self.app.update(e, &mut model, &self.capabilities);
            drop(model);
            self.executor.run_all();
        }
        self.requests.drain().collect()
    }

    /// Get the current state of the app's view model.
    pub fn view(&self) -> app::ViewModel {
        let model = self.model.read();
        self.app.view(&model)
    }
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}

use crate::prelude::*;

use crate::shell::app;

pub(crate) mod protocol;

pub use crate::walkdir::protocol::WalkdirResponse;



// ===============
// === Walkdir ===
// ===============

#[derive(Debug)]
#[must_use]
pub(crate) struct Walkdir {
    context: CapabilityContext<protocol::WalkdirRequest, app::Event>,
    effect_sender: Arc<dyn protocol::EffectSender + Send + Sync>,
}

impl Walkdir {
    fn new(context: CapabilityContext<protocol::WalkdirRequest, app::Event>, effect_sender: impl protocol::EffectSender + Send + Sync + 'static) -> Self {
        let effect_sender = Arc::new(effect_sender);
        Self { context, effect_sender }
    }

    pub(crate) fn run(&self, path: PathBuf) -> RequestBuilder {
        RequestBuilder { capability: self.clone(), path }
    }
}

impl Clone for Walkdir {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            effect_sender: self.effect_sender.clone(),
        }
    }
}

impl From<CapabilityContext<protocol::WalkdirRequest, app::Event>> for Walkdir {
    fn from(context: CapabilityContext<protocol::WalkdirRequest, app::Event>) -> Self {
        Self::new(context.clone(), context)
    }
}



// ======================
// === RequestBuilder ===
// ======================

#[derive(Clone, Debug)]
#[must_use]
pub(crate) struct RequestBuilder {
    capability: Walkdir,
    path: PathBuf,
}

impl RequestBuilder {
    pub(crate) fn send(self, make_event: impl Fn(Vec<PathBuf>) -> app::Event + Send + 'static) {
        let capability = self.capability;
        let path = self.path;
        let context = capability.context.clone();
        context.spawn(async move {
            let request = protocol::WalkdirRequest { path };
            let response = capability.effect_sender.send(request).await;
            let response = response.paths;
            capability.context.update_app(make_event(response));
        })
    }
}

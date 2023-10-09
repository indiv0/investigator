use crate::prelude::*;

use crate::context;



// ==============
// === Export ===
// ==============

pub(crate) mod protocol;

pub use crate::hash::protocol::HashResponse;



// ============
// === Hash ===
// ============

#[derive(Debug)]
pub(crate) struct Hash<E> {
    context: CapabilityContext<protocol::HashRequest, E>,
    effect_sender: Arc<dyn protocol::EffectSender + Send + Sync>,
}

/// Public API of the capability, called by `App::update`.
impl<E> Hash<E>
where
    E: 'static,
{
    pub(crate) fn new(context: CapabilityContext<protocol::HashRequest, E>, effect_sender: impl protocol::EffectSender + Send + Sync + 'static) -> Self {
        let effect_sender = Arc::new(effect_sender);
        Self { context, effect_sender }
    }


    /// Call `hash` from [`App::update`] to signal to the Shell that a file needs to be hashed.
    ///
    /// [`App::update`]: crate::shell::app::App::update
    pub(crate) fn hash(&self, path: PathBuf) -> RequestBuilder<E> {
        RequestBuilder { capability: self.clone(), path }
    }
}

impl<E> Clone for Hash<E> {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            effect_sender: self.effect_sender.clone(),
        }
    }
}

impl<E> From<CapabilityContext<protocol::HashRequest, E>> for Hash<E>
where
    E: Debug + 'static,
{
    fn from(context: CapabilityContext<protocol::HashRequest, E>) -> Self {
        Self::new(context.clone(), context)
    }
}

// FIXME [NP]: Remove?
//impl<Ev> context::Capability<Ev> for Render<Ev> {
//    type Operation = RenderOperation;
//    type MappedSelf<MappedEv> = Render<MappedEv>;
//
//    fn map_event<F, NewEv>(&self, f: F) -> Self::MappedSelf<NewEv>
//    where
//        F: Fn(NewEv) -> Ev + Send + Sync + Copy + 'static,
//        Ev: 'static,
//        NewEv: 'static,
//    {
//        Render::new(self.context.map_event(f))
//    }
//}



// ======================
// === RequestBuilder ===
// ======================

#[derive(Clone, Debug)]
#[must_use]
pub(crate) struct RequestBuilder<E> {
    capability: Hash<E>,
    path: PathBuf,
}

impl<E> RequestBuilder<E>
where
    E: 'static,
{
    pub(crate) fn send(self, make_event: impl Fn(PathBuf, String) -> E + Send + 'static) {
        let capability = self.capability;
        let path = self.path;
        let context = capability.context.clone();
        context.spawn(async move {
            let request = protocol::HashRequest { path: path.clone() };
            let response = capability.effect_sender.send(request).await;
            let response = response.hash;
            capability.context.update_app(make_event(path, response));
        })
    }
}

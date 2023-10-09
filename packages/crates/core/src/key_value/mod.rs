use crate::prelude::*;



// ==============
// === Export ===
// ==============

pub(crate) mod protocol;

pub use crate::key_value::protocol::KeyValueOutput;



// ================
// === KeyValue ===
// ================

#[derive(Debug)]
pub(crate) struct KeyValue<E> {
    context: CapabilityContext<protocol::KeyValueOperation, E>,
    effect_sender: Arc<dyn protocol::EffectSender + Send + Sync>,
}

/// Public API of the capability, called by `App::update`.
impl<E> KeyValue<E>
where
    E: 'static,
{
    pub(crate) fn new(context: CapabilityContext<protocol::KeyValueOperation, E>, effect_sender: impl protocol::EffectSender + Send + Sync + 'static) -> Self {
        let effect_sender = Arc::new(effect_sender);
        Self { context, effect_sender }
    }


    /// Read a value under `key`, will dispatch the event with a
    /// `KeyValueOutput::Read(Option<Vec<u8>>)` as payload
    pub(crate) fn read<F>(&self, key: &str, make_event: F)
    where
        F: Fn(KeyValueOutput) -> E + Send + Sync + 'static,
    {
        let ctx = self.context.clone();
        let key = key.to_string();
        self.context.spawn(async move {
            let output = ctx.request_from_shell(protocol::KeyValueOperation::Read(key)).await;
            ctx.update_app(make_event(output))
        });
    }

    /// Set `key` to be the provided `value`. Typically the bytes would be a value
    /// serialized/deserialized by the app.
    ///
    /// Will dispatch the event with a `KeyValueOutput::Write(bool)` as payload
    pub fn write<F>(&self, key: &str, value: Vec<u8>, make_event: F)
    where
        F: Fn(KeyValueOutput) -> E + Send + Sync + 'static,
    {
        self.context.spawn({
            let context = self.context.clone();
            let key = key.to_string();
            async move {
                let resp = context
                    .request_from_shell(KeyValueOperation::Write(key, value))
                    .await;

                context.update_app(make_event(resp))
            }
        });
    }
}

impl<E> Clone for KeyValue<E> {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            effect_sender: self.effect_sender.clone(),
        }
    }
}

impl<E> From<CapabilityContext<protocol::KeyValueOperation, E>> for KeyValue<E>
where
    E: Debug + 'static,
{
    fn from(context: CapabilityContext<protocol::KeyValueOperation, E>) -> Self {
        Self::new(context.clone(), context)
    }
}

use crate::prelude::*;

use crate::context;



// =========================
// === KeyValueOperation ===
// =========================

#[derive(Clone, Debug)]
pub enum KeyValueOperation {
    /// Read bytes stored under a key.
    Read(String),
    /// Write bytes under a key.
    Write(String, Vec<u8>),
}

impl context::Operation for KeyValueOperation {
    type Output = KeyValueOutput;
}



// ======================
// === KeyValueOutput ===
// ======================

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KeyValueOutput {
    Read(Result<Vec<u8>, ()>),
    Write(Result<(), ()>),
}



// ====================
// === EffectSender ===
// ====================

pub(crate) trait EffectSender: Debug {
    fn send<'a>(&'a self, effect: KeyValueOperation) -> BoxFutureLifetime<'a, KeyValueOutput>;
}

impl<E> EffectSender for CapabilityContext<KeyValueOperation, E>
where
    E: Debug + 'static,
{
    fn send<'a>(&'a self, effect: KeyValueOperation) -> BoxFutureLifetime<'a, KeyValueOutput> {
        async fn run<E>(this: &CapabilityContext<KeyValueOperation, E>, effect: KeyValueOperation) -> KeyValueOutput
        where
            E: 'static,
        {
            this.request_from_shell(effect).await
        }

        Box::pin(run(self, effect))
    }
}



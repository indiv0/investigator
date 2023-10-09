use crate::prelude::*;

use crate::context;



// ===================
// === HashRequest ===
// ===================

#[derive(Clone, Debug, Default)]
pub struct HashRequest {
    pub path: PathBuf,
}

impl context::Operation for HashRequest {
    type Output = HashResponse;
}



// ====================
// === HashResponse ===
// ====================

#[derive(Clone, Debug, Default)]
pub struct HashResponse {
    pub hash: String,
}



// ====================
// === EffectSender ===
// ====================

pub(crate) trait EffectSender: Debug {
    fn send<'a>(&'a self, effect: HashRequest) -> BoxFutureLifetime<'a, HashResponse>;
}

impl<E> EffectSender for CapabilityContext<HashRequest, E>
where
    E: Debug + 'static,
{
    fn send<'a>(&'a self, effect: HashRequest) -> BoxFutureLifetime<'a, HashResponse> {
        async fn run<E>(this: &CapabilityContext<HashRequest, E>, effect: HashRequest) -> HashResponse
        where
            E: 'static,
        {
            this.request_from_shell(effect).await
        }

        Box::pin(run(self, effect))
    }
}


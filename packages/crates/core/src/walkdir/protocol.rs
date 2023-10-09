use crate::prelude::*;

use crate::context;
use crate::shell::app;



// ======================
// === WalkdirRequest ===
// ======================

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WalkdirRequest {
    pub path: PathBuf,
}

impl context::Operation for WalkdirRequest {
    type Output = WalkdirResponse;
}



// =======================
// === WalkdirResponse ===
// =======================

#[derive(Clone, Debug, Default)]
pub struct WalkdirResponse {
    pub paths: Vec<PathBuf>,
}



// ====================
// === EffectSender ===
// ====================

pub(crate) trait EffectSender: Debug {
    fn send<'a>(&'a self, effect: WalkdirRequest) -> BoxFutureLifetime<'a, WalkdirResponse>;
}

impl EffectSender for CapabilityContext<WalkdirRequest, app::Event> {
    fn send<'a>(&'a self, effect: WalkdirRequest) -> BoxFutureLifetime<'a, WalkdirResponse> {
        async fn run(this: &CapabilityContext<WalkdirRequest, app::Event>, effect: WalkdirRequest) -> WalkdirResponse {
            this.request_from_shell(effect).await
        }

        Box::pin(run(self, effect))
    }
}

use crate::prelude::*;

use crate::context;



// =======================
// === RenderOperation ===
// =======================

/// The single operation `Render` implements.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RenderOperation;

impl context::Operation for RenderOperation {
    type Output = ();
}



// ==============
// === Render ===
// ==============

/// Use an instance of `Render` to notify the Shell that it should update the user interface. This
/// assumes a declarative UI framework is used in the Shell, which will take the ViewModel provided
/// by [`Core::view`] and reconcile the new UI state based on the view model with
/// the previous one.
///
/// For imperative UIs, the Shell will need to understand the difference between the two view models
/// and update the user interface accordingly.
///
/// [`Core::view`]: crate::shell::core::Core::view
#[derive(Clone, Debug)]
pub(crate) struct Render<E> {
    context: CapabilityContext<RenderOperation, E>,
}

/// Public API of the capability, called by `App::update`.
impl<E> Render<E>
where
    E: 'static,
{
    pub(crate) fn new(context: CapabilityContext<RenderOperation, E>) -> Self {
        Self { context }
    }

    /// Call `render` from [`App::update`] to signal to the Shell that
    /// UI should be re-drawn.
    ///
    /// [`App::update`]: crate::shell::app::App::update
    pub(crate) fn render(&self) {
        let context = self.context.clone();
        self.context.spawn(async move {
            context.notify_shell(RenderOperation).await;
        });
    }
}

impl<E> From<CapabilityContext<RenderOperation, E>> for Render<E>
where
    E: 'static,
{
    fn from(context: CapabilityContext<RenderOperation, E>) -> Self {
        Self::new(context)
    }
}

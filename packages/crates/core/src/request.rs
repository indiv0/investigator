use crate::prelude::*;

use crate::context;
use crate::resolve;

/// Request represents an effect request from the core to the shell.
///
/// The `operation` is the input needed to process the effect, and will be one
/// of the capabilities' [`Operation`] types.
///
/// The request can be resolved by passing it to `Core::resolve` along with the
/// corresponding result of type `Operation::Output`.
#[derive(Debug)]
pub struct Request<O>
where
    O: context::Operation,
{
    pub operation: O,
    pub(crate) resolve: resolve::Resolve<O::Output>,
}

impl<O> Request<O>
where
    O: context::Operation,
{
    pub(crate) fn resolves_never(operation: O) -> Self {
        Self {
            operation,
            resolve: resolve::Resolve::Never,
        }
    }

    pub(crate) fn resolves_once<F>(operation: O, resolve: F) -> Self
    where
        F: FnOnce(O::Output) + Send + 'static,
    {
        Self {
            operation,
            resolve: resolve::Resolve::Once(Box::new(resolve)),
        }
    }

    pub(crate) fn resolves_many_times<F>(operation: O, resolve: F) -> Self
    where
        F: Fn(O::Output) -> Result<(), ()> + Send + 'static,
    {
        Self {
            operation,
            resolve: resolve::Resolve::Many(Box::new(resolve)),
        }
    }

    pub(crate) fn resolve(&mut self, output: O::Output) -> Result<(), resolve::ResolveError> {
        self.resolve.resolve(output)
    }
}

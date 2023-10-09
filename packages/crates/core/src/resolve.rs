use crate::prelude::*;

type ResolveOnce<O> = Box<dyn FnOnce(O) + Send>;
type ResolveMany<O> = Box<dyn Fn(O) -> Result<(), ()> + Send>;

/// Resolve is a callback used to resolve an effect request and continue
/// one of the capability Tasks running on the executor.
pub(crate) enum Resolve<O> {
    Never,
    Once(ResolveOnce<O>),
    Many(ResolveMany<O>),
}

impl<O> Resolve<O> {
    pub(crate) fn resolve(&mut self, output: O) -> Result<(), ResolveError> {
        match self {
            Resolve::Never => Err(ResolveError::Never),
            Resolve::Many(f) => f(output).map_err(|_| ResolveError::FinishedMany),
            Resolve::Once(_) => {
                // The resolve has been used, turn it into a Never
                if let Resolve::Once(f) = std::mem::replace(self, Resolve::Never) {
                    f(output);
                }

                Ok(())
            }
        }
    }
}

impl<O> Debug for Resolve<O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Resolve::Never => write!(f, "Resolve::Never"),
            Resolve::Once(_) => write!(f, "Resolve::Once"),
            Resolve::Many(_) => write!(f, "Resolve::Many"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ResolveError {
    Never,
    FinishedMany,
}

impl Display for ResolveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ResolveError::Never => write!(f, "Attempted to resolve a request that is not expected to be resolved."),
            ResolveError::FinishedMany => write!(f, "Attempted to resolve a request that has concluded."),
        }
    }
}

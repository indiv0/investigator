use crate::prelude::*;

use std::error;
use std::io;



// ===================
// === IoResultExt ===
// ===================

pub trait IoResultExt<T> {
    fn with_err_path<P>(self, path: P) -> Self
    where
        P: Into<PathBuf>;
}


// === Trait `impl`s ===

impl<T> IoResultExt<T> for io::Result<T> {
    fn with_err_path<P>(self, path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        self.map_err(|error| {
            let path = path.into();
            let kind = error.kind();
            let error = PathError { path, error };
            io::Error::new(kind, error)
        })
    }
}



// =================
// === PathError ===
// =================

#[derive(Debug)]
struct PathError {
    path: PathBuf,
    error: io::Error,
}


// === Trait `impl`s ===

impl Display for PathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} at path \"{:?}\".", self.error, self.path)
    }
}

impl error::Error for PathError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.error.source()
    }
}

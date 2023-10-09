use crate::prelude::*;

use std::fs;
use std::io;



// ===============
// === PathExt ===
// ===============

pub trait PathExt {
    fn create_dir_all(&self) -> io::Result<()>;
}


// === Trait `impl`s ===

impl<P> PathExt for P
where
    P: AsRef<Path>,
{
    fn create_dir_all(&self) -> io::Result<()> {
        let path = self.as_ref();
        fs::create_dir_all(path).with_err_path(path)
    }
}

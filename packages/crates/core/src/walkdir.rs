use crate::prelude::*;

use crate::fs;
use std::io;



// ============
// === find ===
// ============

/// Recursively iterates over the directory starting at the file path `root`, returning a list of
/// inodes.
pub fn find(fs: Fs, root: impl Into<PathBuf>) -> impl Iterator<Item = io::Result<DirEntry>> {
    let walkdir = WalkDir::new(fs, root.into());
    let walkdir = walkdir.into_iter();
    walkdir.map(|e| e)
}



// ===============
// === WalkDir ===
// ===============

/// Builder for a recursive directory iterator.
#[derive(Clone, Debug)]
pub struct WalkDir {
    fs: Fs,
    root: PathBuf,
}


// === Main `impl` ===

impl WalkDir {
    /// Creates a builder for a recursive directory iterator starting at the file path `root`.
    ///
    /// [`WalkDir`]: crate::walkdir::WalkDir
    pub fn new(fs: Fs, root: PathBuf) -> Self {
        Self { fs, root }
    }
}


// === Trait `impl`s ===

impl IntoIterator for WalkDir {
    type Item = io::Result<DirEntry>;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { fs: self.fs, start: Some(self.root), stack_list: vec![] }
    }
}



// ================
// === IntoIter ===
// ================

/// Iterator for recursively descending into a directory.
#[derive(Debug)]
pub struct IntoIter {
    /// Handle to the [`Fs`] to use for the search.
    ///
    /// [`Fs`]: crate::fs::Fs
    fs: Fs,
    /// Path to start the search from.
    ///
    /// This is only [`Some`] at the beginning. After the first iteration, it is always [`None`].
    ///
    /// [`Some`]: std::option::Option::Some
    /// [`None`]: std::option::Option::None
    start: Option<PathBuf>,
    // FIXME [NP]: update docs
    /// A stack of open (up to max fd) or closed handles to directories.
    /// An open handle is a plain [`fs::ReadDir`] while a closed handle is
    /// a `Vec<fs::DirEntry>` corresponding to the as-of-yet consumed entries.
    ///
    /// [`fs::ReadDir`]: std::fs::ReadDir
    stack_list: Vec<DirList>,
}


// === Internal `impl` ===

impl IntoIter {
    fn handle_entry(&mut self, entry: DirEntry) -> Option<io::Result<DirEntry>> {
        if entry.file_type().is_symlink() {
            unimplemented!("Symlinks are not supported.");
        }

        let is_normal_dir = !entry.file_type().is_symlink() && entry.is_dir();
        if is_normal_dir {
            itry!(self.push(&entry));
        }

        Some(Ok(entry))
    }

    fn push(&mut self, entry: &DirEntry) -> io::Result<()> {
        let rd = crate::fs::read_dir(&self.fs, entry.path());
        let rd = rd.map_err(|e| {
            // FIXME [NP]: Handle the error here.
            panic!("Error: {e:?}.");
            //Some(io::Error::from_path(self.depth, dent.path().to_path_buf(), err))
        });
        // FIXME [NP]: Remove?
        let rd = rd.unwrap();
        let rd = Ok(rd);
        assert!(rd.is_ok(), "Read dir: {rd:?}.");

        let list = DirList { fs: self.fs.clone(), it: rd };
        self.stack_list.push(list);

        Ok(())
            //// Make room for another open file descriptor if we've hit the max.
            //let free = self.stack_list.len().checked_sub(self.oldest_opened).unwrap();
            //if free == self.opts.max_open {
            //    self.stack_list[self.oldest_opened].close();
            //}
            //// Open a handle to reading the directory's entries.
            //let rd = fs::read_dir(dent.path()).map_err(|err| {
            //    Some(Error::from_path(self.depth, dent.path().to_path_buf(), err))
            //});
            //let mut list = DirList::Opened { depth: self.depth, it: rd };
            //if let Some(ref mut cmp) = self.opts.sorter {
            //    let mut entries: Vec<_> = list.collect();
            //    entries.sort_by(|a, b| match (a, b) {
            //        (&Ok(ref a), &Ok(ref b)) => cmp(a, b),
            //        (&Err(_), &Err(_)) => Ordering::Equal,
            //        (&Ok(_), &Err(_)) => Ordering::Greater,
            //        (&Err(_), &Ok(_)) => Ordering::Less,
            //    });
            //    list = DirList::Closed(entries.into_iter());
            //}
            //if self.opts.follow_links {
            //    let ancestor = Ancestor::new(&dent)
            //        .map_err(|err| Error::from_io(self.depth, err))?;
            //    self.stack_path.push(ancestor);
            //}
            //// We push this after stack_path since creating the Ancestor can fail.
            //// If it fails, then we return the error and won't descend.
            //self.stack_list.push(list);
            //// If we had to close out a previous directory stream, then we need to
            //// increment our index the oldest still-open stream. We do this only
            //// after adding to our stack, in order to ensure that the oldest_opened
            //// index remains valid. The worst that can happen is that an already
            //// closed stream will be closed again, which is a no-op.
            ////
            //// We could move the close of the stream above into this if-body, but
            //// then we would have more than the maximum number of file descriptors
            //// open at a particular point in time.
            //if free == self.opts.max_open {
            //    // Unwrap is safe here because self.oldest_opened is guaranteed to
            //    // never be greater than `self.stack_list.len()`, which implies
            //    // that the subtraction won't underflow and that adding 1 will
            //    // never overflow.
            //    self.oldest_opened = self.oldest_opened.checked_add(1).unwrap();
            //}
            //Ok(())
    }

    fn pop(&mut self) {
        self.stack_list.pop().expect("Stack should be non-empty.");
    }
}


// === Trait `impl`s ===

impl Iterator for IntoIter {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(start) = self.start.take() {
            let entry = itry!(DirEntry::from_path(&self.fs, start));
            if let Some(result) = self.handle_entry(entry) {
                assert!(result.is_ok(), "Error: {result:?}.");
                return Some(result)
            }
        }

        while !self.stack_list.is_empty() {
            // Unwrap is safe here because we've verified above that `self.stack_list` is not empty.
            let next = self.stack_list.last_mut();
            let next = next.expect("Stack should be non-empty.");
            let next = next.next();
            match next {
                None => self.pop(),
                // FIXME [NP]: replace
                //Some(Err(e)) => return Some(Err(e)),
                Some(Err(e)) => panic!("Error: {e:?}."),
                Some(Ok(entry)) => {
                    if let Some(result) = self.handle_entry(entry) {
                        assert!(result.is_ok(), "Error: {result:?}.");
                        return Some(result)
                    }
                }
            }
        }

        None
    }
}



// ===============
// === DirList ===
// ===============

// FIXME [NP]: Update docs.
/// A sequence of unconsumed directory entries.
///
/// This represents the opened or closed state of a directory handle. When open, future entries are
/// read by iterating over the raw [`fs::ReadDir`]. When closed, all future entries are read into
/// memory. Iteration then proceeds over a [`Vec<fs::DirEntry>`].
///
/// [`fs::ReadDir`]: std::fs::ReadDir
/// [`Vec<fs::DirEntry>`]: std::vec::Vec
#[derive(Debug)]
struct DirList {
    /// Handle to the [`Fs`] to use for the search.
    ///
    /// [`Fs`]: crate::fs::Fs
    fs: Fs,
    // FIXME [NP]: Update docs.
    /// An opened handle.
    ///
    /// This includes the depth of the handle itself.
    ///
    /// If there was an error with the initial [`fs::read_dir`] call, then it is stored here. We use
    /// an [`Option`] to make yielding the error exactly once simpler.
    ///
    /// [`fs::read_dir`]: std::fs::read_dir
    /// [`Option`]: std::option::Option
    it: Result<fs::ReadDir, Option<io::Error>>,
}


// === Trait `impl`s ===

impl Iterator for DirList {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        match self.it {
            Ok(ref mut rd) => {
                let next = rd.next();
                next.map(|r| match r {
                    Ok(r) => Ok(r),
                    // FIXME [NP]: Replace
                    Err(e) => panic!("Error: {e:?}."),
                    //Err(e) => Err(e),
                })
            },
            // FIXME [NP]: Replace
            Err(ref mut e) => panic!("Error: {e:?}."),
            //Err(ref mut e) => e.take().map(Err),
        }
    }
}


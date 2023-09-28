use crate::prelude::*;

use core::iter;
use crate::path;
use std::fs;
use std::io;



// =================
// === Constants ===
// =================

/// Prefix for absolute directories on UNIX-like operating systems.
const ROOT: &str = "/";
/// A single occurrence of the path separator on UNIX-like operating systems.
const SLASH: &str = "/";
/// Constant defining a double occurrence of the path separator.
const DOUBLE_SLASH: &str = "//";



// ==========
// === Fs ===
// ==========

/// An abstraction over the file system that enables in-memory mocking.
#[derive(Clone, Debug, Default)]
pub struct Fs {
    inner: Inner,
}


// === Main `impl` ===

impl Fs {
    /// Creates an empty, new in-memory file system.
    pub fn mock() -> Self {
        Self { inner: Inner::Mock(Mock::Map(Default::default())) }
    }

    /// Creates a new file system that is a namespace of the actual file system.
    pub fn namespaced(real_path: impl Into<PathBuf>, namespace: impl Into<String>) -> Self {
        let real_path = real_path.into();
        let namespace = namespace.into();
        Self { inner: Inner::Mock(Mock::Namespaced { real_path, namespace }) }
    }

    /// Creates a new in-memory file system from the given files.
    pub fn from_map(input: BTreeMap<OsString, impl Into<Vec<u8>>>) -> Self {
        let input = input.into_iter();
        let input = input.map(|(key, value)| (key.into(), Entry::File(value.into())));
        // FIXME [NP]: Create the parent directory for each file in the database.
        let input = input.collect();
        let input = RefCell::new(Map(input));
        let input = Rc::new(input);
        Self { inner: Inner::Mock(Mock::Map(input)) }
    }

    /// Creates a new directory at the given path.
    pub fn create_dir(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        let path = path.as_ref();
        println!("Fs::create_dir: {path:?}.");
        match &mut self.inner {
            Inner::Real => unimplemented!(),
            Inner::Mock(ref mut mock) => match mock {
                // TODO [NP]: Deny too-long file names.
                Mock::Map(ref mut fs) => {
                    let path = path::AbsolutePathBuf::from(path);
                    let mut fs = fs.borrow_mut();

                    // Treat the root path as a special case, because it is always assumed to exist.
                    if path::is_root(&*path) {
                        println!("Directory exists: {path:?}.");
                        return Err(io::Error::from(io::ErrorKind::AlreadyExists));
                    }

                    check_ancestors_are_dirs(&fs, path.clone())?;
                    check_parent_exists(&fs, &path)?;

                    // Check that the directory does not already exist.
                    if fs.exists(&path) {
                        println!("Directory exists: {path:?}.");
                        return Err(io::Error::from(io::ErrorKind::AlreadyExists));
                    }

                    // Insert the directory into the file system.
                    let path = OsString::from(&*path);
                    println!("Creating directory: {path:?}.");
                    let previous = fs.insert(path, Entry::Directory);
                    assert!(previous.is_none(), "Duplicate directory.");
                    Ok(())
                },
                Mock::Namespaced { real_path, namespace } => {
                    let path = namespace_path(path, real_path, namespace)?;
                    fs::create_dir(path)
                },
            },
        }
    }

    /// Writes a slice as the entire contents of a file.
    ///
    /// This function will create a file if it does not exist, and will entirely replace its
    /// contents if it does.
    ///
    /// Depending on the platform, this function may fail if the full directory path does not exist.
    ///
    /// This is a convenience function for using [`File::create`] and [`write_all`] with fewer
    /// imports.
    ///
    /// [`File::create`]: std::fs::File::create
    /// [`write_all`]: std::io::Write::write_all
    pub fn write(&mut self, path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> io::Result<()> {
        let path = path.as_ref();
        println!("Fs::write: {path:?}.");
        match &mut self.inner {
            Inner::Real => unimplemented!(),
            Inner::Mock(ref mut mock) => match mock {
                // TODO [NP]: Deny too-long file names.
                Mock::Map(ref mut fs) => {
                    let path = path::AbsolutePathBuf::from(path);
                    let mut fs = fs.borrow_mut();

                    // The root is a directory and can't be written to, so return an error.
                    if path::is_root(&*path) {
                        println!("Root is directory: {path:?}.");
                        return Err(io::Error::from(io::ErrorKind::IsADirectory));
                    }

                    check_ancestors_are_dirs(&fs, path.clone())?;
                    check_parent_exists(&fs, &path)?;

                    // If a directory with the given name exists, return an error.
                    if fs.is_dir(&*path) {
                        println!("Directory exists: {path:?}.");
                        return Err(io::Error::from(io::ErrorKind::IsADirectory));
                    }

                    // Insert the file into the file system.
                    let path = OsString::from(&*path);
                    let contents = contents.as_ref();
                    let contents = contents.to_vec();
                    let contents = Entry::File(contents);
                    println!("Creating file: {path:?}.");
                    let _previous = fs.insert(path, contents);
                    Ok(())

                },
                Mock::Namespaced { real_path, namespace } => {
                    let path = namespace_path(path, real_path, namespace)?;
                    fs::write(path, contents)
                },
            }
        }
    }

    /// Returns the [`FileType`] for the file that this entry points to.
    pub fn file_type(&self, path: impl AsRef<Path>) -> io::Result<FileType> {
        let path = path.as_ref();
        println!("Fs::file_type: {path:?}.");
        match &self.inner {
            Inner::Real => unimplemented!(),
            Inner::Mock(ref mock) => match mock {
                Mock::Map(ref fs) => {
                    let path = path::AbsolutePathBuf::from(path);
                    let fs = fs.borrow();

                    // The root is a special case, so return the appropriately mocked value.
                    if path::is_root(&*path) {
                        return Ok(FileType {})
                    }

                    check_ancestors_are_dirs(&fs, path.clone())?;

                    let entry = fs.get(&*path);
                    entry.map(|e| e.file_type()).ok_or_else(|| {
                        println!("Path does not exist: {path:?}.");
                        io::Error::from(io::ErrorKind::NotFound)
                    })
                },
                Mock::Namespaced { real_path, namespace } => {
                    let path = namespace_path(path, real_path, namespace)?;
                    let metadata = fs::metadata(path)?;
                    let file_type = metadata.file_type();
                    let file_type = FileType::from_file_type(file_type);
                    Ok(file_type)
                },
            },
        }
    }
}



// ================
// === DirEntry ===
// ================

/// A directory entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirEntry {
    path: PathBuf,
    file_type: crate::fs::FileType,
}


// === Main `impl` ===

impl DirEntry {
    // FIXME [NP]: This can be private?
    /// Creates a new [`DirEntry`] from the given [`PathBuf`].
    ///
    /// [`DirEntry`]: crate::walkdir::DirEntry
    /// [`PathBuf`]: std::path::PathBuf
    pub(crate) fn from_path(fs: &Fs, path: PathBuf) -> io::Result<Self> {
        println!("From path: {path:?}.");
        // FIXME [NP]: Assert that the path is a directory?
        let file_type = fs.file_type(&path);
        // FIXME [NP]: Don't unwrap here.
        let file_type = file_type.expect("File type error.");
        Ok(Self { path, file_type })
    }

    // FIXME [NP]: This can be private?
    /// Creates a new [`DirEntry`] from the given [`std::fs::DirEntry`].
    ///
    /// [`DirEntry`]: crate::walkdir::DirEntry
    /// [`std::fs::DirEntry`]: std::fs::DirEntry
    pub(crate) fn from_entry(fs: &Fs, entry: &std::fs::DirEntry) -> io::Result<Self> {
        println!("From entry: {entry:?}.");
        let path = entry.path();
        let file_type = fs.file_type(&path);
        // FIXME [NP]: Don't unwrap here.
        let file_type = file_type.expect("File type error.");
        Ok(Self { path, file_type })
    }

    /// Returns the [`Path`] of this [`DirEntry`].
    ///
    /// [`Path`]: std::path::Path
    /// [`DirEntry`]: crate::walkdir::DirEntry
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    /// Returns the [`FileType`] for the file that this entry points to.
    ///
    /// [`FileType`]: crate::fs::FileType
    pub fn file_type(&self) -> crate::fs::FileType {
        self.file_type
    }

    /// Returns `true` if and only if this [`DirEntry`] points to a directory.
    ///
    /// [`DirEntry`]: crate::walkdir::DirEntry
    pub(crate) fn is_dir(&self) -> bool {
        self.file_type.is_dir()
    }
}



/// Iterator over the entries in a directory.
///
/// This iterator is returned from the [`read_dir`] function of this module and will yield instances
/// of <code>[io::Result]<[DirEntry]></code>. Through a [`DirEntry`] information like the entry's
/// path and possibly other metadata can be learned.
///
/// The order in which this iterator returns entries is platform and filesystem dependent.
///
/// # Errors
///
/// This [`io::Result`] will be an [`Err`] if there's some sort of intermittent IO error during
/// iteration.
///
/// [`read_dir`]: crate::fs::read_dir
/// [`io::Result`]: std::io::Result
/// [`DirEntry`]: crate::fs::DirEntry
/// [`Err`]: std::result::Result::Err
#[derive(Debug)]
pub enum ReadDir {
    Namespaced { fs: Fs, read_dir: fs::ReadDir },
    Map { fs: Fs },
}


// === Trait `impl` ===

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Map { fs } => unimplemented!(),
            Self::Namespaced { fs, read_dir } => {
                let entry = itry!(read_dir.next()?);
                let entry = DirEntry::from_entry(fs, &entry);
                Some(entry)
            },
        }
    }
}



/// Returns an iterator over the entries within a directory.
///
/// The iterator will yield instances of <code>[io::Result]<[DirEntry]></code>. New Errors may be
/// encountered after an iterator is initially constructed. Entries for the current and parent
/// directories (typically `.` and `..`) are skipped.
///
/// # Platform-specific behaviour
///
/// This function currently corresponds to the `opendir` function on Unix and the `FindFirstFile`
/// function on Windows. Advancing the iterator currently corresponds to `readdir` on Unix and
/// `FindNextFile` on Windows. Note that, this [may change in the future][changes].
///
/// [`io::Result`]: std::io::Result
/// [`DirEntry`]: std::fs::DirEntry
/// [changes]: std::io#platform-specific-behavior
///
/// The order in which this iterator returns entries is platform and filesystem dependent.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just these
/// cases:
///
/// * The provided `path` doesn't exist.
/// * The process lacks permissions to view the contents.
/// * The `path` points at a non-directory file.
pub fn read_dir(fs: &Fs, path: impl AsRef<Path>) -> io::Result<ReadDir> {
    let path = path.as_ref();
    println!("fs::read_dir: {path:?}.");
    match &fs.inner {
        Inner::Real => unimplemented!(),
        Inner::Mock(ref mock) => match mock {
            Mock::Map(ref _fs) => Ok(ReadDir::Map { fs: fs.clone() }),
            Mock::Namespaced { real_path, namespace } => {
                let path = namespace_path(path, real_path, namespace)?;
                let read_dir = fs::read_dir(path)?;
                Ok(ReadDir::Namespaced { fs: fs.clone(), read_dir })
            },
        },
    }
}

/// Returns an error if any existing ancestors of the [`Path`] is not a directory.
fn check_ancestors_are_dirs(fs: &Map, path: path::AbsolutePathBuf) -> io::Result<()> {
    for ancestor in fs.ancestors(path) {
        if fs.exists(&ancestor) && !fs.is_dir(&*ancestor) {
            println!("Ancestor is not a directory: {ancestor:?}.");
            return Err(io::Error::from(io::ErrorKind::NotADirectory));
        }
    }
    Ok(())
}

/// Returns an error if the parent of the [`Path`] does not exist.
fn check_parent_exists(fs: &Map, path: impl AsRef<path::AbsolutePathBuf>) -> io::Result<()> {
    let path = path.as_ref();
    let parent = fs.parent(path).expect("Not a child.");
    if !fs.exists(&parent) {
        println!("Path does not exist: {parent:?}.");
        return Err(io::Error::from(io::ErrorKind::NotFound));
    }
    Ok(())
}

/// Returns a `Path` with the `namespace` prefix stripped, and the `real_path` prefix added.
fn namespace_path<'a>(path: &Path, real_path: &PathBuf, namespace: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.strip_prefix(namespace);
    let path = path.map_err(|_| io::Error::from(io::ErrorKind::NotFound))?;
    // `PathBuf::join` behaves like `PathBuf::push`, which states: "If `path` is
    // absolute, it replaces the current path."
    //
    // This is not correct behaviour for a namespaced file system, so absolute paths
    // must be made relative, first.
    // TODO [NP]: Make this cross-platform, since not all platforms use `/` as the
    // path separator.
    let path = path.strip_prefix(ROOT).unwrap_or(&path);
    let path = real_path.join(path);
    assert!(path.starts_with(&real_path));
    Ok(path)
}



// ================
// === FileType ===
// ================

/// A structure representing a type of file with accessors for each file type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FileType;


// === Main `impl` ===

impl FileType {
    /// Returns `true` if this [`FileType`] represents a symlink.
    ///
    /// [`FileType`]: crate::fs::FileType
    pub(crate) fn is_symlink(&self) -> bool {
        println!("FileType::is_symlink: {self:?}.");
        false
    }

    /// Returns `true` if this [`FileType`] represents a directory.
    ///
    /// [`FileType`]: crate::fs::FileType
    pub(crate) fn is_dir(&self) -> bool {
        println!("FileType::is_dir: {self:?}.");
        false
    }

    /// Returns `true` if this [`FileType`] represents a file.
    ///
    /// [`FileType`]: crate::fs::FileType
    pub fn is_file(&self) -> bool {
        println!("FileType::is_file: {self:?}.");
        false
    }
}


// === Internal `impl` ===

impl FileType {
    /// Creates a new [`FileType`] from the given [`std::fs::FileType`].
    ///
    /// [`FileType`]: crate::fs::FileType
    /// [`std::fs::FileType`]: std::fs::FileType
    fn from_file_type(file_type: fs::FileType) -> Self {
        Self {}
    }
}



// =============
// === Entry ===
// =============

/// Possible types of entries in an in-memory file system.
#[derive(Clone, Debug)]
enum Entry {
    Directory,
    File(Vec<u8>),
}


// === Internal `impl` ===

impl Entry {
    fn file_type(&self) -> FileType {
        match self {
            Entry::Directory => FileType {},
            Entry::File(_) => FileType {},
        }
    }
}



// =============
// === Inner ===
// =============

#[derive(Clone, Debug, Default)]
enum Inner {
    #[default]
    Real,
    Mock(Mock),
}



// ============
// === Mock ===
// ============

#[derive(Clone, Debug)]
enum Mock {
    Map(Rc<RefCell<Map>>),
    Namespaced {
        /// Path under which all files/directories are namedspaced.
        ///
        /// For example, if this this is set to `/example`, then `create_dir("/foo")` will create a
        /// directory called `/example/foo`.
        real_path: PathBuf,
        /// Path prefix to strip from all paths.
        ///
        /// For example, if this is set to `/example`, then `create_dir("/example/foo")` will create
        /// a directory called `/foo`.
        namespace: String,
    }
}



// ===========
// === Map ===
// ===========

/// An in-memory file system backed by a [`BTreeMap`].
///
/// [`BTreeMap`]: std::collections::BTreeMap
#[derive(Clone, Debug, Default)]
struct Map(BTreeMap<OsString, Entry>);


// === Internal `impl` ===

impl Map {
    /// Returns the [`Path`] without its final component, if there is one.
    ///
    /// Returns [`None`] if the [`Path`] terminates in a root or prefix.
    fn parent(&self, path: &Path) -> Option<path::AbsolutePathBuf> {
        println!("Map::parent: {self:?} {path:?}.");
        let parent = path.parent();
        parent.map(Into::into)
    }

    /// Produces an iterator over [`Path`] and its ancestors.
    ///
    /// The iterator will yield the [`Path`] that is returned if the [`parent`] method is used zero
    /// or more times. That means, the iterator will yield `&self`, `&self.parent().unwrap()`,
    /// `&self.parent().unwrap().parent().unwrap()`, and so on. If the [`parent`] method returns
    /// [`None`], the iterator will do likewise. The iterator will always yield at least one value,
    /// namely `&self`.
    ///
    /// [`Path`]: std::path::Path
    /// [`parent`]: crate::fs::Map::parent
    /// [`None`]: std::option::Option::None
    fn ancestors(&self, path: path::AbsolutePathBuf) -> impl Iterator<Item = path::AbsolutePathBuf> + '_ {
        println!("Map::ancestors: {self:?} {path:?}.");
        let mut path = Some(path);
        iter::from_fn(move || {
            match &path {
                Some(p) => {
                    let parent = self.parent(&*p)?;
                    path = Some(parent.clone());
                    Some(parent)
                },
                None => None,
            }
        })
    }

    /// Returns `true` if the [`Path`] exists on disk.
    fn exists(&self, path: impl AsRef<path::AbsolutePathBuf>) -> bool {
        let path = path.as_ref();
        println!("Map::exists: {self:?} {path:?}.");

        // Treat the root path as a special case, because it is always assumed to exist.
        if path::is_root(&**path) {
            return true;
        }

        self.contains_key(&**path)
    }

    /// Returns `true` if the [`Path`] exists on disk and is pointing at a [`Directory`].
    ///
    /// [`Directory`]: crate::fs::Entry::Directory
    fn is_dir(&self, path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();
        println!("Map::is_dir: {self:?} {path:?}.");

        // Treat the root path as a special case, because it is always assumed to exist.
        if path::is_root(&path) {
            return true;
        }

        match self.get(path) {
            Some(Entry::Directory) => true,
            Some(Entry::File(_)) => false,
            None => false,
        }
    }

    /// Returns the first key in the given [`BTreeMap`] that matches the given [`Path`], when
    /// compared case-insensitively.
    // TODO [NP]: Support case-sensitivity?
    // TODO [NP]: Only checks ASCII, add Unicode support.
    fn key(&self, path: impl AsRef<Path>) -> Option<&OsStr> {
        let path = path.as_ref();
        println!("Map::key: {self:?} {path:?}.");
        let path = path.as_os_str();
        self.entry(path).map(|(k, _)| k)
    }

    /// Returns `true` if the [`Path`] exists on disk, when compared case-insensitively.
    // TODO [NP]: Support case-sensitivity?
    // TODO [NP]: Only checks ASCII, add Unicode support.
    fn contains_key(&self, path: impl AsRef<Path>) -> bool {
        self.key(path).is_some()
    }

    /// Returns the value of the first entry in the given [`BTreeMap`] whose key matches the given
    /// [`Path`], when compared case-insensitively.
    // TODO [NP]: Support case-sensitivity?
    // TODO [NP]: Only checks ASCII, add Unicode support.
    fn get(&self, path: impl AsRef<Path>) -> Option<&Entry> {
        let path = path.as_ref();
        println!("Map::get: {self:?} {path:?}.");
        self.entry(path).map(|(_, v)| v)
    }

    /// Returns the first entry in the given [`BTreeMap`] whose key matches the given [`Path`], when
    /// compared case-insensitively.
    // TODO [NP]: Support case-sensitivity?
    // TODO [NP]: Only checks ASCII, add Unicode support.
    fn entry(&self, path: impl AsRef<Path>) -> Option<(&OsStr, &Entry)> {
        let Self(inner) = self;
        let path = path.as_ref();
        println!("Map::entry: {self:?} {path:?}.");
        inner.iter().find(|(k, _)| k.eq_ignore_ascii_case(path)).map(|(k, v)| (k.as_os_str(), v))
    }
}


// === Trait `impl`s ===

impl Deref for Map {
    type Target = BTreeMap<OsString, Entry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

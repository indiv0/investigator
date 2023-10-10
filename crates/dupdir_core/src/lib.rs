use crate::prelude::*;

use core::fmt;
use dupdir_hash::Hasher as _;
use indicatif::ProgressIterator as _;
use indicatif::ParallelProgressIterator as _;
use rayon::iter::IntoParallelIterator as _;
use rayon::iter::IntoParallelRefIterator as _;
use rayon::iter::ParallelIterator as _;
use std::fs;
use std::str;



// =================
// === Constants ===
// =================

pub const STATE_JSON: &str = "state.json";
const UNIQUE_SEPARATOR: &str = ";";



// ===============
// === Prelude ===
// ===============

pub mod prelude {
    pub(crate) use serde::Deserialize;
    pub(crate) use serde::Serialize;
    pub(crate) use core::fmt::Debug;
    pub(crate) use core::fmt::Formatter;
    pub(crate) use core::marker::PhantomData;
    pub(crate) use rayon::prelude::ParallelIterator;
    pub(crate) use std::collections::BTreeMap;
    pub(crate) use std::collections::BTreeSet;
    pub(crate) use std::collections::HashMap;
    pub(crate) use std::path::Path;
    pub(crate) use std::path::PathBuf;
    pub(crate) use walkdir::WalkDir;
    pub use crate::assert_path_rules;
    pub use crate::path_to_str;
    pub use crate::path_to_string;
    pub use crate::FinderIter;
    pub use crate::State;
    pub use crate::WalkDirFinder;
    pub use crate::STATE_JSON;
}



// =============
// === State ===
// =============

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[must_use]
pub struct State {
    files: Vec<PathBuf>,
    hashes: BTreeMap<PathBuf, String>,
}

// === Main `impl` ===

impl State {
    pub fn save(&self) {
        let json = serde_json::to_string_pretty(&self).expect("Serialize");
        let path = Path::new(STATE_JSON);
        fs::write(path, json).expect("Write");
    }

    pub fn load(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        if path.exists() {
            let json = fs::read_to_string(path).expect("Read");
            serde_json::from_str(&json).expect("Deserialize")
        } else {
            Self::default()
        }
    }
}

#[inline]
pub fn assert_path_rules(p: impl AsRef<Path>) {
    let p = path_to_str(&p);
    assert!(!p.contains('\r'), "Unsupported character in path");
    assert!(!p.is_empty(), "Empty path");
    assert_eq!(p, p.trim(), "Extra whitespace in path");
}

#[inline]
pub fn path_to_str(p: &impl AsRef<Path>) -> &str {
    let p = p.as_ref();
    let p = p.to_str();
    p.expect("Path should be valid UTF-8")
}

#[inline]
pub fn path_to_string(p: impl AsRef<Path>) -> String {
    let p = path_to_str(&p);
    p.to_string()
}



// ===============
// === run_all ===
// ===============

pub fn run_all(
    state: &mut crate::State,
    search_path: &str,
) -> Vec<String> {
    eprintln!("Searching for files...");
    let files = WalkDirFinder::new(search_path);
    let files = files.into_iter();
    let files = files.collect();
    state.files = files;
    eprintln!("Hashing files...");
    hash(state);
    eprintln!("Saving hashes...");
    state.save();
    eprintln!("Computing directory hashes...");
    let dir_hashes = dir_hashes(state);
    eprintln!("Finding duplicate directories...");
    dup_dirs(&dir_hashes)
}




// =====================
// === WalkdirFinder ===
// =====================

#[must_use]
pub struct WalkDirFinder<'a> {
    entries: WalkDir,
    marker: PhantomData<&'a ()>,
}


// === Main `impl` ===

impl WalkDirFinder<'_> {
    pub fn new(path: &str) -> Self {
        let entries = WalkDir::new(path);
        Self { entries, marker: PhantomData }
    }
}


// === Trait `impl`s ===

impl<'a> IntoIterator for WalkDirFinder<'a> {
    type Item = PathBuf;
    type IntoIter = FinderIter<'a, PathBuf>;

    fn into_iter(self) -> Self::IntoIter {
        let entries = self.entries.into_iter();
        let paths = entries.filter_map(|e| {
            let e = e.expect("Failed to read entry");
            let file_type = e.file_type();
            if !file_type.is_file() {
                return None;
            }
            let path = e.into_path();
            Some(path)
        });
        let paths = Box::new(paths);
        Self::IntoIter { paths }
    }
}



// ==================
// === FinderIter ===
// ==================

#[must_use]
pub struct FinderIter<'a, P> {
    pub paths: Box<dyn Iterator<Item = P> + 'a>,
}

// === Trait `impl`s ===

impl<'a, P> Iterator for FinderIter<'a, P>
where
    P: AsRef<Path> + 'a,
{
    type Item = P;

    fn next(&mut self) -> Option<Self::Item> {
        match self.paths.next() {
            Some(path) => {
                assert_path_rules(&path);
                Some(path)
            },
            None => None,
        }
    }
}

impl<P> Debug for FinderIter<'_, P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Finder")
            .field("lines", &"Box<dyn Iterator<Item = P>>")
            .finish()
    }
}



// ============
// === hash ===
// ============

fn hash(state: &mut State) {
    let paths = state.files.clone();
    let paths = paths.into_par_iter();
    let paths = paths.progress();
    let hashes = paths.map(|p| {
        match state.hashes.get(&p) {
            Some(hash) => {
                let hash = hash.as_str();
                let hash = hash.to_string();
                (p, hash)
            },
            None => {
                let path = path_to_str(&p);
                let hash = hash_path(path);
                (p, hash)
            },
        }
    });
    state.hashes = hashes.collect();
}

fn hash_path(path: &str) -> String {
    let mut file = fs::File::open(path).unwrap_or_else(|_| panic!("Failed to open file: {path:?}"));
    let mut hasher = dupdir_hash::T1ha2::default();
    dupdir_hash::copy_wide(&mut file, &mut hasher).expect("Failed to hash file");
    let hash = hasher.finish().to_vec();
    hex::encode(hash)
}



// ==================
// === dir_hashes ===
// ==================

pub fn dir_hashes(
    state: &mut crate::State,
) -> Vec<(String, String)> {
    let dir_hashes = dir_hashes_walk_dir_inner(state);
    let dir_hashes = dir_hashes.map(|(hash, directory)| {
        let hash = hex::encode(hash);
        (hash, directory)
    });
    dir_hashes.collect()
}

fn dir_hashes_walk_dir_inner(
    state: &mut crate::State,
) -> impl ParallelIterator<Item = ([u8; 8], String)> + '_ {
    eprintln!("Mapping file hashes to their ancestors...");
    let entries = state.files.iter();
    let entries = entries.progress();
    let mut files_in_dir = BTreeMap::<_, BTreeSet<&str>>::new();
    entries.for_each(|path| {
        let dir = path.parent().expect("Parent");
        let dir = dir.to_path_buf();
        let h = state.hashes.get(path).expect("File hash");
        for a in dir.ancestors() {
            let a = path_to_string(a);
            let hashes = files_in_dir.entry(a);
            // Note that we store the files in a `BTreeSet` rather than incrementally hashing
            // because the order in which files appear in the directories (e.g., due to renaming)
            // shouldn't affect the hash.
            let hashes = hashes.or_insert_with(BTreeSet::new);
            hashes.insert(h);
        }
    });

    eprintln!("Finalizing directory hashes...");
    let dir_hashes = files_in_dir.into_par_iter();
    dir_hashes.map(|(d, hashes)| {
        let mut hasher = dupdir_hash::T1ha2::default();
        let hashes = hashes.into_iter();
        hashes.for_each(|h| {
            // FIXME [NP]: Is this correct? It'll register directories w/ different amounts of
            // copies of the same file as identical.
            dupdir_hash::copy_wide(&mut &h.as_bytes()[..], &mut hasher).unwrap();
        });
        let hash = hasher.finish();
        (hash, d)
    })
}



// ================
// === dup_dirs ===
// ================

fn dup_dirs(dir_hashes: &[(String, String)]) -> Vec<String> {
    // Read the mapping of hash -> dir
    eprintln!("Reading (hash -> dir) mapping");
    let dir_hashes = dir_hashes.iter();
    let dir_hashes = dir_hashes.progress();
    let dir_hashes = dir_hashes.map(|(h, d)| (h.as_str(), d.as_str()));
    // FIXME [NP]: Remove this collect
    let dir_hashes = dir_hashes.collect::<Vec<_>>();

    // Convert the (hash -> dir) mapping to (hash -> dir1, dir2, ...)
    let mut map = HashMap::new();
    dir_hashes.into_iter().progress().for_each(|(h, d)| {
        map.entry(h).or_insert_with(Vec::new).push(d);
    });

    // Remove any directories with unique hashes.
    let (_unique, dup) = map
        .into_iter()
        .progress()
        .partition::<HashMap<_, _>, _>(|(_, ds)| ds.len() == 1);

    // Among the duplicate directories, sort them by the length of their path, shortest first.
    let dup = dup
        .into_iter()
        .progress()
        .map(|(h, mut ds)| {
            ds.sort_by_key(|d| d.len());
            (h, ds)
        })
        .collect::<HashMap<_, _>>();

    // If a directory is a subdirectory of another directory with the same hash, remove it.
    let dup = dup
        .into_iter()
        .progress()
        .map(|(h, ds)| {
            let mut ds = ds.into_iter();
            let mut ds2 = vec![ds.next().unwrap()];
            for d in ds {
                let ancestor = ds2.iter().find(|d2| d.starts_with(*d2));
                if ancestor.is_none() {
                    ds2.push(d);
                    //} else {
                    //    eprintln!("Removing {d:?} because of {ancestor:?}");
                }
            }
            (h, ds2)
        })
        .collect::<HashMap<_, _>>();
    // If any categories now only contain one dir, remove them.
    let (_unique, dup) = dup
        .into_iter()
        .progress()
        .partition::<HashMap<_, _>, _>(|(_, ds)| ds.len() == 1);

    // Convert the map<hash, vec<dir>> mapping to vec<(hash, dir)>
    eprintln!("Convert map<hash, vec<dir>> to vec<(hash, dir)>");
    let mut dup_dirs = dup
        .into_iter()
        .progress()
        .flat_map(|(h, ds)| {
            let ds = ds.into_iter();
            ds.map(move |d| (h, d))
        })
        .collect::<Vec<_>>();

    // Sort the mapping by dir name.
    dup_dirs.sort_by_key(|(_, d)| *d);

    // Turn this into a list of strings.
    eprintln!("Convert vec<(hash, dir)> to vec<str>");
    let dup_dirs = dup_dirs
        .par_iter()
        .progress()
        .inspect(|(h, d)| {
            assert!(!h.contains(UNIQUE_SEPARATOR));
            assert!(!d.contains(UNIQUE_SEPARATOR));
        })
        .map(|(h, d)| [*h, *d].join(UNIQUE_SEPARATOR))
        .collect::<Vec<_>>();
    dup_dirs
}

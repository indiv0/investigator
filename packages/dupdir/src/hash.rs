use indicatif::ParallelProgressIterator as _;
use investigator::Hasher as _;
use rayon::iter::ParallelIterator as _;
use rayon::iter::IntoParallelIterator as _;
use rayon::prelude::IndexedParallelIterator;
use std::fs;
use std::str;

// ==============
// === Hasher ===
// ==============

#[derive(Debug)]
pub struct Hasher<'a> {
    paths: &'a crate::Lines,
    skip: Option<usize>,
}

impl<'a> Hasher<'a> {
    pub fn new(paths: &'a crate::Lines) -> Self {
        let skip = Default::default();
        Self { skip, paths }
    }

    pub fn skip(mut self, skip: usize) -> Self {
        self.skip = Some(skip);
        self
    }

    pub fn hash(self) -> Vec<String> {
        self.paths.verify_paths();
        let crate::Lines(paths) = self.paths;
        let skip = self.skip.unwrap_or(0);
        let hashes_and_paths = paths
            .into_par_iter()
            .skip(skip)
            .progress()
            .map(|path| {
                let hash = hash_path(path);
                format!("{hash}  {path}")
            });
        hashes_and_paths.collect::<Vec<_>>()
    }
}

fn hash_path(path: &str) -> String {
    let mut file =
        fs::File::open(path).unwrap_or_else(|_| panic!("Failed to open file: {path:?}"));
    let mut hasher = investigator::T1ha2::default();
    investigator::copy_wide(&mut file, &mut hasher).expect("Failed to hash file");
    let hash = hasher.finish().to_vec();
    hex::encode(hash)
}

// ============
// === Main ===
// ============

pub fn main(paths: &crate::Lines) -> crate::Lines {
    const SKIP: usize = 0;

    let hasher = Hasher::new(paths).skip(SKIP);
    let hashes = hasher.hash();
    crate::Lines(hashes)
}

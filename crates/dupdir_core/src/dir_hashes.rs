use crate::prelude::*;

use indicatif::ParallelProgressIterator as _;
use indicatif::ProgressIterator as _;
use rayon::prelude::*;
use std::collections;
use std::str;

// =================
// === DirHashes ===
// =================

#[derive(Clone, Debug)]
pub struct DirHashes<'a> {
    state: &'a crate::State,
    dir_files: &'a crate::Lines,
}

impl<'a> DirHashes<'a> {
    pub fn new(state: &'a mut crate::State, dir_files: &'a crate::Lines) -> Self {
        Self { state, dir_files }
    }

    pub fn dir_hashes(&self) -> Vec<String> {
        // Read the mapping of hash -> file
        eprintln!("Reading (hash -> file) mapping");
        let hashes = self.read_hashes();

        // Read the mapping of dir -> file
        eprintln!("Reading (dir -> file) mapping");
        let dir_files = self.read_dir_files();

        // Convert the mapping of (hash -> file) to (file -> hash)
        eprintln!("Convert (hash -> file) to (file -> hash)");
        // FIXME [NP]: avoid this collect and do the par_iter earlier.
        let hashes = hashes.collect::<Vec<_>>();
        let hashes = hashes
            .into_par_iter()
            .progress()
            .map(|(h, f)| (f, h))
            .collect::<collections::HashMap<_, _>>();

        // Convert the mapping of (dir -> file) to (dir -> hash)
        eprintln!("Convert (dir -> file) to (dir -> hash)");
        // FIXME [NP]: avoid this collect
        let dir_files = dir_files.collect::<Vec<_>>();
        let dir_hashes = dir_files
            .into_iter()
            .progress()
            .map(|(d, f)| {
                let h = hashes.get(&f);
                let h = h.unwrap_or_else(|| panic!("File must have a hash: {f:?}"));
                let h = *h;
                (d, h)
            })
            .collect::<Vec<_>>();

        // Pare down the (dir -> hash) mapping to just unique hashes within a directory.
        eprintln!("Convert (dir -> hash) to unique hashes");
        let mut map = collections::BTreeMap::new();
        dir_hashes.into_iter().for_each(|(d, h)| {
            map.entry(d)
                .or_insert_with(collections::BTreeSet::new)
                .insert(h);
        });

        // Convert the (dir -> hash1, hash2, hash3, ...) mapping to (dir -> hash)
        eprintln!("Convert (dir -> hash1, hash2, hash3, ...) to (dir -> hash)");
        let dir_hashes = map.into_iter().progress().map(|(d, hs)| {
            let hs = hs.into_iter().collect::<String>();
            let h = crate::hash_bytes(hs.as_bytes());
            (d, h)
        });

        // Sort the (dir -> hash) mapping by hash
        eprintln!("Sort (dir -> hash) by hash");
        let mut dir_hashes = dir_hashes.map(|(d, h)| (h, d)).collect::<Vec<_>>();
        dir_hashes.sort();

        // Turn this into a list of strings.
        eprintln!("Convert (dir -> hash) to list of strings");
        let dir_hashes = dir_hashes
            .par_iter()
            .progress()
            .map(|(h, d)| [h.as_str(), d].join("  "))
            .collect::<Vec<_>>();
        dir_hashes
    }

    fn read_hashes(&self) -> impl Iterator<Item = (&str, &str)> {
        let file_hashes = self.state.hashes.iter();
        file_hashes.map(|(file, hash)| {
            let hash = hash.as_str();
            let file = <PathBuf as AsRef<Path>>::as_ref(file);
            let file = crate::path_to_str(file);
            crate::assert_path_rules(hash);
            crate::assert_path_rules(file);
            (hash, file)
        })
    }

    fn read_dir_files(&self) -> impl Iterator<Item = (&str, &str)> {
        let crate::Lines(lines) = self.dir_files;
        let lines = lines.iter();
        lines.map(|line| {
            let (dir, file) = line
                .split_once(crate::UNIQUE_SEPARATOR)
                .expect("Failed to split line");
            crate::assert_path_rules(dir);
            crate::assert_path_rules(file);
            (dir, file)
        })
    }
}

// ============
// === Main ===
// ============

pub fn main(state: &mut crate::State, dir_files: &crate::Lines) -> crate::Lines {
    let dir_hashes = DirHashes::new(state, dir_files);
    let dir_hashes = dir_hashes.dir_hashes();
    crate::Lines(dir_hashes)
}

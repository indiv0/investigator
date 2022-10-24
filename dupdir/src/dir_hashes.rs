use indicatif::ProgressIterator as _;
use indicatif::ParallelProgressIterator as _;
use rayon::prelude::*;
use std::collections;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead as _;
use std::io::Write as _;
use std::str;



// =================
// === DirHashes ===
// =================

#[derive(Clone, Debug, Default)]
pub struct DirHashes<'a> {
    files: &'a str,
    hashes: &'a str,
}

impl<'a> DirHashes<'a> {
    pub fn files(mut self, files: &'a str) -> Self {
        self.files = files;
        self
    }

    pub fn hashes(mut self, hashes: &'a str) -> Self {
        self.hashes = hashes;
        self
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
        let hashes = hashes
            .into_par_iter()
            .progress()
            .map(|(h, f)| (f, h))
            .collect::<collections::HashMap<_, _>>();

        // Convert the mapping of (dir -> file) to (dir -> hash)
        eprintln!("Convert (dir -> file) to (dir -> hash)");
        let dir_hashes = dir_files
            .into_iter()
            .progress()
            .map(|(d, f)| {
                let h = hashes.get(&f);
                let h = h.unwrap_or_else(|| panic!("File must have a hash: {f:?}"));
                let h = h.clone();
                (d, h)
            })
            .collect::<Vec<_>>();

        // Pare down the (dir -> hash) mapping to just unique hashes within a directory.
        eprintln!("Convert (dir -> hash) to unique hashes");
        let mut map = collections::BTreeMap::new();
        dir_hashes
            .into_iter()
            .for_each(|(d, h)| {
                map.entry(d).or_insert_with(collections::BTreeSet::new).insert(h);
            });

        // Convert the (dir -> hash1, hash2, hash3, ...) mapping to (dir -> hash)
        eprintln!("Convert (dir -> hash1, hash2, hash3, ...) to (dir -> hash)");
        let dir_hashes = map
            .into_iter()
            .progress()
            .map(|(d, hs)| {
                let hs = hs.into_iter().collect::<String>();
                let h = crate::hash_bytes(hs.as_bytes());
                (d, h)
            });

        // Sort the (dir -> hash) mapping by hash
        eprintln!("Sort (dir -> hash) by hash");
        let mut dir_hashes = dir_hashes
            .map(|(d, h)| (h, d))
            .collect::<Vec<_>>();
        dir_hashes.sort();

        // Turn this into a list of strings.
        eprintln!("Convert (dir -> hash) to list of strings");
        let dir_hashes = dir_hashes
            .par_iter()
            .progress()
            .map(|(h, d)| [h.as_str(), d.as_str()].join("  "))
            .collect::<Vec<_>>();
        dir_hashes
    }

    fn read_hashes(&self) -> Vec<(String, String)> {
        let file = fs::File::open(self.hashes).expect("Failed to open file");
        let file = io::BufReader::new(file);
        let lines = file.lines().map(|l| l.expect("Failed to read line"));
        let hashes_and_files = lines.map(|l| {
            let (hash, file) = l.split_once("  ").expect("Failed to split line");
            crate::assert_path_rules(hash);
            crate::assert_path_rules(file);
            (hash.to_string(), file.to_string())
        });
        let hashes_and_files = hashes_and_files.collect();
        hashes_and_files
    }

    fn read_dir_files(&self) -> Vec<(String, String)> {
        let file = fs::File::open(self.files).expect("Failed to open file");
        let file = io::BufReader::new(file);
        let lines = file.lines().map(|l| l.expect("Failed to read line"));
        let dir_files = lines.map(|l| {
            let (dir, file) = l.split_once(crate::UNIQUE_SEPARATOR).expect("Failed to split line");
            crate::assert_path_rules(dir);
            crate::assert_path_rules(file);
            (dir.to_string(), file.to_string())
        });
        let dir_files = dir_files.collect();
        dir_files
    }
}



// ============
// === Main ===
// ============

pub fn main(mut args: env::Args) {
    let files = args.next().expect("Path not provided");
    let hashes = args.next().expect("Path not provided");

    let dir_hashes = DirHashes::default()
        .files(&files)
        .hashes(&hashes);
    let dir_hashes = dir_hashes.dir_hashes();

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    dir_hashes
        .iter()
        .progress()
        .for_each(|l| {
            write!(handle, "{l}\n").expect("Failed to write to stdout");
        })
}



// =============
// === Tests ===
// =============

#[cfg(test)]
mod tests {
    use crate::dir_hashes;

    #[test]
    #[ignore]
    fn test_dir_hashes() {
        const FILES: &str = "./data/dir_files.txt";
        const HASHES: &str = "./data/hashes.txt";
        let dir_hashes = dir_hashes::DirHashes::default()
            .files(FILES)
            .hashes(HASHES);
        let _dir_hashes = dir_hashes.dir_hashes();
    }
}

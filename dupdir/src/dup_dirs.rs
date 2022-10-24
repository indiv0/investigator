use indicatif::*;
use rayon::prelude::*;
use std::collections;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead as _;
use std::io::Write as _;
use std::str;

const UNIQUE_SEPARATOR: &str = ";";



// ===============
// === DupDirs ===
// ===============

#[derive(Clone, Debug, Default)]
pub struct DupDirs<'a> {
    dir_hashes: &'a str,
}

impl<'a> DupDirs<'a> {
    pub fn dir_hashes(mut self, dir_hashes: &'a str) -> Self {
        self.dir_hashes = dir_hashes;
        self
    }

    pub fn dup_dirs(&self) -> Vec<String> {
        // Read the mapping of hash -> dir 
        eprintln!("Reading (hash -> dir) mapping");
        let dir_hashes = self.read_dir_hashes();
        assert_eq!(dir_hashes.len(), 44345);

        // Convert the (hash -> dir) mapping to (hash -> dir1, dir2, ...)
        let mut map = collections::HashMap::new();
        dir_hashes
            .into_iter()
            .progress()
            .for_each(|(h, d)| {
                map.entry(h).or_insert_with(Vec::new).push(d);
            });

        // Remove any directories with unique hashes.
        let (unique, dup) = map
            .into_iter()
            .progress()
            .partition::<collections::HashMap<_, _>, _>(|(_, ds)| ds.len() == 1);
        assert_eq!(unique.len(), 2297);
        assert_eq!(dup.len(), 11481);
        assert_eq!(dup.values().map(|ds| ds.len()).sum::<usize>() + unique.len(), 44345);

        // Among the duplicate directories, sort them by the length of their path, shortest first.
        let dup = dup
            .into_iter()
            .progress()
            .map(|(h, mut ds)| {
                ds.sort_by_key(|d| d.len());
                (h, ds)
            })
            .collect::<collections::HashMap<_, _>>();
        assert_eq!(dup.len(), 11481);
        assert_eq!(dup.values().map(|ds| ds.len()).sum::<usize>() + unique.len(), 44345);
       
        

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
            .collect::<collections::HashMap<_, _>>();
        assert_eq!(dup.len(), 11481);
        assert_eq!(dup.values().map(|ds| ds.len()).sum::<usize>(), 37789);
        // If any categories now only contain one dir, remove them.
        let (unique, dup) = dup
            .into_iter()
            .progress()
            .partition::<collections::HashMap<_, _>, _>(|(_, ds)| ds.len() == 1);
        assert_eq!(unique.len(), 419);
        assert_eq!(dup.len(), 11062);
        assert_eq!(dup.values().map(|ds| ds.len()).sum::<usize>(), 37370);
        assert_eq!(dup.values().map(|ds| ds.len()).sum::<usize>() + unique.len(), 37789);

        // Convert the map<hash, vec<dir>> mapping to vec<(hash, dir)>
        eprintln!("Convert map<hash, vec<dir>> to vec<(hash, dir)>");
        let mut dup_dirs = dup
            .into_iter()
            .progress()
            .flat_map(|(h, ds)| {
                let ds = ds.into_iter();
                let ds = ds.map(move |d| (h.clone(), d));
                ds
            })
            .collect::<Vec<_>>();

        // Sort the mapping by dir name.
        dup_dirs.sort_by_key(|(_, d)| d.clone());

        // Turn this into a list of strings.
        eprintln!("Convert vec<(hash, dir)> to vec<str>");
        let dup_dirs = dup_dirs
            .par_iter()
            .progress()
            .inspect(|(h, d)| {
                assert!(!h.contains(UNIQUE_SEPARATOR));
                assert!(!d.contains(UNIQUE_SEPARATOR));
            })
            .map(|(h, d)| [h.as_str(), d.as_str()].join(UNIQUE_SEPARATOR))
            .collect::<Vec<_>>();
        assert_eq!(dup_dirs.len(), 37370);
        dup_dirs
    }

    fn read_dir_hashes(&self) -> Vec<(String, String)> {
        let file = fs::File::open(self.dir_hashes).expect("Failed to open file");
        let file = io::BufReader::new(file);
        let lines = file.lines().map(|l| l.expect("Failed to read line"));
        let dir_hashes = lines.map(|l| {
            let (hash, dir) = l.split_once("  ").expect("Failed to split line");
            crate::assert_path_rules(hash);
            crate::assert_path_rules(dir);
            (hash.to_string(), dir.to_string())
        });
        let dir_hashes = dir_hashes.collect();
        dir_hashes
    }
}



// ============
// === Main ===
// ============

pub fn main(mut args: env::Args) {
    let dir_hashes = args.next().expect("Path not provided");

    let dup_dirs = DupDirs::default()
        .dir_hashes(&dir_hashes);
    let dup_dirs = dup_dirs.dup_dirs();

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    dup_dirs
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
    use crate::dup_dirs;

    #[test]
    fn test_dup_dirs() {
        const DIR_HASHES: &str = "./data/dir_hashes.txt";
        let dup_dirs = dup_dirs::DupDirs::default()
            .dir_hashes(DIR_HASHES);
        let _dup_dirs = dup_dirs.dup_dirs();
    }
}

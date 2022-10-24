use indicatif::ProgressIterator as _;
use indicatif::ParallelProgressIterator as _;
use investigator::Hasher as _;
use rayon::iter::ParallelIterator as _;
use rayon::iter::IntoParallelRefIterator as _;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead as _;
use std::io::Write as _;
use std::str;



// ==============
// === Hasher ===
// ==============

#[derive(Clone, Debug, Default)]
pub struct Hasher<'a> {
    path: &'a str,
    skip: Option<usize>,
}

impl<'a> Hasher<'a> {
    pub fn skip(mut self, skip: usize) -> Self {
        self.skip = Some(skip);
        self
    }

    pub fn path(mut self, path: &'a str) -> Self {
        self.path = path;
        self
    }

    pub fn hash(&self) -> Vec<String> {
        let mut paths = self.read_paths();
        if let Some(skip) = self.skip {
            paths = paths.into_iter().skip(skip).collect::<Vec<_>>();
        }
        let hashes = paths.par_iter().progress().map(|p| self.hash_path(p)).collect::<Vec<_>>();

        let hashes_and_paths = hashes.into_iter().progress().zip(paths);
        let hashes_and_paths = hashes_and_paths.map(|(h, p)| [h, p].join("  "));
        let hashes_and_paths = hashes_and_paths.collect::<Vec<_>>();
        hashes_and_paths
    }

    fn read_paths(&self) -> Vec<String> {
        let file = fs::File::open(self.path).expect("Failed to open file");
        let file = io::BufReader::new(file);
        let lines = file.lines().map(|l| l.expect("Failed to read line"));
        let lines = lines.inspect(|l| crate::assert_path_rules(&l));
        let paths = lines.collect();
        paths
    }

    fn hash_path(&self, path: &str) -> String {
        let mut file = fs::File::open(path).unwrap_or_else(|_| panic!("Failed to open file: {path:?}"));
        let mut hasher = investigator::T1ha2::default();
        investigator::copy_wide(&mut file, &mut hasher).expect("Failed to hash file");
        let hash = hasher.finish().to_vec();
        let hash = hex::encode(hash);
        hash
    }
}



// ============
// === Main ===
// ============

pub fn main(mut args: env::Args) {
    const SKIP: usize = 0;

    let path = args.next().expect("Path not provided");

    let hasher = Hasher::default()
        .path(&path)
        .skip(SKIP);
    let hashes = hasher.hash();

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    hashes
        .iter()
        .progress()
        .for_each(|h| {
            write!(handle, "{h}\n").expect("Failed to write to stdout");
        })
}



// =============
// === Tests ===
// =============

#[cfg(test)]
mod tests {
    use crate::hash;

    #[test]
    #[ignore]
    fn test_hash() {
        const PATH: &str = "./data/files.txt";
        let hasher = hash::Hasher::default()
            .path(PATH);
        let _hashes = hasher.hash();
    }
}


//! Usage:
//! ```
//! clear && cargo check && RUST_BACKTRACE=1 time cargo run --release
//! cat dupdirs_by_path.txt | awk '{ print length, $0 }' | sort -n -s -r | cut -d" " -f2- > tmp.txt
//! scp tmp.txt 172.30.194.6:
//! ssh 172.30.194.6
//! sudo mv tmp.txt /storage/tmp.txt
//! sudo su
//! cd /storage
//! cat tmp.txt | grep -v "'" | grep -v ' \./lap-ca-nik-01\| \./lab-ca-kvm-02' | cut -d' ' -f2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27 | xargs -I{} du -d 0 "{}" | sort -n
//! ```
use std::collections;
use std::fs;
use std::io;
use std::io::BufRead as _;
use std::io::Write as _;
use std::path;
use std::thread;
use std::time;
use indicatif::ProgressIterator as _;
use investigator::Hasher as _;

type Error = Box<dyn std::error::Error>;

// ==================
// === FileRecord ===
// ==================

#[derive(Clone, Debug, Default)]
struct FileRecord {
    hash: String,
    path: String,
}

// ==============
// === Reader ===
// ==============

#[derive(Clone, Debug)]
struct Reader<I>
where
    I: Iterator<Item = String>,
{
    reader: I,
}

impl<I> Reader<I>
where
    I: Iterator<Item = String>,
{
    fn new(reader: I) -> Self {
        Self { reader }
    }

    fn read_record(&mut self, record: &mut FileRecord) -> Result<bool, Error> {
        if let Some(line) = self.reader.next() {
            let (hash, path) = line.split_once("  ").ok_or("invalid")?;
            assert_eq!(record.hash, record.hash.trim());
            assert_eq!(record.path, record.path.trim());
            record.hash.replace_range(.., hash);
            record.path.replace_range(.., path);
            return Ok(true);
        }

        Ok(false)
    }
}

#[derive(Clone, Debug, Default)]
struct Dir {
    hash: Option<String>,
    files: collections::HashSet<(String, String)>,
    dirs: collections::HashSet<path::PathBuf>,
}

// Steps:
// 1. Make a list of every directory, and which files/directories are in it.
// 2. For each directory, compute it's hash by 
#[derive(Clone, Debug, Default)]
struct Entries {
    //// File -> Hash
    //// Each file has one, pre-computed hash.
    //file_hashes: collections::HashMap<String, String>,
    // Dir -> Vec<(File, Hash)>
    // Each dir has a list of files in it.
    // Each file must have a unique path.
    dir_files: collections::HashMap<path::PathBuf, Dir>,
}

fn valid_path<P>(path: &P) -> bool
where
    P: AsRef<path::Path>,
{
    path.as_ref() != path::Path::new(".") && path.as_ref() != path::Path::new("")
}

impl Entries {
    fn add_file(&mut self, record: &FileRecord) -> bool {
        // Determine the path to the parent directory of the file.
        let path = path::Path::new(&record.path);
        let dir = path.parent().unwrap();
        if !valid_path(&dir){
            println!("Skipping {:?}", path);
            return false;
        }

        // List the file as a child under it's parent directory.
        // Each file path under a directory must be unique.
        let dir = self.get_mut_dir(&*dir);
        let new = dir.files.insert((record.path.clone(), record.hash.clone()));
        assert!(new, "Each file should appear once in it's parent directory.");
        true
    }

    fn get_mut_dir(&mut self, dir: &path::Path) -> &mut Dir {
        // Exclude the root paths "." and "" from consideration.
        assert!(dir != path::Path::new("."));
        assert!(dir != path::Path::new(""));

        if !self.dir_files.contains_key(dir) {
            // If the entry doesn't exist, add it's ancestors to our list of directories.
            let dir = dir.to_path_buf();
            let parent = dir.parent();
            let parent = parent.filter(valid_path);
            if let Some(parent) = parent {
                let parent = self.get_mut_dir(parent);
                // Add the directory as a child of it's parent.
                let new = parent.dirs.insert(dir.clone());
                assert!(new, "Each directory should appear in it's parent directory.");
            }
            // Then add the directory itself.
            let new = self.dir_files.insert(dir, Default::default());
            assert!(new.is_none(), "We should never be inserting the same directory twice.");
        }
        self.dir_files.get_mut(dir).expect("Impossible since we just inserted a value.")
    }

    // If the dir does not already have a hash, compute it.
    // The hash consists of:
    // - The hashes of all the files in the directory, AND
    // - The hashes of all files of every descendant directory.
    // This way, separate directories that contain all the same files (at any child path)
    // will have the same hash. This is a good way to find directories with identical
    // contents that have been moved around.
    fn dir_hash(&self, path: &path::Path) -> String {
        // Exclude the root paths "." and "" from consideration.
        assert!(path != path::Path::new("."));
        assert!(path != path::Path::new(""));

        let dir = self.dir_files.get(path).expect("Directory should exist.");
        if dir.hash.is_none() {
            let hashes = self.file_hashes(path);
            let mut hashes = hashes.collect::<Vec<_>>();
            hashes.sort();
            hashes.dedup();
            let hashes = hashes.into_iter().collect::<String>();
            let hash = hash(hashes.as_bytes());
            hash
        } else {
            dir.hash.clone().expect("Impossible since we just checked.")
        }
    }

    fn file_hashes(&self, dir: &path::Path) -> impl Iterator<Item = &str> {
        // Exclude the root paths "." and "" from consideration.
        assert!(dir != path::Path::new("."));
        assert!(dir != path::Path::new(""));

        // Get the hashes of just this directory.
        let dir = self.dir_files.get(dir).expect("Directory should exist.");
        // FIXME [NP]: avoid this box
        let mut hashes: Box<dyn Iterator<Item = &str>> = Box::new(dir.files.iter().map(|(_path, hash)| hash).map(AsRef::as_ref));

        // Add the hashes of all files in all descendant directories.
        for dir in &dir.dirs {
            // FIXME [NP]: avoid this box
            hashes = Box::new(hashes.chain(self.file_hashes(dir)));
        }

        hashes
    }

    fn file_count(&self) -> usize {
        self.dir_files.values().map(|dir| dir.files.len()).sum()
    }

    fn dir_count(&self) -> usize {
        self.dir_files.len()
    }

    // Gets a list of every directory whose file contents are identical to at least one other directory.
    fn dup_dirs(&self) -> Vec<(&path::Path, &str)> {
        // Create a map of directory paths to hashes.
        let dirs = self
            .dir_files
            .iter()
            .map(|(path, dir)| {
                let path = path.as_path();
                let hash = dir.hash.as_ref().map(|s| s.as_str()).expect("Directory should have a hash.");
                (path, hash)
            });

        // Create a reverse lookup table of hashes to directories.
        let lookup = dirs
            .clone()
            .fold(collections::HashMap::new(), |mut lookup, (path, hash)| {
                let dirs = lookup.entry(hash).or_insert_with(Vec::new);
                dirs.push(path);
                lookup
            });

        // Make a list of all directories that *aren't* unique.
        let dupes = dirs
            .filter(|(_path, hash)| {
                let dirs = lookup.get(hash).expect("Directory should have a hash.");
                dirs.len() > 1
            });

        // Exclude any duplicate directory whose hash matches that of it's parent.
        // We only care about the *root* of the duplicated subtree.
        let dupes = dupes
            .filter(|(path, hash)| {
                let parent = path.parent();
                let parent = parent.filter(valid_path);
                if let Some(parent) = parent {
                    let parent_hash = self.dir_hash(parent);
                    if parent_hash == *hash {
                        return false;
                    }
                }
                true
            });
        let dupes = dupes.collect::<collections::HashMap<_, _>>();

        // If any ancestor of the duplicate directory is also a duplicate of another directory, exclude it.
        // We only care about the *root* of the duplicated subtree.
        let dupes = dupes
            .clone()
            .into_iter()
            .filter(|(path, _hash)| {
                let ancestors = path.ancestors().skip(1);
                !ancestors
                    .into_iter()
                    .filter(valid_path)
                    .any(|ancestor| dupes.contains_key(ancestor))
            });
        let dupes = dupes.collect::<collections::HashMap<_, _>>();

        // Make a list of all directories that *aren't* unique.
        // We do this again because some directories may have been considered as duplicates of their
        // ancestors, and thus accidentally included in the list.
        let lookup = dupes
            .clone()
            .into_iter()
            .fold(collections::HashMap::new(), |mut lookup, (path, hash)| {
                let dirs = lookup.entry(hash).or_insert_with(Vec::new);
                dirs.push(path);
                lookup
            });
        let dupes = dupes
            .into_iter()
            .filter(|(_path, hash)| {
                let dirs = lookup.get(hash).expect("Directory should have a hash.");
                dirs.len() > 1
            });
        let dupes = dupes.collect::<collections::HashMap<_, _>>();

        let dupes = dupes.into_iter();
        dupes.collect()
    }
}

fn hash(bytes: &[u8]) -> String {
    let mut hasher = investigator::T1ha2::default();
    investigator::copy_wide(&mut &bytes[..], &mut hasher).unwrap();
    let hash = hasher.finish().to_vec();
    hex::encode(hash)
}

fn main() {
    // For each file with hashes, add it to our list of entries.
    let mut record = FileRecord::default();
    let mut entries = Entries::default();
    let files = ["localhashes.txt", "remotehashes.txt"];
    let mut file_count = 0;
    for file in files {
        // Parse the list of Hash -> File path paths as a list of lines.
        println!("Reading entries from {}...", file);
        let file = fs::File::open(file).unwrap();
        let file = std::io::BufReader::new(file);
        let lines = file.lines().map(|l| l.unwrap());

        // For each Hash -> File path pair, insert it into our list of entries.
        let mut reader = Reader::new(lines);
        while let Ok(true) = reader.read_record(&mut record) {
            let added = entries.add_file(&record);
            if added { file_count += 1; }
            // FIXME [NP]: for now, only read the first 1000 files.
            //if file_count >= 1000 {
            //    break;
            //}
        }
    }
    println!("Found {} files in {} directories.", entries.file_count(), entries.dir_count());
    assert!(file_count == entries.file_count(), "Unexpected file count {}. Expected {}.", file_count, entries.file_count());

    println!("Computing dir hashes...");
    let dir_paths = entries.dir_files.keys().cloned().collect::<Vec<_>>();
    for path in dir_paths.into_iter().progress() {
        let hash = entries.dir_hash(&path);
        let dir = entries.dir_files.get_mut(&path).expect("Impossible since we just computed it's hash");
        dir.hash = Some(hash);
    }

    println!("Writing dir hashes to {}...", "dirhashes.txt");
    let file = fs::File::create("dirhashes.txt").unwrap();
    let mut file = io::LineWriter::new(file);
    for (path, dir) in entries.dir_files.iter().progress() {
        write!(file, "{} {}\n", dir.hash.as_ref().unwrap(), path.display()).unwrap();
    }

    println!("Finding directories with duplicate contents...");
    let mut dup_dirs = entries.dup_dirs();
    println!("Found {} directories with duplicate contents.", dup_dirs.len());

    // Sort directories by hash.
    println!("Writing duplicate directories to {} (sorted by hash)...", "dupdirs_by_hash.txt");
    dup_dirs.sort_by_key(|(_dir, hash)| *hash);
    let file = fs::File::create("dupdirs_by_hash.txt").unwrap();
    let mut file = io::LineWriter::new(file);
    for (dir, hash) in dup_dirs.iter() {
        write!(file, "{} {}\n", hash, dir.display()).unwrap();
    }

    // Sort directories by file path.
    println!("Writing duplicate directories to {} (sorted by path)...", "dupdirs_by_path.txt");
    dup_dirs.sort_by_key(|(dir, _hash)| *dir);
    let file = fs::File::create("dupdirs_by_path.txt").unwrap();
    let mut file = io::LineWriter::new(file);
    for (dir, hash) in dup_dirs {
        write!(file, "{} {}\n", hash, dir.display()).unwrap();
    }

    // Wait for exit.
    println!("Done...");
    loop {
        thread::sleep(time::Duration::from_secs(1));
    }


    //let entries = lines.into_iter().map(|x| x.split_once("  "));
}

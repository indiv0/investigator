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

        // Convert the (hash -> dir) mapping to (hash -> dir1, dir2, ...)
        let mut map = collections::HashMap::new();
        dir_hashes
            .into_iter()
            .progress()
            .for_each(|(h, d)| {
                map.entry(h).or_insert_with(Vec::new).push(d);
            });

        // Remove any directories with unique hashes.
        let (_unique, dup) = map
            .into_iter()
            .progress()
            .partition::<collections::HashMap<_, _>, _>(|(_, ds)| ds.len() == 1);

        // Among the duplicate directories, sort them by the length of their path, shortest first.
        let dup = dup
            .into_iter()
            .progress()
            .map(|(h, mut ds)| {
                ds.sort_by_key(|d| d.len());
                (h, ds)
            })
            .collect::<collections::HashMap<_, _>>();
       
        

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
        // If any categories now only contain one dir, remove them.
        let (_unique, dup) = dup
            .into_iter()
            .progress()
            .partition::<collections::HashMap<_, _>, _>(|(_, ds)| ds.len() == 1);

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
    use std::fs;
    use std::io::Write as _;
    use std::process;
    use std::io;
    use std::env;

    const OUT_FILES: &str = "out/files.txt";
    const OUT_HASHES: &str = "out/hashes.txt";
    const OUT_DIR_FILES: &str = "out/dir_files.txt";
    const OUT_DIR_HASHES: &str = "out/dir_hashes.txt";
    const OUT_DUP_DIRS: &str = "out/dup_dirs.txt";

    #[test]
    fn test_dup_dirs() {
        create_dir("out").unwrap();
        find();
        hash();
        dir_files();
        dir_hashes();
        dup_dirs();
    }

    fn find() {
        let src = src_dir();
        let args = format!("find {src}");
        cargo_run_command(&args, OUT_FILES);
    }

    fn hash() {
        let args = format!("hash {OUT_FILES}");
        cargo_run_command(&args, OUT_HASHES);
    }

    fn dir_files() {
        let args = format!("dir_files {OUT_FILES}");
        cargo_run_command(&args, OUT_DIR_FILES);
    }

    fn dir_hashes() {
        let args = format!("dir_hashes {OUT_DIR_FILES} {OUT_HASHES}");
        cargo_run_command(&args, OUT_DIR_HASHES);
    }

    fn dup_dirs() {
        let args = format!("dup_dirs {OUT_DIR_HASHES}");
        cargo_run_command(&args, OUT_DUP_DIRS);
    }

    fn create_dir(path: &str) -> io::Result<()> {
        match fs::create_dir(path) {
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => Ok(()),
            result => result,
        }
    }

    fn src_dir() -> String {
        let cwd = env::current_dir().unwrap();
        let src = cwd.join("src");
        let src = src.to_str();
        let src = src.unwrap();
        assert!(!src.contains(" "), "src path contains spaces: {:?}", src);
        src.to_string()
    }

    fn cargo_run_command(args: &str, out_file: &str) {
        let args = args.split(" ");
        let output = process::Command::new("cargo")
            .arg("run")
            .args(args)
            .output()
            .unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        eprintln!("stderr: {}", stderr);
        println!("stdout: {}", stdout);
        let status = output.status;
        assert!(status.success(), "Failed to run cargo: {:?}", status);
        write_to_file(out_file, &stdout);
    }

    fn write_to_file(path: &str, contents: &str) {
        let mut file = fs::File::create(path).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }
}

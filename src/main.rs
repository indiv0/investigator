use std::collections;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::iter;
use std::path;
use std::hash::Hash;
use std::hash::Hasher;

enum Command {
    Investigate,
    Compare,
}

fn main() {
    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let mut args = env::args().skip(1);
        let command = args.next();
        match command.as_ref().map(|s| s.as_str()) {
            Some("compare") => {
                let action = args.next().unwrap();
                let file_1 = args.next().unwrap();
                let file_2 = args.next().unwrap();
                compare(&action, &file_1, &file_2)
            },
            Some("investigate") => investigate(),
            _ => panic!("Unknown command"),
        }
    }

    main().unwrap();
}

fn investigate() -> Result<(), Box<dyn std::error::Error>> {
    for file in walk_files() {
        let file = file?;
        let entry = hash_file(file)?;
        write_hash(entry);
    }
    Ok(())
}

struct File {
    path: path::PathBuf,
}

impl File {
    fn new(path: path::PathBuf) -> Self {
        Self { path }
    }
}

fn walk_files() -> impl Iterator<Item = Result<File, Box<dyn std::error::Error>>> {
    let mut walker = walkdir::WalkDir::new(".").into_iter();

    iter::from_fn(move || {
        loop {
            match walker.next() {
                Some(Ok(entry)) => {
                    if entry.file_type().is_file() {
                        let path = entry.path().to_path_buf();
                        let file = File::new(path);
                        return Some(Ok(file));
                    }
                }
                Some(Err(error)) => return Some(Err(error).map_err(Into::into)),
                None => return None,
            }
        }
    })
}

struct HashWriter<T>(T)
where
    T: Hasher;

impl<T> io::Write for HashWriter<T>
where
    T: Hasher,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn hash_file(file: File) -> io::Result<(File, u64)> {
    let hasher = rustc_hash::FxHasher::default();
    let mut hash_writer = HashWriter(hasher);
    let mut fs_file = fs::File::open(&file.path)?;
    io::copy(&mut fs_file, &mut hash_writer)?;
    let hash = hash_writer.0.finish();
    Ok((file, hash))
}

fn write_hash((file, hash): (File, u64)) {
    println!("{hash:016x} {path}", path = file.path.display());
}

fn hash(mut reader: impl io::Read) -> u64 {
    let mut hasher = rustc_hash::FxHasher::default();
    let mut buf = [0; 1024];
    loop {
        let n = reader.read(&mut buf).unwrap();
        if n == 0 {
            break;
        }
        for i in 0..n {
            buf[i].hash(&mut hasher);
        }
    }
    hasher.finish()
}

fn compare(action: &str, file_1: &str, file_2: &str) -> Result<(), Box<dyn std::error::Error>> {
    //let file_1 = read_file(file_1)?;
    //let file_2 = read_file(file_2)?;
    let file_1 = read_file(file_1)?.collect::<Result<Vec<_>, _>>()?;
    let file_2 = read_file(file_2)?.collect::<Result<Vec<_>, _>>()?;
    let hashes_1 = file_1.iter().map(|s| s.as_str()).map(read_hash).collect::<Result<collections::HashMap<_, _>, _>>()?;
    let hashes_2 = file_2.iter().map(|s| s.as_str()).map(read_hash).collect::<Result<collections::HashMap<_, _>, _>>()?;

    match action {
        "AND" => {
            let keys_1 = hashes_1.keys().cloned().collect::<collections::HashSet<_>>();
            let keys_2 = hashes_2.keys().cloned().collect::<collections::HashSet<_>>();
            let keys = keys_1.intersection(&keys_2);

            for key in keys {
                println!("{key:016x} {path}", path = hashes_1[key]);
            }
        },
        "NAND" => {
            let keys_1 = hashes_1.keys().cloned().collect::<collections::HashSet<_>>();
            let keys_2 = hashes_2.keys().cloned().collect::<collections::HashSet<_>>();
            let keys = keys_1.difference(&keys_2);

            for key in keys {
                println!("{key:016x} {path}", path = hashes_1[key]);
            }
        },
        _ => panic!("Unknown action"),
    }
    Ok(())
}

fn read_file(path: &str) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_hash(hash_line: &str) -> Result<(u64, &str), Box<dyn std::error::Error>> {
    let (hash, path) = hash_line.split_at(16);
    let hash = u64::from_str_radix(hash, 16)?;
    let path = path.trim_start();
    Ok((hash, path))
}

#[cfg(test)]
mod test {
    #[test]
    fn hash_is_deterministic() {
        for _ in 0..100 {
            let mut reader = std::io::Cursor::new("Hello, world!");
            let hash = super::hash(&mut reader);
            assert_eq!(hash, 0x56_2d_c0_28_4e_81_df_f2);
        }
    }
}

//! Usage:
//! ```
//! mkdir -p target/data
//! clear && cargo check && RUST_BACKTRACE=1 time cargo run --release old_dup_dirs
//! cat target/data/dupdirs_by_path.txt | awk '{ print length, $0 }' | sort -n -s -r | cut -d" " -f2- > tmp.txt
//! scp tmp.txt 172.30.194.6:
//! ssh 172.30.194.6
//! sudo mv tmp.txt /storage/tmp.txt
//! sudo su
//! cd /storage
//! cat tmp.txt | grep -v "'" | grep -v ' \./lap-ca-nik-01\| \./lab-ca-kvm-02' | cut -d' ' -f2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27 | xargs -I{} du -d 0 "{}" | sort -n
//! ```
//!
//! Other usage:
//! ```
//! mkdir -p target/data
//! clear && cargo check && RUST_BACKTRACE=1 time cargo run --release old_dup_dirs
//! cat target/data/dupdirs_by_path.txt | cut -d' ' -f2- | xargs -d '\n' du -d0 | sort -n
//! ```
//!
//! New usage:
//! ```
//! clear && \
//!   cargo check && \
//!   cargo test --color=always 2>&1 && \
//!   cargo build --release
//! find /Users/indiv0/Desktop/files -type f -name '*'$'\r''*'
//! find /Users/indiv0/Desktop/files -type f -name '*'$'\r''*' -delete
//! find /Users/indiv0/Desktop/files -not -perm -u=r -not -perm -u=w -not -perm -u=x -ls
//! find /Users/indiv0/Desktop/files -not -perm -u=r -not -perm -u=w -not -perm -u=x -delete
//! mkdir -p target/data
//! sudo su
//! time ./target/release/dupdir find /Users/indiv0/Desktop/files > target/data/files.txt && chown indiv0 target/data/files.txt
//! time ./target/release/dupdir hash target/data/files.txt > target/data/hashes.txt && chown indiv0 target/data/hashes.txt
//! time ./target/release/dupdir dir_files target/data/files.txt > target/data/dir_files.txt && chown indiv0 target/data/dir_files.txt
//! time ./target/release/dupdir dir_hashes target/data/dir_files.txt target/data/hashes.txt > target/data/dir_hashes.txt && chown indiv0 target/data/dir_hashes.txt
//! time ./target/release/dupdir dup_dirs target/data/dir_hashes.txt > target/data/dup_dirs.txt && chown indiv0 target/data/dup_dirs.txt
//! exit
//! cat target/data/dup_dirs.txt | cut -d';' -f2 | xargs -d '\n' du -d0 | sort -n
//! ```
use std::env;
use std::path;
use investigator::Hasher as _;

mod find;
mod dir_files;
mod dir_hashes;
mod dup_dirs;
mod hash;
mod old_dup_dirs;
mod reader;

const UNIQUE_SEPARATOR: &str = "    ";



fn main() {
    let mut args = env::args();
    let _command = args.next();
    let command = args.next().expect("Command required");
    match command.as_str() {
        "find" => find::main(args),
        "hash" => hash::main(args),
        "dir_files" => dir_files::main(args),
        "dir_hashes" => dir_hashes::main(args),
        "dup_dirs" => dup_dirs::main(args),
        "old_dup_dirs" => old_dup_dirs::main(),
        other => panic!("Unknown command: {other:?}"),
    }
}

fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = investigator::T1ha2::default();
    investigator::copy_wide(&mut &bytes[..], &mut hasher).unwrap();
    let hash = hasher.finish().to_vec();
    hex::encode(hash)
}

#[inline]
fn assert_path_rules(p: &str) {
    assert!(!p.contains("\r"), "Unsupported character in path");
    assert!(!p.is_empty(), "Empty path");
    assert_eq!(p, p.trim(), "Extra whitespace in path");
}

#[inline]
fn path_to_str(p: &path::Path) -> &str {
    p.to_str().expect("Path should be valid UTF-8")
}

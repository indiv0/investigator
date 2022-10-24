//! Usage:
//! ```
//! mkdir data_old
//! clear && cargo check && RUST_BACKTRACE=1 time cargo run --release old_dup_dirs
//! cat data_old/dupdirs_by_path.txt | awk '{ print length, $0 }' | sort -n -s -r | cut -d" " -f2- > tmp.txt
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
//! mkdir data_old
//! clear && cargo check && RUST_BACKTRACE=1 time cargo run --release old_dup_dirs
//! cat dupdirs_by_path.txt | cut -d' ' -f2- | xargs -d '\n' du -d0 | sort -n
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
//! mkdir data
//! sudo su
//! time ./target/release/dupdir find /Users/indiv0/Desktop/files > data/files.txt && chown indiv0 data/files.txt
//! time ./target/release/dupdir hash data/files.txt > data/hashes.txt && chown indiv0 data/hashes.txt
//! time ./target/release/dupdir ancestors data/files.txt > data/ancestors.txt
//! time ./target/release/dupdir dir_files data/files.txt > data/dir_files.txt
//! time ./target/release/dupdir dir_hashes data/dir_files.txt data/hashes.txt > data/dir_hashes.txt
//! time ./target/release/dupdir dup_dirs data/dir_hashes.txt > data/dup_dirs.txt
//! cat data/dup_dirs.txt | cut -d';' -f2 | xargs -d '\n' du -d0 | sort -n
//! exit
//! ```
use std::env;
use std::path;
use investigator::Hasher as _;

mod ancestors;
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
        "ancestors" => ancestors::main(args),
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

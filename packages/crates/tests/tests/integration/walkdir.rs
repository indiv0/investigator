use crate::prelude::*;

use std::fs;
use std::io;



// FIXME [NP]: Uncomment
//// ====================
//// === test_walkdir ===
//// ====================
//
///// Tests that the custom `walkdir` implementation behaves identically to the real `walkdir`.
//#[test]
//fn test_walkdir() {
//    model! {
//        Model => let (mut tmp, mut dirs) = {
//            let tmp = TempDir::new().expect("Temp directory");
//            let dirs = vec![];
//            (tmp, dirs)
//        },
//        Implementation => let mut _i = {},
//        CreateDir(String)(p in crate::fs::prop_path()) => {
//            let p = tmp.path().join(&p);
//            match p.create_dir_all() {
//                Ok(()) => {},
//                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {},
//                Err(e) if e.kind() == io::ErrorKind::NotADirectory => {},
//                Err(e) => panic!("Create directory: {e:?}"),
//            }
//            {
//                let ancestors = p.ancestors();
//                // Only include the ancestors up to and including the test directory.
//                let ancestors = ancestors.take_while(|p| p != &tmp.path());
//                let mut ancestors = ancestors.map(Path::to_path_buf);
//                dirs.extend(&mut ancestors);
//            }
//            tmp = tmp;
//        },
//        WriteFile((String, Vec<u8>))((p, b) in crate::fs::prop_file_name_and_bytes()) => {
//            // FIXME [NP]: Choose the path randomly rather than using the last one?
//            if let Some(d) = dirs.last() {
//                let p = d.join(&p);
//                match fs::write(&p, &b) {
//                    Ok(()) => {},
//                    Err(e) if e.kind() == io::ErrorKind::IsADirectory => {},
//                    e @ Err(_) => panic!("Write: {:?}", e.with_err_path(p)),
//                }
//            }
//            tmp = tmp;
//        },
//        WalkDir(())(_p in pt::arbitrary::any::<()>()) => {
//            let p = tmp.path();
//            let expected = walkdir_model(p);
//            let expected = walkdir(p, expected);
//            let actual = walkdir_impl(p);
//            let actual = walkdir(p, actual);
//            pretty_assertions::assert_eq!(expected, actual);
//            tmp = tmp;
//        }
//    }
//}
//
///// A recursive directory search on the `root` path, taking an iterator of directory entries.
//fn walkdir(root: impl AsRef<Path>, entries: impl Iterator<Item = io::Result<DirEntry>>) -> Vec<String> {
//    let root = root.as_ref();
//    let namespace = root.to_str();
//    let namespace = namespace.expect("Namespace");
//    let entries = entries.filter_map(|e| {
//        let e = e.expect("Entry");
//        let path = e.path();
//        if !e.file_type().is_file() {
//            return None;
//        }
//        let path = path.to_str();
//        let path = path.expect("UTF-8");
//        assert!(!path.contains('\r'), "Unsupported character \"\\r\"");
//        assert!(!path.is_empty(), "Non-empty path");
//        let path = path.strip_prefix(namespace);
//        let path = path.expect("Missing namespace");
//        Some(path.to_string())
//    });
//    entries.collect()
//}

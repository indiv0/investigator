use crate::prelude::*;

use std::fs;
use std::io;



// =================
// === Constants ===
// =================

/// Path to the directory on the real file system to use for testing.
///
/// This directory will be deleted and re-created repeatedly. Take care not to use a directory that
/// contains important files.
const TEST_DIR: &str = "./tests/test_data/fs";
/// Namespace to use under the test directory.
///
/// Currently this is empty because the test directory is used directly in the tests.
const NAMESPACE: &str = "";
/// Regular expression used to generate example paths when creating directories.
// TODO [NP]: Update the mock implementation to handle the following correctly:
// - `` (empty string)
// - `.` (`\x2E`)
// - `/`
// - `\0`
// - Remainder of unicode (e.g., `\u{323b0}`)
// TODO [NP]: Test that this behaves with the expanded charset
// `[^./\0\u{0000E0}-\u{10FFFF}][^.\0\u{0000E0}-\u{10FFFF}]*`.
// The charset was simplified to ensure that paths with subdirectories are generated frequently.
const PATH_REGEX: &str = "[aA]( ?/[aA ]?){0,2}";
/// Regular expression used to generate example file names.
const FILE_REGEX: &str = "[aA ]{1,3}";



// ===========================
// === assert_eq_io_result ===
// ===========================

/// Asserts that the two [`io::Result`]s are equal.
///
/// If both are [`Ok`], compares the values of the two results.
/// If both are [`Err`], compares the [`ErrorKind`]s of the two results.
///
/// [`io::Result`]: std::io::Result
/// [`Ok`]: std::Result::Ok
/// [`Err`]: std::Result::Err
/// [`ErrorKind`]: std::io::ErrorKind
pub(crate) fn assert_eq_io_result<T>(expected: io::Result<T>, actual: io::Result<T>)
where
    T: Debug + PartialEq,
{
    match (&expected, &actual) {
        (Ok(expected), Ok(actual)) if expected == actual => {},
        (Err(expected), Err(actual)) if expected.kind() == actual.kind() => {},
        _ => panic!("Expected: {expected:?}, actual: {actual:?}"),
    }
}



// =================
// === prop_path ===
// =================

/// Returns a `proptest` strategy for generating arbitrary paths.
pub(crate) fn prop_path() -> impl pt::prelude::Strategy<Value = String> {
    pt::string::string_regex(PATH_REGEX).expect("Invalid regex")
}



// ======================
// === prop_file_name ===
// ======================

/// Returns a `proptest` strategy for generating arbitrary file names.
pub(crate) fn prop_file_name() -> impl pt::prelude::Strategy<Value = String> {
    pt::string::string_regex(FILE_REGEX).expect("Invalid regex")
}



// ================================
// === prop_file_name_and_bytes ===
// ================================

/// Returns a `proptest` strategy for generating arbitrary pairs of (`file_name, bytes)`.
pub(crate) fn prop_file_name_and_bytes() -> impl pt::prelude::Strategy<Value = (String, Vec<u8>)> {
    let file_name = pt::string::string_regex(FILE_REGEX).expect("Invalid regex");
    let bytes = pt::arbitrary::any_with::<Vec<u8>>(pt::collection::size_range(0..=4).lift());
    (file_name, bytes)
}



// ===========================
// === prop_path_and_bytes ===
// ===========================

/// Returns a `proptest` strategy for generating arbitrary pairs of `(path, bytes)`.
pub(crate) fn prop_path_and_bytes() -> impl pt::prelude::Strategy<Value = (String, Vec<u8>)> {
    (prop_path(), pt::arbitrary::any_with::<Vec<u8>>(pt::collection::size_range(0..=4).lift()))
}



// FIXME [NP]: Uncomment
//// =============================
//// === test_mock_file_system ===
//// =============================
//
///// Tests that the in-memory file system behaves identically to the real file system.
//#[test]
//fn test_mock_file_system() {
//    model! {
//        Model => let mut m = create_or_empty_dir(TEST_DIR),
//        Implementation => let mut i = Fs::mock(),
//        CreateDir(String)(p in prop_path()) => {
//            // Create the directory on the disk.
//            println!("Model::CreateDir: {p:?}.");
//            let expected = m.create_dir(&p);
//            let actual = i.create_dir(&p);
//            assert_eq_io_result(expected, actual)
//        },
//        WriteFile((String, Vec<u8>))((p, b) in prop_path_and_bytes()) => {
//            // Write the file to the disk.
//            println!("Model::WriteFile: {p:?}.");
//            let expected = m.write(&p, &b);
//            let actual = i.write(&p, &b);
//            assert_eq_io_result(expected, actual)
//        },
//        FileType(String)(p in prop_path()) => {
//            // Check the file type of the path.
//            println!("Model::FileType: {p:?}.");
//            let expected = m.file_type(&p);
//            let actual = i.file_type(&p);
//            assert_eq_io_result(expected, actual)
//        },
//        ReadDir(String)(p in prop_path()) => {
//            // Iterate over the entries within a directory.
//            println!("Model::ReadDir: {p:?}.");
//            let expected = find_files_core::fs::read_dir(&m, &p);
//            let actual = find_files_core::fs::read_dir(&i, &p);
//            // FIXME [NP]: Macro-ify?
//            match (&expected, &actual) {
//                (Ok(expected), Ok(actual)) => {},
//                (Err(expected), Err(actual)) if expected.kind() == actual.kind() => return Ok(()),
//                _ => panic!("Expected: {expected:?}, actual: {actual:?}"),
//            };
//            // FIXME [NP]: Document that unwrap is safe because we just matched.
//            let expected = expected.unwrap();
//            let actual = actual.unwrap();
//            let mut entries = expected.zip(actual);
//            while let Some((expected, actual)) = entries.next() {
//                // FIXME [NP]: Macro-ify?
//                match (&expected, &actual) {
//                    (Ok(expected), Ok(actual)) => {},
//                    (Err(expected), Err(actual)) if expected.kind() == actual.kind() => return Ok(()),
//                    _ => panic!("Expected: {expected:?}, actual: {actual:?}"),
//                };
//                // FIXME [NP]: Document that unwrap is safe because we just matched.
//                let expected = expected.unwrap();
//                let actual = actual.unwrap();
//
//                pretty_assertions::assert_eq!(expected.path(), actual.path());
//                // FIXME [NP]: Uncomment
//                //assert_eq_io_result(expected.metadata(), actual.metadata());
//                assert_eq_io_result(Ok(expected.file_type()), Ok(actual.file_type()));
//                // FIXME [NP]: Uncomment
//                //assert_eq_io_result(Ok(expected.file_name()), Ok(actual.file_name()));
//            }
//        }
//    }
//}
//
///// Returns a namespaced handle to the directory at the given path.
/////
///// Removes the directory if it already exists.
//pub(crate) fn create_or_empty_dir(path: impl AsRef<Path>) -> Fs {
//    let path = path.as_ref();
//    if path.exists() {
//        fs::remove_dir_all(path).expect("Failed to empty directory.");
//    }
//    fs::create_dir(path).expect("Failed to create directory.");
//    Fs::namespaced(path, NAMESPACE)
//}

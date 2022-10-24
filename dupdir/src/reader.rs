// ==================
// === FileRecord ===
// ==================

#[derive(Clone, Debug, Default)]
pub struct FileRecord {
    pub hash: String,
    pub path: String,
}

impl FileRecord {
    fn new(hash: String, path: String) -> Self {
        Self { hash, path }
    }
}



// =================
// === read_line ===
// =================

pub fn read_line(input: &str) -> FileRecord {
    let (hash, path) = input.split_once("  ").expect("Expected double space between hash and path");
    assert_eq!(hash, hash.trim(), "Extra whitespace in hash");
    assert_eq!(path, path.trim(), "Extra whitespace in path");
    FileRecord::new(hash.to_string(), path.to_string())
}

// ================
// === FileItem ===
// ================

pub struct FileItem {
    pub id: u32,
    pub path: String,
}



// =============
// === Files ===
// =============

pub type Files = im_rc::HashMap<u32, FileItem>;



// =================
// === FileModel ===
// =================

pub struct FileModel {
    pub files: Files,
}


// === Main `impl` ===

impl FileModel {
    /// Creates a new [`FileModel`].
    pub fn new() -> Self {
        let files = Files::new();
        Self { files }
    }

    pub fn files(&self) -> Vec<u32> {
        let files = self.files.iter();
        let files = files.map(|(key, _value)| *key);
        files.collect::<Vec<_>>()
    }
}

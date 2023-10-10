use crate::prelude::*;

use dupdir_hash::Hasher as _;



// ============
// === Main ===
// ============

pub fn main(
    state: &mut crate::State,
    path: impl AsRef<Path>,
) -> Vec<String> {
    let dir_hashes = dir_hashes_walk_dir_inner(state, path);
    let dir_hashes = dir_hashes.map(|(directory, hash)| {
        let hash = hex::encode(hash);
        format!("{h}  {d}", h = hash, d = directory)
    });
    dir_hashes.collect()
}

fn dir_hashes_walk_dir_inner(
    state: &mut crate::State,
    path: impl AsRef<Path>,
) -> impl Iterator<Item = (String, [u8; 8])> + '_ {
    let path = path.as_ref();
    let walkdir = walkdir::WalkDir::new(path);
    let entries = walkdir.into_iter();
    let mut cur_dir = None;
    let mut files_in_dir = BTreeMap::<_, BTreeSet<&str>>::new();
    entries.for_each(|e| {
        let e = e.expect("DirEntry");
        let file_type = e.file_type();
        if !file_type.is_file() {
            return
        }
        let path = e.into_path();

        let dir = path.parent().expect("Parent");
        let dir = dir.to_path_buf();

        match &cur_dir {
            Some(cur) if cur != &dir => cur_dir = Some(dir.clone()),
            None => cur_dir = Some(dir.clone()),
            Some(_) => {},
        }

        let h = state.hashes.get(&path).expect("File hash");
        for ancestor in dir.ancestors() {
            let ancestor = ancestor.to_str();
            let ancestor = ancestor.expect("Ancestor");
            let ancestor = ancestor.to_string();
            let hashes = files_in_dir.entry(ancestor);
            // Note that we store the files in a `BTreeSet` rather than incrementally hashing
            // because the order in which files appear in the directories (e.g., due to renaming)
            // shouldn't affect the hash.
            let hashes = hashes.or_insert_with(BTreeSet::new);
            hashes.insert(h);
        }
    });

    let dir_hashes = files_in_dir.into_iter();
    dir_hashes.map(|(d, hashes)| {
        let mut hasher = dupdir_hash::T1ha2::default();
        let hashes = hashes.into_iter();
        hashes.for_each(|h| {
            // FIXME [NP]: Is this correct? It'll register directories w/ different amounts of
            // copies of the same file as identical.
            dupdir_hash::copy_wide(&mut &h.as_bytes()[..], &mut hasher).unwrap();
        });
        let hash = hasher.finish();
        (d, hash)
    })
}

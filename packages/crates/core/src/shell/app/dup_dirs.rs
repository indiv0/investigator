use crate::prelude::*;

const UNIQUE_SEPARATOR: &str = ";";

// ===============
// === DupDirs ===
// ===============

#[derive(Clone, Debug)]
pub(crate) struct DupDirs<'a> {
    dir_hashes: &'a crate::Lines,
}

impl<'a> DupDirs<'a> {
    pub(crate) fn new(dir_hashes: &'a crate::Lines) -> Self {
        Self { dir_hashes }
    }

    pub fn dup_dirs(&self) -> Vec<String> {
        // Read the mapping of hash -> dir
        eprintln!("Reading (hash -> dir) mapping");
        let dir_hashes = self.read_dir_hashes();
        // FIXME [NP]: avoid this collect
        let dir_hashes = dir_hashes.collect::<Vec<_>>();

        // Convert the (hash -> dir) mapping to (hash -> dir1, dir2, ...)
        let mut map = HashMap::new();
        // FIXME [NP]: Make this progress
        //dir_hashes.into_iter().progress().for_each(|(h, d)| {
        dir_hashes.into_iter().for_each(|(h, d)| {
            map.entry(h).or_insert_with(Vec::new).push(d);
        });

        // Remove any directories with unique hashes.
        let (_unique, dup) = map
            .into_iter()
            // FIXME [NP]: Make this progress
            //.progress()
            .partition::<HashMap<_, _>, _>(|(_, ds)| ds.len() == 1);

        // Among the duplicate directories, sort them by the length of their path, shortest first.
        let dup = dup
            .into_iter()
            // FIXME [NP]: Make this progress
            //.progress()
            .map(|(h, mut ds)| {
                ds.sort_by_key(|d| d.len());
                (h, ds)
            })
            .collect::<HashMap<_, _>>();

        // If a directory is a subdirectory of another directory with the same hash, remove it.
        let dup = dup
            .into_iter()
            // FIXME [NP]: Make this progress
            //.progress()
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
            .collect::<HashMap<_, _>>();
        // If any categories now only contain one dir, remove them.
        let (_unique, dup) = dup
            .into_iter()
            // FIXME [NP]: Make this progress
            //.progress()
            .partition::<HashMap<_, _>, _>(|(_, ds)| ds.len() == 1);

        // Convert the map<hash, vec<dir>> mapping to vec<(hash, dir)>
        eprintln!("Convert map<hash, vec<dir>> to vec<(hash, dir)>");
        let mut dup_dirs = dup
            .into_iter()
            // FIXME [NP]: Make this progress
            //.progress()
            .flat_map(|(h, ds)| {
                let ds = ds.into_iter();
                ds.map(move |d| (h, d))
            })
            .collect::<Vec<_>>();

        // Sort the mapping by dir name.
        dup_dirs.sort_by_key(|(_, d)| *d);

        // Turn this into a list of strings.
        eprintln!("Convert vec<(hash, dir)> to vec<str>");
        let dup_dirs = dup_dirs
            .iter()
            // FIXME [NP]: make this par_iter + progress
            //.par_iter()
            //.progress()
            .inspect(|(h, d)| {
                assert!(!h.contains(UNIQUE_SEPARATOR));
                assert!(!d.contains(UNIQUE_SEPARATOR));
            })
            .map(|(h, d)| [*h, *d].join(UNIQUE_SEPARATOR))
            .collect::<Vec<_>>();
        dup_dirs
    }

    fn read_dir_hashes(&self) -> impl Iterator<Item = (&str, &str)> {
        let crate::Lines(lines) = self.dir_hashes;
        let lines = lines.iter();
        lines.map(|line| {
            let (hash, dir) = line.split_once("  ").expect("Failed to split line");
            crate::assert_path_rules(hash);
            crate::assert_path_rules(dir);
            (hash, dir)
        })
    }
}

#[cfg(test)]
use std::process;
use std::str;

// ================
// === Strategy ===
// ================

#[derive(Clone, Debug, Default)]
pub enum Strategy {
    #[cfg(test)]
    Unix,
    #[default]
    WalkDir,
}

// ==============
// === Finder ===
// ==============

#[derive(Clone, Debug, Default)]
pub struct Finder<'a> {
    path: &'a str,
    max_depth: Option<usize>,
    strategy: Strategy,
}

impl<'a> Finder<'a> {
    pub fn path(mut self, path: &'a str) -> Self {
        self.path = path;
        self
    }

    #[allow(dead_code)]
    pub fn max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = Some(max_depth);
        self
    }

    pub fn strategy(mut self, strategy: Strategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn find(&self) -> Vec<String> {
        match self.strategy {
            #[cfg(test)]
            Strategy::Unix => self.find_unix(),
            Strategy::WalkDir => self.find_walk_dir(),
        }
    }

    #[cfg(test)]
    fn find_unix(&self) -> Vec<String> {
        let mut args = vec![self.path, "-type", "f"];
        let max_depth = self.max_depth.map(|m| m.to_string());
        let max_depth = max_depth.as_ref();
        if let Some(max_depth) = max_depth {
            args.push("-maxdepth");
            args.push(max_depth);
        }

        let output = process::Command::new("find")
            .args(args)
            .output()
            .expect("Failed to execute process");
        assert!(output.status.success());
        assert!(output.stderr.is_empty());
        let stdout = output.stdout;
        let stdout = str::from_utf8(&stdout).expect("Failed to read stdout as utf8");
        let stdout = stdout.trim().to_string();
        let paths = stdout.lines();
        let paths = paths.inspect(|p| crate::assert_path_rules(p));
        let paths = paths.map(|p| p.trim().to_string());
        paths.collect()
    }

    fn find_walk_dir(&self) -> Vec<String> {
        let mut entries = walkdir::WalkDir::new(self.path);
        if let Some(max_depth) = self.max_depth {
            entries = entries.max_depth(max_depth);
        }
        let entries = entries.into_iter();
        let entries = entries.map(|e| e.expect("Failed to read entry"));
        let files = entries.filter(|e| e.file_type().is_file());
        let paths = files.map(|f| {
            let path = crate::path_to_str(f.path());
            crate::assert_path_rules(path);
            path.trim().to_string()
        });
        let paths = paths.map(|p| {
            assert!(!p.is_empty(), "Path is empty");
            p
        });
        paths.collect::<Vec<String>>()
    }
}

// ============
// === Main ===
// ============

pub fn main(path: &str) -> crate::Lines {
    let finder = Finder::default().path(path).strategy(Strategy::WalkDir);
    let paths = finder.find();
    crate::Lines(paths)
}

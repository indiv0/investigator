// =============
// === Error ===
// =============

type Error = Box<dyn std::error::Error>;



// ==================
// === FileRecord ===
// ==================

#[derive(Clone, Debug, Default)]
pub struct FileRecord {
    pub hash: String,
    pub path: String,
}



// ==============
// === Reader ===
// ==============

#[derive(Clone, Debug)]
pub struct Reader<I>
where
    I: Iterator<Item = String>,
{
    reader: I,
}

impl<I> Reader<I>
where
    I: Iterator<Item = String>,
{
    pub fn new(reader: I) -> Self {
        Self { reader }
    }

    pub fn read_record(&mut self, record: &mut FileRecord) -> Result<bool, Error> {
        if let Some(line) = self.reader.next() {
            let (hash, path) = line.split_once("  ").ok_or("invalid")?;
            assert_eq!(record.hash, record.hash.trim());
            assert_eq!(record.path, record.path.trim());
            record.hash.replace_range(.., hash);
            record.path.replace_range(.., path);
            return Ok(true);
        }

        Ok(false)
    }
}

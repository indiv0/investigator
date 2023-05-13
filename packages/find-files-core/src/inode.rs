use anyhow::Context as _;
use std::os::unix::prelude::FileTypeExt;
use std::str;
use std::os::unix::prelude::PermissionsExt as _;
use std::time;



// =================
// === Constants ===
// =================

/// Error message to use when an entry could not be retrieved.
const FAILED_TO_GET_ENTRY_ERROR: &str = "Failed to get entry.";
/// Error message to use when the type of an entry could not be determined.
const UNKNOWN_INODE_TYPE_ERROR: &str = "Unknown inode type";
/// Error message to use when a path is not valid UTF-8.
const PATH_IS_NOT_VALID_UTF8_ERROR: &str = "Path is not valid UTF-8.";

/// String representation of the [`InodeType::File`] variant.
///
/// [`InodeType::File`]: crate::InodeType::File
pub(crate) const FILE_INODE_TYPE: &str = "file";
/// String representation of the [`InodeType::Directory`] variant.
///
/// [`InodeType::Directory`]: crate::InodeType::Directory
pub(crate) const DIRECTORY_INODE_TYPE: &str = "directory";
/// String representation of the [`InodeType::SymbolicLink`] variant.
///
/// [`InodeType::SymbolicLink`]: crate::InodeType::SymbolicLink
pub(crate) const SYMBOLIC_LINK_INODE_TYPE: &str = "symbolic_link";
/// String representation of the [`InodeType::Socket`] variant.
///
/// [`InodeType::Socket`]: crate::InodeType::Socket
pub(crate) const SOCKET_INODE_TYPE: &str = "socket";
/// String representation of the [`InodeType::BlockDevice`] variant.
///
/// [`InodeType::BlockDevice`]: crate::InodeType::BlockDevice
pub(crate) const BLOCK_DEVICE_INODE_TYPE: &str = "block_device";
/// String representation of the [`InodeType::CharDevice`] variant.
///
/// [`InodeType::CharDevice`]: crate::InodeType::CharDevice
pub(crate) const CHAR_DEVICE_INODE_TYPE: &str = "char_device";
/// String representation of the [`InodeType::Fifo`] variant.
///
/// [`InodeType::Fifo`]: crate::InodeType::Fifo
pub(crate) const FIFO_INODE_TYPE: &str = "fifo";



// =================
// === InodeType ===
// =================

/// The type of an [`Inode`].
///
/// [`Inode`]: crate::Inode
#[derive(Clone, Copy, Debug)]
pub(crate) enum InodeType {
    /// A file.
    File,
    /// A directory.
    Directory,
    /// A symbolic link.
    SymbolicLink,
    /// A socket.
    Socket,
    /// A block device.
    BlockDevice,
    /// A character device.
    CharDevice,
    /// A FIFO.
    Fifo,
}


// === Trait `impl`s ===

impl rusqlite::ToSql for InodeType {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let inode_type = match self {
            InodeType::File => FILE_INODE_TYPE,
            InodeType::Directory => DIRECTORY_INODE_TYPE,
            InodeType::SymbolicLink => SYMBOLIC_LINK_INODE_TYPE,
            InodeType::Socket => SOCKET_INODE_TYPE,
            InodeType::BlockDevice => BLOCK_DEVICE_INODE_TYPE,
            InodeType::CharDevice => CHAR_DEVICE_INODE_TYPE,
            InodeType::Fifo => FIFO_INODE_TYPE,
        };
        let to_sql_output = inode_type.to_sql()?;
        Ok(to_sql_output)
    }
}

impl TryFrom<&str> for InodeType {
    type Error = anyhow::Error;

    fn try_from(inode_type: &str) -> anyhow::Result<Self> {
        let inode_type = match inode_type {
            FILE_INODE_TYPE => InodeType::File,
            DIRECTORY_INODE_TYPE => InodeType::Directory,
            SYMBOLIC_LINK_INODE_TYPE => InodeType::SymbolicLink,
            SOCKET_INODE_TYPE => InodeType::Socket,
            BLOCK_DEVICE_INODE_TYPE => InodeType::BlockDevice,
            CHAR_DEVICE_INODE_TYPE => InodeType::CharDevice,
            FIFO_INODE_TYPE => InodeType::Fifo,
            _ => anyhow::bail!(UNKNOWN_INODE_TYPE_ERROR),
        };
        Ok(inode_type)
    }
}

impl rusqlite::types::FromSql for InodeType {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let value = value.as_str()?;
        let inode_type = match Self::try_from(value) {
            Ok(inode_type) => inode_type,
            _ => return Err(rusqlite::types::FromSqlError::InvalidType),
        };
        Ok(inode_type)
    }
}



// =============
// === Inode ===
// =============

/// A file or directory.
#[derive(Debug)]
pub(crate) struct Inode {
    /// Path to this [`Inode`].
    ///
    /// [`Inode`]: crate::Inode
    pub path: String,
    /// The file extension of this [`Inode`].
    ///
    /// * [`None`], if there is no file name;
    /// * [`None`], if there is no embedded `.`;
    /// * [`None`], if the file name begins with `.` and has no other `.`s within;
    /// * Otherwise, the portion of the file name after the final `.`
    ///
    /// [`Inode`]: crate::Inode
    /// [`None`]: std::option::Option::None
    pub file_extension: Option<String>,
    /// The [`InodeType`] of this [`Inode`].
    ///
    /// [`InodeType`]: crate::InodeType
    /// [`Inode`]: crate::Inode
    pub inode_type: InodeType,
    /// The depth of this [`Inode`] in the search directory.
    ///
    /// [`Inode`]: crate::Inode
    pub depth: usize,
    /// Size in bytes of the file this [`Inode`] is for.
    ///
    /// [`Inode`]: crate::Inode
    pub size: u64,
    /// Permissions for this [`Inode`].
    ///
    /// [`Inode`]: crate::Inode
    pub permissions: u32,
    /// The last time this [`Inode`] was modified.
    ///
    /// [`Inode`]: crate::Inode
    pub modified: i128,
    /// The last time this [`Inode`] was accessed.
    pub accessed: i128,
    /// The time when this [`Inode`] was created.
    ///
    /// [`Inode`]: crate::Inode
    pub created: i128,
    /// The file name of this [`Inode`].
    ///
    /// [`Inode`]: crate::Inode
    pub file_name: String,
    /// The file name of this [`Inode`] without the file extension.
    ///
    /// [`Inode`]: crate::Inode
    pub file_stem: Option<String>,
}


// === Main `impl` ===

impl Inode {
    /// Creates a new list of [`Inode`]s by finding all files and directories in the search
    /// directory and all subdirectories.
    ///
    /// [`Inode`]: crate::Inode
    pub(crate) fn from_search_directory(search_directory: &str) -> anyhow::Result<Vec<Self>> {
        println!("Searching directory: \"{search_directory}\".");
        let from_search_directory = || {
            let walkdir = walkdir::WalkDir::new(search_directory);
            let mut inodes = Vec::new();
            for dir_entry in walkdir {
                let dir_entry = dir_entry.context(FAILED_TO_GET_ENTRY_ERROR)?;
                let inode = Self::from_dir_entry(dir_entry)?;
                inodes.push(inode);
            }
            Ok::<_, anyhow::Error>(inodes)
        };
        let (elapsed, inodes) = crate::with_timer(from_search_directory);
        let inodes = inodes?;
        let count = inodes.len();
        println!("Found {count} files in {elapsed} seconds.");
        Ok(inodes)
    }
}


// === Internal `impl` ===

impl Inode {
    /// Creates a new [`Inode`] from a [`DirEntry`].
    ///
    /// [`Inode`]: crate::Inode
    /// [`DirEntry`]: walkdir::DirEntry
    fn from_dir_entry(dir_entry: walkdir::DirEntry) -> anyhow::Result<Self> {
        let path = path_from_dir_entry(&dir_entry)?;
        let file_extension = file_extension_from_dir_entry(&dir_entry)?;
        let inode_type = inode_type_from_dir_entry(&dir_entry)?;
        let depth = dir_entry.depth();
        let metadata = dir_entry.metadata()?;
        let size = metadata.len();
        let permissions = metadata.permissions();
        let permissions = permissions.mode();
        let modified = metadata.modified()?;
        let modified = time_as_i128_nanos(modified)?;
        let accessed = metadata.accessed()?;
        let accessed = time_as_i128_nanos(accessed)?;
        let created = metadata.created()?;
        let created = time_as_i128_nanos(created)?;
        let file_name = file_name_from_dir_entry(&dir_entry)?;
        let file_stem = file_stem_from_dir_entry(&dir_entry)?;
        let inode = Inode {
            path,
            file_extension,
            inode_type,
            depth,
            size,
            permissions,
            modified,
            accessed,
            created,
            file_name,
            file_stem,
        };
        Ok(inode)
    }
}

/// Converts a [`time::SystemTime`] into [`i128`] nanoseconds.
///
/// [`time::SystemTime`]: std::time::SystemTime
/// [`i128`]: std::primitive::i128
fn time_as_i128_nanos(time: time::SystemTime) -> anyhow::Result<i128> {
    let duration = time.duration_since(time::UNIX_EPOCH)?;
    let nanos = duration.as_nanos();
    let nanos = nanos.try_into()?;
    Ok(nanos)
}

/// Gets the [`Path`] as a UTF-8 [`String`] from a [`DirEntry`].
///
/// [`Path`]: std::path::Path
/// [`String`]: std::string::String
/// [`DirEntry`]: walkdir::DirEntry
fn path_from_dir_entry(dir_entry: &walkdir::DirEntry) -> anyhow::Result<String> {
    let path = dir_entry.path();
    let path = path.to_str();
    let path = path.context(PATH_IS_NOT_VALID_UTF8_ERROR)?;
    let path = path.to_string();
    Ok(path)
}

/// Gets the file extension as a UTF-8 [`String`] from a [`DirEntry`].
///
/// [`String`]: std::string::String
/// [`DirEntry`]: walkdir::DirEntry
fn file_extension_from_dir_entry(dir_entry: &walkdir::DirEntry) -> anyhow::Result<Option<String>> {
    let path = dir_entry.path();
    let file_extension = match path.extension() {
        Some(file_extension) => {
            let file_extension = file_extension.to_str();
            let file_extension = file_extension.context(PATH_IS_NOT_VALID_UTF8_ERROR)?;
            let file_extension = file_extension.to_string();
            Some(file_extension)
        },
        None => None,
    };
    Ok(file_extension)
}

/// Gets the file name as a UTF-8 [`String`] from a [`DirEntry`].
///
/// [`String`]: std::string::String
/// [`DirEntry`]: walkdir::DirEntry
fn file_name_from_dir_entry(dir_entry: &walkdir::DirEntry) -> anyhow::Result<String> {
    let path = dir_entry.path();
    let file_name = path.file_name();
    let file_name = file_name.expect("File name terminated with \"..\".");
    let file_name = file_name.to_str();
    let file_name = file_name.context(PATH_IS_NOT_VALID_UTF8_ERROR)?;
    let file_name = file_name.to_string();
    Ok(file_name)
}

/// Gets the file stem as a UTF-8 [`String`] from a [`DirEntry`].
///
/// [`String`]: std::string::String
/// [`DirEntry`]: walkdir::DirEntry
fn file_stem_from_dir_entry(dir_entry: &walkdir::DirEntry) -> anyhow::Result<Option<String>> {
    let path = dir_entry.path();
    let file_stem = match path.file_stem() {
        Some(file_stem) => {
            let file_stem = file_stem.to_str();
            let file_stem = file_stem.context(PATH_IS_NOT_VALID_UTF8_ERROR)?;
            let file_stem = file_stem.to_string();
            Some(file_stem)
        },
        None => panic!("Expected file stem."),
    };
    Ok(file_stem)
}

/// Gets the [`InodeType`] from a [`DirEntry`].
///
/// [`InodeType`]: crate::InodeType
/// [`DirEntry`]: walkdir::DirEntry
fn inode_type_from_dir_entry(dir_entry: &walkdir::DirEntry) -> anyhow::Result<InodeType> {
    let file_type = dir_entry.file_type();
    let inode_type = file_type.is_file().then(|| InodeType::File);
    let inode_type = inode_type.or_else(|| file_type.is_dir().then(|| InodeType::Directory));
    let inode_type = inode_type.or_else(|| file_type.is_symlink().then(|| InodeType::SymbolicLink));
    let inode_type = inode_type.or_else(|| file_type.is_socket().then(|| InodeType::Socket));
    let inode_type = inode_type.or_else(|| file_type.is_char_device().then(|| InodeType::CharDevice));
    let inode_type = inode_type.or_else(|| file_type.is_block_device().then(|| InodeType::BlockDevice));
    let inode_type = inode_type.or_else(|| file_type.is_fifo().then(|| InodeType::Fifo));
    let inode_type = inode_type.ok_or_else(|| format!("{UNKNOWN_INODE_TYPE_ERROR}: \"{file_type:?}\"."));
    let inode_type = inode_type.map_err(anyhow::Error::msg);
    let path = path_from_dir_entry(dir_entry)?;
    let inode_type = inode_type.context(path)?;
    Ok(inode_type)
}

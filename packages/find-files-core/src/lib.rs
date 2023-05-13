use anyhow::Context as _;
use std::env;
use std::iter;
use std::io;
use std::str;
use std::time;

use crate::html::write_html;



// ===============
// === Exports ===
// ===============

mod html;
mod inode;
//pub mod sql;
pub mod sql2;
pub use sql2 as sql;



// =================
// === Constants ===
// =================

/// Error message to use when the search directory argument is missing.
const MISSING_SEARCH_DIRECTORY_ARGUMENT_ERROR: &str = "Missing search directory argument.";
/// Error message to use when a line of text could not be read from [`Stdin`].
///
/// [`Stdin`]: std::io::Stdin
const FAILED_TO_READ_LINE_ERROR: &str = "Failed to read line from stdin.";
/// Error message to use when a command could not be read from [`Stdin`].
///
/// [`Stdin`]: std::io::Stdin
const FAILED_TO_READ_COMMAND_ERROR: &str = "Failed to read command from stdin.";
/// Error message to use when a command is not recognized.
const UNKNOWN_COMMAND_ERROR: &str = "Unknown command";

/// String for the newline character.
const NEW_LINE: &str = "\n";
/// Separator for multi-value terms in the search query.
const LIST_SEPARATOR: &str = ",";
/// String representing the [`FindAll`] [`Command`].
///
/// [`FindAll`]: crate::Command::FindAll
/// [`Command`]: crate::Command
const FIND_ALL_COMMAND: &str = "find_all";
/// String representing the [`FindByFileExtensions`] [`Command`].
///
/// [`FindByFileExtensions`]: crate::Command::FindByFileExtensions
/// [`Command`]: crate::Command
const FIND_BY_EXT_COMMAND: &str = "find_by_ext";
/// String representing the [`FindByDepth`] [`Command`].
///
/// [`FindByDepth`]: crate::Command::FindByDepth
/// [`Command`]: crate::Command
const FIND_BY_DEPTH_COMMAND: &str = "find_by_depth";
/// String representing the [`FindByInodeType`] [`Command`].
///
/// [`FindByInodeType`]: crate::Command::FindByInodeType
/// [`Command`]: crate::Command
const FIND_BY_INODE_TYPE_COMMAND: &str = "find_by_inode_type";
/// String representing the [`Exit`] [`Command`].
///
/// [`Exit`]: crate::Command::Exit
/// [`Command`]: crate::Command
const EXIT_COMMAND: &str = "exit";

/// Name of the file used to store the database.
const DATABASE_FILE: &str = "sqlite:find-files.db";



// ============
// === Args ===
// ============

/// Arguments passed to the program.
#[derive(Debug)]
pub struct Args {
    /// The directory to search in for files and directories.
    pub search_directory: String,
    /// Name of the database file to use. If [`None`], then an in-memory database is used.
    ///
    /// [`None`]: std::option::Option::None
    pub database_name: Option<String>,
}


// === Main `impl` ===

impl Args {
    /// Creates a new [`Args`] from the environment.
    pub fn from_env() -> anyhow::Result<Self> {
        let args = env::args();
        Args::parse(args)
    }

    /// Parses the [`Args`] passed to the program.
    ///
    /// [`Args`]: std::env::Args
    fn parse(mut args: env::Args) -> anyhow::Result<Self> {
        // Skip the name of the program itself.
        let _program_name = args.next();
        let search_directory = args.next();
        let search_directory = search_directory.ok_or_else(|| anyhow::Error::msg(MISSING_SEARCH_DIRECTORY_ARGUMENT_ERROR))?;
        let database_name = DATABASE_FILE.to_string();
        let database_name = Some(database_name);
        Ok(Args { search_directory, database_name })
    }
}



// ===========
// === Run ===
// ===========

//pub fn run() -> anyhow::Result<()> {
//    let args = Args::from_env()?;
//    let mut database = sql::Database::new(DATABASE_FILE).await?;
//
//    find_inodes(&mut database, &args.search_directory)?;
//
//    loop {
//        println!("Enter a command ({FIND_ALL_COMMAND} {FIND_BY_EXT_COMMAND}, {FIND_BY_DEPTH_COMMAND}, {FIND_BY_INODE_TYPE_COMMAND}, {EXIT_COMMAND}):");
//        let input = read_line()?;
//        let mut parser = CommandParser::from_str(&input)?;
//        let command = parser.parse_command()?;
//        let inodes = match command {
//            //Command::FindAll => {
//            //    let find_inodes = || find_all(&mut database);
//            //    let (elapsed, inodes) = with_timer(find_inodes);
//            //    let inodes = inodes?;
//            //    let count = inodes.len();
//
//            //    let paths = {
//            //        let inodes = inodes.iter();
//            //        let strings = inodes.map(|inode| {
//            //            let inode::Inode {
//            //                path,
//            //                file_extension,
//            //                inode_type,
//            //                depth,
//            //                size,
//            //                permissions,
//            //                modified,
//            //                accessed,
//            //                created,
//            //                file_name,
//            //                file_stem,
//            //            } = inode;
//            //            format!("{path} ({file_extension:?}, {inode_type:?}, {depth}, {size}, {permissions:?}, {modified:?}, {accessed:?}, {created:?}, {file_name}, {file_stem:?})")
//            //        });
//            //        let strings = strings.collect::<Vec<String>>();
//            //        strings.join(NEW_LINE)
//            //    };
//
//            //    println!("{paths}\nFound {count} files in {elapsed} seconds.");
//
//            //    inodes
//            //}
//            //Command::FindByFileExtensions { file_extensions } => {
//            //    let find_inodes = || find_inodes_by_file_extensions(&mut database, &file_extensions);
//            //    let (elapsed, inodes) = with_timer(find_inodes);
//            //    let inodes = inodes?;
//            //    let count = inodes.len();
//
//            //    let paths = {
//            //        let inodes = inodes.iter();
//            //        let paths = inodes.map(|inode| inode.path.as_str());
//            //        let paths = paths.collect::<Vec<&str>>();
//            //        paths.join(NEW_LINE)
//            //    };
//
//            //    println!("{paths}\nFound {count} files in {elapsed} seconds with the extensions {file_extensions:?}.");
//
//            //    inodes
//            //},
//            //Command::FindByDepth { depth } => {
//            //    let find_inodes = || find_inodes_by_depth(&mut database, depth);
//            //    let (elapsed, inodes) = with_timer(find_inodes);
//            //    let inodes = inodes?;
//            //    let count = inodes.len();
//
//            //    let paths = {
//            //        let inodes = inodes.iter();
//            //        let paths = inodes.map(|inode| inode.path.as_str());
//            //        let paths = paths.collect::<Vec<&str>>();
//            //        paths.join(NEW_LINE)
//            //    };
//
//            //    println!("{paths}\nFound {count} files in {elapsed} seconds with depth \"{depth:?}\".");
//
//            //    inodes
//            //},
//            //Command::FindByInodeType { inode_type } => {
//            //    let find_inodes = || find_inodes_by_type(&mut database, inode_type);
//            //    let (elapsed, inodes) = with_timer(find_inodes);
//            //    let inodes = inodes?;
//            //    let count = inodes.len();
//
//            //    let paths = {
//            //        let inodes = inodes.iter();
//            //        let paths = inodes.map(|inode| inode.path.as_str());
//            //        let paths = paths.collect::<Vec<&str>>();
//            //        paths.join(NEW_LINE)
//            //    };
//
//            //    println!("{paths}\nFound {count} files in {elapsed} seconds with type \"{inode_type:?}\".");
//
//            //    inodes
//            //},
//            Command::Exit => break,
//        };
//
//        let html = html::html(inodes)?;
//        write_html(&html)?;
//    }
//    Ok(())
//}

/// Sets up the database by performing the initial search.
pub async fn find_inodes(database: &sql::Database, search_directory: &str) -> anyhow::Result<()> {
    // FIXME [NP]: re-add this optimization.
    //if database.inodes().count()? > 0 {
    //    return Ok(());
    //}

    let inodes = inode::Inode::from_search_directory(search_directory)?;
    database.insert_inodes(inodes).await?;
    Ok(())
}

///// Finds all [`Inode`]s.
//pub fn find_all(database: &mut sql::Database) -> anyhow::Result<Vec<inode::Inode>> {
//    let query = database.inodes();
//    let query = query.select();
//    let inodes = query.all()?;
//    Ok(inodes)
//}

///// Find all [`Inode`]s that match the given `depth`.
/////
///// [`Inode`]: crate::inode::Inode
//// FIXME [NP]: Find a way to do this in one query.
//fn find_inodes_by_depth(database: &mut sql::Database, depth: usize) -> anyhow::Result<Vec<inode::Inode>> {
//    let query = database.inodes();
//    let query = query.select();
//    let inodes = query.equals(sql::schema::inodes::Field::Depth, &depth)?;
//    Ok(inodes)
//}

///// Find all [`Inode`]s that match the given [`InodeType`].
/////
///// [`Inode`]: crate::inode::Inode
///// [`InodeType`]: crate::inode::InodeType
//// FIXME [NP]: Find a way to do this in one query.
//fn find_inodes_by_type(database: &mut sql::Database, inode_type: inode::InodeType) -> anyhow::Result<Vec<inode::Inode>> {
//    let query = database.inodes();
//    let query = query.select();
//    let inodes = query.equals(sql::schema::inodes::Field::InodeType, &inode_type)?;
//    Ok(inodes)
//}
//
///// Find all [`Inode`]s that match the given `file_extension`.
/////
///// [`Inode`]: crate::inode::Inode
//// FIXME [NP]: Find a way to do this in one query.
//fn find_inodes_by_file_extensions(database: &mut sql::Database, file_extensions: &Vec<String>) -> anyhow::Result<Vec<inode::Inode>> {
//    let mut all_inodes = Vec::new();
//    for file_extension in file_extensions {
//        let query = database.inodes();
//        let query = query.select();
//        let inodes = query.r#match(sql::schema::inodes::Field::FileExtension, file_extension)?;
//        all_inodes.extend(inodes);
//    }
//    // FIXME [NP]: This is a very expensive way to sort, find a way to do so in SQL or with slices.
//    all_inodes.sort_by_key(|inode| inode.path.clone());
//    Ok(all_inodes)
//}



// ==================
// === With Timer ===
// ==================

/// Executes the provided function and returns the elapsed time in seconds and the result of the
/// function.
fn with_timer<T>(mut f: impl FnMut() -> T) -> (f64, T) {
    let now = time::Instant::now();
    let result = f();
    let elapsed = now.elapsed();
    let elapsed = elapsed.as_secs_f64();
    (elapsed, result)
}



// ===============
// === Command ===
// ===============

/// A command to execute against the [`Database`].
///
/// [`Database`]: crate::sql::Database
#[derive(Debug)]
enum Command {
    /// Finds all [`Inode`]s in the given directory.
    ///
    /// [`Inode`]: crate::Inode
    FindAll,
    /// Finds all [`Inode`]s with the given file extension.
    ///
    /// [`Inode`]: crate::Inode
    FindByFileExtensions {
        file_extensions: Vec<String>,
    },
    /// Finds all [`Inode`]s at the given depth.
    ///
    /// [`Inode`]: crate::Inode
    FindByDepth {
        depth: usize,
    },
    /// Finds all [`Inode`]s with the given [`InodeType`].
    ///
    /// [`Inode`]: crate::Inode
    /// [`InodeType`]: crate::inode::InodeType
    FindByInodeType {
        inode_type: inode::InodeType,
    },
    /// Exits the program.
    Exit,
}



// =====================
// === CommandParser ===
// =====================

/// A utility to parse [`Command`]s from input read from [`Stdin`].
///
/// [`Command`]: crate::Command
/// [`Stdin`]: std::io::Stdin
#[derive(Debug)]
struct CommandParser<'a> {
    /// A reader over a line of input, trimmed and split by ASCII whitespace.
    reader: iter::Peekable<str::SplitAsciiWhitespace<'a>>,
}


// === Main `impl` ===

impl<'a> CommandParser<'a> {
    /// Creates a new [`CommandParser`] from a line of input read from [`Stdin`].
    ///
    /// [`CommandParser`]: crate::CommandParser
    /// [`Stdin`]: std::io::Stdin
    fn from_str(input: &'a str) -> anyhow::Result<CommandParser<'a>> {
        let input = input.trim();
        let reader = input.split_ascii_whitespace();
        let reader = reader.peekable();
        Ok(Self { reader })
    }

    /// Reads and parses the next [`Command`] from [`Stdin`].
    ///
    /// [`Command`]: crate::Command
    /// [`Stdin`]: std::io::Stdin
    fn parse_command(&mut self) -> anyhow::Result<Command> {
        let command = self.read_next_word()?;
        match command {
            FIND_ALL_COMMAND => Ok(Command::FindAll),
            FIND_BY_EXT_COMMAND => self.parse_find_by_file_extension_command(),
            FIND_BY_DEPTH_COMMAND => self.parse_find_by_depth_command(),
            FIND_BY_INODE_TYPE_COMMAND => self.parse_find_by_inode_type_command(),
            EXIT_COMMAND => Ok(Command::Exit),
            command => Err(anyhow::Error::msg(format!("{UNKNOWN_COMMAND_ERROR}: \"{command}\"."))),
        }
    }
}


// === Internal `impl` ===

impl CommandParser<'_> {
    /// Reads the next ASCII-whitespace delimited string from the [`CommandParser`].
    ///
    /// [`CommandParser`]: crate::CommandParser
    fn read_next_word(&mut self) -> anyhow::Result<&str> {
        let word = self.reader.next();
        word.ok_or_else(|| anyhow::Error::msg(FAILED_TO_READ_COMMAND_ERROR))
    }

    /// Peeks the next ASCII-whitespace delimited string from the [`CommandParser`].
    ///
    /// [`CommandParser`]: crate::CommandParser
    #[allow(dead_code)]
    fn peek_next_word(&mut self) -> anyhow::Result<&str> {
        let word = self.reader.peek();
        let word = word.map(|word| *word);
        word.ok_or_else(|| anyhow::Error::msg(FAILED_TO_READ_COMMAND_ERROR))
    }

    /// Reads and parses the arguments for the [`FindByFileExtensions`] [`Command`].
    ///
    /// [`FindByFileExtensions`]: crate::Command::FindByFileExtensions
    /// [`Command`]: crate::Command
    fn parse_find_by_file_extension_command(&mut self) -> anyhow::Result<Command> {
        let file_extensions = self.read_next_word()?;
        let file_extensions = file_extensions.split(LIST_SEPARATOR);
        let file_extensions = file_extensions.map(|file_extension| file_extension.to_string());
        let file_extensions = file_extensions.collect::<Vec<_>>();
        let command = Command::FindByFileExtensions { file_extensions };
        Ok(command)
    }

    /// Reads and parses the arguments for the [`FindByDepth`] [`Command`].
    ///
    /// [`FindByDepth`]: crate::Command::FindByDepth
    /// [`Command`]: crate::Command
    fn parse_find_by_depth_command(&mut self) -> anyhow::Result<Command> {
        let depth= self.read_next_word()?;
        let (depth, remainder) = parse_usize(depth)?;
        assert!(remainder.is_empty(), "Failed to parse depth.");
        let command = Command::FindByDepth { depth };
        Ok(command)
    }

    /// Reads and parses the arguments for the [`FindByInodeType`] [`Command`].
    ///
    /// [`FindByInodeType`]: crate::Command::FindByInodeType
    /// [`Command`]: crate::Command
    fn parse_find_by_inode_type_command(&mut self) -> anyhow::Result<Command> {
        let inode_type = self.read_next_word()?;
        let (inode_type, remainder) = parse_inode_type(inode_type)?;
        assert!(remainder.is_empty(), "Failed to parse Inode type.");
        let command = Command::FindByInodeType { inode_type };
        Ok(command)
    }
}

/// Reads a line of text from [`Stdin`].
///
/// [`Stdin`]: std::io::Stdin
fn read_line() -> anyhow::Result<String> {
    let mut command = String::new();
    let stdin = io::stdin();
    let result = stdin.read_line(&mut command);
    let _ = result.context(FAILED_TO_READ_LINE_ERROR)?;
    Ok(command)
}

/// Parses an [`InodeType`] from the given `str`.
///
/// [`InodeType`]: crate::inode::InodeType
fn parse_inode_type(string: &str) -> anyhow::Result<(inode::InodeType, &str)> {
    let predicate = |character: &str| character == " ";
    let (matched, remainder) = take_until(string, predicate)?;
    let matched = inode::InodeType::try_from(matched)?;
    Ok((matched, remainder))
}

/// Parses a `usize` from the given `str`.
fn parse_usize(string: &str) -> anyhow::Result<(usize, &str)> {
    let predicate = |character| is_digit(character);
    let (digits, remainder) = take_while(string, predicate)?;
    let number = digits.parse::<usize>()?;
    Ok((number, remainder))
}

/// Returns the largest input slice that doesn't match the predicate.
fn take_until<'a, P>(string: &'a str, predicate: P) -> anyhow::Result<(&str, &str)>
where
    P: Fn(&'a str) -> bool,
{
    let predicate = |character| !predicate(character);
    take_while(string, predicate)
}

/// Returns the largest input slice that matches the predicate.
fn take_while<'a, P>(string: &'a str, predicate: P) -> anyhow::Result<(&str, &str)>
where
    P: Fn(&'a str) -> bool,
{
    if string.is_empty() {
        return Ok(("", ""));
    }

    let mut length = 0;
    while length < string.len() {
        if predicate(&string[length..length + 1]) {
            length += 1;
        } else {
            break;
        }
    }

    let (taken, remainder) = string.split_at(length);
    Ok((taken, remainder))
}

/// Returns `true` if the given `char` is an ASCII digit.
fn is_digit(character: &str) -> bool {
    assert_eq!(character.len(), 1, "Character must be a single ASCII character.");
    let mut chars = character.chars();
    let character = chars.next().expect("Character was somehow empty.");
    match character {
        '0'..='9' => true,
        _ => false,
    }
}

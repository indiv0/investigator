use std::iter;



// =================
// === Constants ===
// =================

/// Constant to use when there are no SQL parameters for [`Connection::execute`].
///
/// [`Connection::execute`]: rusqlite::Connection::execute
const NO_SQL_PARAMS: &[&dyn rusqlite::ToSql] = &[];
/// Character used as the placeholder for a parameter in a SQL query.
const PLACEHOLDER: &str = "?";



// ===============
// === Exports ===
// ===============

pub(super) mod schema;



// =============
// === Count ===
// =============

/// Executes a query to count the number of rows in a table.
fn count(connection: &rusqlite::Connection, table: &str) -> rusqlite::Result<usize> {
    let sql = format!("SELECT COUNT(*) FROM {table}");
    let mapper = |row: &rusqlite::Row| row.get(0);
    connection.query_row(&sql, crate::sql::NO_SQL_PARAMS, mapper)
}



// ==============
// === Insert ===
// ==============

/// Executes a query to insert an item into a table.
fn insert<'a>(connection: &'a rusqlite::Connection, table: &str, fields: &[&str]) -> rusqlite::Result<rusqlite::Statement<'a>> {
    let count = fields.len();
    let fields = fields.join(", ");
    let placeholders = iter::repeat(PLACEHOLDER);
    let placeholders = placeholders.take(count);
    let placeholders = placeholders.collect::<Vec<_>>();
    let placeholders = placeholders.join(", ");
    let sql = format!("INSERT INTO {table} ({fields}) VALUES ({placeholders})");
    connection.prepare(&sql)
}



// ==============
// === Select ===
// ==============

/// A builder for a query that selects all fields from a table.
#[derive(Debug)]
struct Select<'a, F> {
    connection: &'a rusqlite::Connection,
    query: String,
    mapper: F,
}


// === Internal `impl` ===

impl<'a, T, F> Select<'a, F>
where
    F: Fn(&rusqlite::Row) -> rusqlite::Result<T>,
{
    /// Returns a builder for a query that selects all fields from a table.
    fn new(connection: &'a rusqlite::Connection, table: &str, fields: &[&str], mapper: F) -> Self {
        let fields = fields.join(", ");
        let query = format!("SELECT {fields} FROM {table}");
        Self { connection, query, mapper }
    }

    /// Adds a `WHERE x MATCH y` clause to the query.
    fn r#match(&self, field: &str, value: &str) -> rusqlite::Result<Vec<T>> {
        let query = &self.query;
        let query = format!("{query} WHERE {field} MATCH {PLACEHOLDER}1");
        let params = [value];
        let mut statement = self.connection.prepare(&query)?;
        let query = statement.expanded_sql();
        println!("Executing query \"{query:?}\" with params {params:?}");
        let rows = statement.query_map(params, &self.mapper)?;
        rows.collect::<Result<_, _>>()
    }

    /// Adds a `WHERE x = y` clause to the query.
    fn equals(&self, field: &str, value: &dyn rusqlite::ToSql) -> rusqlite::Result<Vec<T>> {
        let query = &self.query;
        let query = format!("{query} WHERE {field} = {PLACEHOLDER}1");
        let params = [value];
        let mut statement = self.connection.prepare(&query)?;
        let query = statement.expanded_sql();
        println!("Executing query \"{query:?}\".");
        let rows = statement.query_map(params, &self.mapper)?;
        rows.collect::<Result<_, _>>()
    }

    /// Executes this query without any further clauses.
    fn all(&self) -> rusqlite::Result<Vec<T>> {
        let query = &self.query;
        let mut statement = self.connection.prepare(&query)?;
        let query = statement.expanded_sql();
        println!("Executing query \"{query:?}\".");
        let rows = statement.query_map(crate::sql::NO_SQL_PARAMS, &self.mapper)?;
        rows.collect::<Result<_, _>>()
    }
}



// ================
// === Database ===
// ================

/// A database of files and directories.
#[derive(Debug)]
pub struct Database {
    connection: rusqlite::Connection,
}


// === Main `impl` ===

impl Database {
    /// Creates a new [`Database`].
    ///
    /// If the `database_name` is [`None`], an in-memory database is created.
    ///
    /// Tables will be created on the database, if necessary.
    ///
    /// [`Database`]: crate::sql::Database
    /// [`None`]: std::option::Option::None
    pub fn new(database_name: Option<String>) -> anyhow::Result<Self> {
        let connection = open(database_name)?;
        schema::inodes::create(&connection)?;
        Ok(Self { connection })
    }

    /// Returns a guard for operations on the [`inodes::TABLE`].
    ///
    /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
    pub(crate) fn inodes(&mut self) -> schema::inodes::Inodes<'_> {
        let connection = &mut self.connection;
        schema::inodes::Inodes { connection }
    }
}



// ========================
// === With Transaction ===
// ========================

/// Performs an operation in a [`Transaction`] context, committing the [`Transaction`] at the end.
///
/// [`Transaction`]: rusqlite::Transaction
fn with_transaction(connection: &mut rusqlite::Connection, f: impl FnOnce(&rusqlite::Transaction) -> rusqlite::Result<()>) -> rusqlite::Result<()> {
    let transaction = connection.transaction()?;
    f(&transaction)?;
    transaction.commit()?;
    Ok(())
}



// ============
// === Open ===
// ============

/// Opens a [`Connection`] to a database.
///
/// If `database_name` is provided, the database is opened from the filesystem. Otherwise, an
/// in-memory database is created.
///
/// [`Connection`]: rusqlite::Connection
fn open(database_name: Option<String>) -> anyhow::Result<rusqlite::Connection> {
    let connection = match database_name {
        Some(database_name) => rusqlite::Connection::open(database_name)?,
        None => rusqlite::Connection::open_in_memory()?,
    };
    Ok(connection)
}

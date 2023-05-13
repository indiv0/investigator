// ==============
// === Inodes ===
// ==============

pub(crate) mod inodes {
    // =================
    // === Constants ===
    // =================

    /// Name of the [`inodes`] table.
    ///
    /// [`inodes`]: crate::sql::schema::inodes
    const TABLE: &str = "inodes";
    /// Number of fields in the [`inodes::TABLE`].
    ///
    /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
    const FIELD_COUNT: usize = 11;
    /// List of the fields in the [`inodes::TABLE`].
    ///
    /// [`inodes`]: crate::sql::schema::inodes
    const FIELDS: [&str; FIELD_COUNT] = [
        fields::PATH,
        fields::FILE_EXTENSION,
        fields::INODE_TYPE,
        fields::DEPTH,
        fields::SIZE,
        fields::PERMISSIONS,
        fields::MODIFIED,
        fields::ACCESSED,
        fields::CREATED,
        fields::FILE_NAME,
        fields::FILE_STEM,
    ];



    // ===========
    // === Sql ===
    // ===========

    pub(super) mod sql {
        // =================
        // === Constants ===
        // =================

        /// Returns a query to create the [`inodes::TABLE`].
        ///
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        pub(super) fn create() -> String {
            let table = crate::sql::schema::inodes::TABLE;
            let fields = crate::sql::schema::inodes::FIELDS.join(", ");
            format!("CREATE VIRTUAL TABLE IF NOT EXISTS {table} USING fts5({fields})")
        }
    }



    // ==============
    // === Fields ===
    // ==============

    pub(super) mod fields {
        // =================
        // === Constants ===
        // =================

        /// Name of the `path` field of the [`inodes::TABLE`].
        ///
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        pub(super) const PATH: &str = "path";
        /// Name of the `file_extension` field of the [`inodes::TABLE`].
        ///
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        pub(super) const FILE_EXTENSION: &str = "file_extension";
        /// Name of the `inode_type` field of the [`inodes::TABLE`].
        ///
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        pub(super) const INODE_TYPE: &str = "inode_type";
        /// Name of the `depth` field of the [`inodes::TABLE`].
        ///
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        pub(super) const DEPTH: &str = "depth";
        /// Name of the `size` field of the [`inodes::TABLE`].
        ///
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        pub(super) const SIZE: &str = "size";
        /// Name of the `permissions` field of the [`inodes::TABLE`].
        ///
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        pub(super) const PERMISSIONS: &str = "permissions";
        /// Name of the `modified` field of the [`inodes::TABLE`].
        ///
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        pub(super) const MODIFIED: &str = "modified";
        /// Name of the `accessed` field of the [`inodes::TABLE`].
        ///
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        pub(super) const ACCESSED: &str = "accessed";
        /// Name of the `created` field of the [`inodes::TABLE`].
        ///
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        pub(super) const CREATED: &str = "created";
        /// Name of the `file_name` field of the [`inodes::TABLE`].
        ///
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        pub(super) const FILE_NAME: &str = "file_name";
        /// Name of the `file_stem` field of the [`inodes::TABLE`].
        ///
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        pub(super) const FILE_STEM: &str = "file_stem";
    }



    // =============
    // === Field ===
    // =============

    /// An enum of the fields in the [`inodes::TABLE`].
    ///
    /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
    #[derive(Debug)]
    pub(crate) enum Field {
        /// The [`field::PATH`] of the [`inodes::TABLE`].
        ///
        /// [`field::PATH`]: crate::sql::schema::inodes::fields::PATH
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        Path,
        /// The [`field::FILE_EXTENSION`] of the [`inodes::TABLE`].
        ///
        /// [`field::FILE_EXTENSION`]: crate::sql::schema::inodes::fields::FILE_EXTENSION
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        FileExtension,
        /// The [`field::INODE_TYPE`] of the [`inodes::TABLE`].
        ///
        /// [`field::INODE_TYPE`]: crate::sql::schema::inodes::fields::INODE_TYPE
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        InodeType,
        /// The [`field::DEPTH`] of the [`inodes::TABLE`].
        ///
        /// [`field::DEPTH`]: crate::sql::schema::inodes::fields::DEPTH
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        Depth,
    }


    // === Trait `impl`s ===

    impl AsRef<str> for Field {
        fn as_ref(&self) -> &str {
            match self {
                Self::Path => fields::PATH,
                Self::FileExtension => fields::FILE_EXTENSION,
                Self::InodeType => fields::INODE_TYPE,
                Self::Depth => fields::DEPTH,
            }
        }
    }



    // ==============
    // === Create ===
    // ==============

    /// Creates a new [`inodes::TABLE`].
    ///
    /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
    pub(crate) fn create(connection: &rusqlite::Connection) -> rusqlite::Result<()> {
        let sql = sql::create();
        connection.execute(&sql, crate::sql::NO_SQL_PARAMS)?;
        Ok(())
    }



    // ==============
    // === Select ===
    // ==============

    /// A builder for a query that selects all fields from the [`inodes::TABLE`].
    ///
    /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
    #[derive(Debug)]
    pub(crate) struct Select<'a, F>(crate::sql::Select<'a, F>);


    // === Main `impl` ===

    impl<F> Select<'_, F>
    where
        F: Fn(&rusqlite::Row) -> rusqlite::Result<crate::inode::Inode>,
    {
        /// Adds a `WHERE x MATCH y` clause to the query, then executes the query.
        pub(crate) fn r#match(&self, field: Field, value: &str) -> rusqlite::Result<Vec<crate::inode::Inode>> {
            let Self(builder) = self;
            let field = field.as_ref();
            builder.r#match(field, value)
        }

        /// Adds a `WHERE x = y` clause to the query, then executes the query.
        pub(crate) fn equals(&self, field: Field, value: &dyn rusqlite::ToSql) -> rusqlite::Result<Vec<crate::inode::Inode>> {
            let Self(builder) = self;
            let field = field.as_ref();
            builder.equals(field, value)
        }

        /// Executes this query without any further clauses.
        pub(crate) fn all(&self) -> rusqlite::Result<Vec<crate::inode::Inode>> {
            let Self(builder) = self;
            builder.all()
        }
    }


    // === Internal `impl` ===

    impl<'a, F> Select<'a, F>
    where
        F: Fn(&rusqlite::Row) -> rusqlite::Result<crate::inode::Inode>,
    {
        /// Returns a builder for a query that selects all fields from the [`inodes::TABLE`].
        ///
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        fn new(connection: &'a rusqlite::Connection, mapper: F) -> Self {
            let builder = crate::sql::Select::new(connection, TABLE, &FIELDS, mapper);
            Self(builder)
        }
    }



    // ==============
    // === Inodes ===
    // ==============

    /// A guard for operations on the [`inodes::TABLE`].
    ///
    /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
    pub(crate) struct Inodes<'a> {
        pub connection: &'a mut rusqlite::Connection,
    }


    // === Main `impl` ===

    impl Inodes<'_> {
        /// Returns the number of rows in the [`inodes::TABLE`].
        ///
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        pub(crate) fn count(&self) -> rusqlite::Result<usize> {
            crate::sql::count(self.connection, TABLE)
        }

        /// Insert a [`Vec`] of [`Inode`]s into the [`inodes::TABLE`].
        ///
        /// [`Vec`]: std::vec::Vec
        /// [`Inode`]: crate::inode::Inode
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        pub(crate) fn insert_many<'a>(&mut self, items: impl IntoIterator<Item = &'a crate::inode::Inode>) -> rusqlite::Result<()> {
            crate::sql::with_transaction(self.connection, move |transaction| {
                let items = items.into_iter();
                let mut statement = crate::sql::insert(&transaction, TABLE, &FIELDS)?;
                for inode in items {
                    Self::insert_one(&mut statement, inode)?;
                }
                Ok(())
            })?;
            Ok(())
        }

        /// Returns a constructor for a query that selects all the fields in the [`inodes::TABLE`].
        ///
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        pub(crate) fn select(&self) -> Select<'_, impl Fn(&rusqlite::Row) -> rusqlite::Result<crate::inode::Inode>> {
            let mapper = |row: &rusqlite::Row| {
                let path = row.get(fields::PATH)?;
                let file_extension = row.get(fields::FILE_EXTENSION)?;
                let inode_type = row.get(fields::INODE_TYPE)?;
                let depth = row.get(fields::DEPTH)?;
                let size = row.get(fields::SIZE)?;
                let permissions = row.get(fields::PERMISSIONS)?;
                let modified = row.get(fields::MODIFIED)?;
                let accessed = row.get(fields::ACCESSED)?;
                let created = row.get(fields::CREATED)?;
                let file_name = row.get(fields::FILE_NAME)?;
                let file_stem = row.get(fields::FILE_STEM)?;
                Ok(crate::inode::Inode {
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
                })
            };
            Select::new(self.connection, mapper)
        }
    }


    // === Internal `impl` ===

    impl Inodes<'_> {
        /// Executes a prepared statement to insert a value into the [`inodes::TABLE`].
        ///
        /// [`inodes::TABLE`]: crate::sql::schema::inodes::TABLE
        fn insert_one(statement: &mut rusqlite::Statement, inode: &crate::inode::Inode) -> rusqlite::Result<()> {
            let crate::inode::Inode {
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
            } = inode;
            let params: [&dyn rusqlite::ToSql; FIELD_COUNT] = [
                &path,
                &file_extension,
                &inode_type,
                &depth,
                &size,
                &permissions,
                &modified,
                &accessed,
                &created,
                &file_name,
                &file_stem,
            ];
            statement.execute(&params)?;
            Ok(())
        }
    }
}

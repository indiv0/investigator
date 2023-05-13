use crate::inode;
use sqlx::sqlite;
use sqlx::migrate;
use std::str::FromStr as _;
use std::time;

const CREATE_IF_MISSING: bool = true;
const MAX_CONNECTIONS: u32 = 5;
/// Busy timeout in seconds.
const BUSY_TIMEOUT: u64 = 1;
const JOURNAL_MODE: sqlite::SqliteJournalMode = sqlite::SqliteJournalMode::Wal;
const SYNCHRONOUS: sqlite::SqliteSynchronous = sqlite::SqliteSynchronous::Normal;

async fn create_pool(connection_address: &str) -> Result<sqlx::SqlitePool, sqlx::Error> {
    let busy_timeout = time::Duration::from_secs(BUSY_TIMEOUT);
    let connect_options = sqlite::SqliteConnectOptions::from_str(connection_address)?;
    let connect_options = connect_options
        .create_if_missing(CREATE_IF_MISSING)
        .journal_mode(JOURNAL_MODE)
        .synchronous(SYNCHRONOUS)
        .busy_timeout(busy_timeout);
    let pool_options = sqlite::SqlitePoolOptions::new()
        .max_connections(MAX_CONNECTIONS);
    let pool = pool_options.connect_with(connect_options).await?;
    Ok(pool)
}

pub struct Database {
    pool: sqlx::SqlitePool,
}

impl Database {
    pub async fn new(connection_address: &str) -> Result<Self, sqlx::Error> {
        let pool = create_pool(connection_address).await?;
        let database = Self { pool };
        database.migrate().await?;
        Ok(database)
    }

    async fn migrate(&self) -> Result<(), migrate::MigrateError> {
        sqlx::migrate!("./migrations").run(&self.pool).await
    }

    pub async fn insert_inodes(&self, inodes: Vec<inode::Inode>) -> Result<(), sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
        for inode in inodes {
            let inode_type = match inode.inode_type {
                inode::InodeType::Directory => "directory",
                inode::InodeType::File => "file",
                inode::InodeType::SymbolicLink => "symlink",
                inode::InodeType::Socket => "socket",
                inode::InodeType::BlockDevice => "block_device",
                inode::InodeType::CharDevice => "char_device",
                inode::InodeType::Fifo => "fifo",
            };
            let depth = inode.depth as i32;
            let size = inode.size as i64;
            let permissions = inode.permissions as i32;
            let modified = inode.modified as i64;
            let accessed = inode.accessed as i64;
            let created = inode.created as i64;
            sqlx::query!(
                "INSERT INTO inodes (
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
                    file_stem
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                inode.path,
                inode.file_extension,
                inode_type,
                depth,
                size,
                permissions,
                modified,
                accessed,
                created,
                inode.file_name,
                inode.file_stem
            ).execute(&mut transaction).await?;
        }
        transaction.commit().await?;
        Ok(())
    }

    pub async fn select_inodes(&self) -> Result<Vec<inode::Inode>, sqlx::Error> {
        // FIXME [NP]: find out why SELECT * doesn't work here
        //let inodes = sqlx::query_as!(
        //    inode::Inode,
        //    "SELECT
        //        path as \"path!: String\",
        //        file_extension as \"file_extension!: String\",
        //        inode_type as \"inode_type!: inode::InodeType\",
        //        depth as \"depth!: usize\",
        //        size as \"size!: u64\",
        //        permissions as \"permissions!: u32\",
        //        modified as \"modified!: i128\",
        //        accessed as \"accessed!: i128\",
        //        created as \"created!: i128\",
        //        file_name as \"file_name!: String\",
        //        file_stem as \"file_stem!: String\"
        //    FROM inodes"
        //).fetch_all(&self.pool).await?;
        let inodes = sqlx::query!(
            "SELECT
                path as \"path!: String\",
                file_extension as \"file_extension: String\",
                inode_type as \"inode_type!: String\",
                depth as \"depth!: i64\",
                size as \"size!: i64\",
                permissions as \"permissions!: i32\",
                modified as \"modified!: i64\",
                accessed as \"accessed!: i64\",
                created as \"created!: i64\",
                file_name as \"file_name!: String\",
                file_stem as \"file_stem: String\"
            FROM inodes"
        )
        .map(|row| {
            println!("row: {:?}", row);
            // FIXME [NP]: get rid of these lossy conversions
            inode::Inode {
                path: row.path,
                file_extension: row.file_extension,
                inode_type: match row.inode_type.as_str() {
                    "directory" => inode::InodeType::Directory,
                    "file" => inode::InodeType::File,
                    "symlink" => inode::InodeType::SymbolicLink,
                    "socket" => inode::InodeType::Socket,
                    "block_device" => inode::InodeType::BlockDevice,
                    "char_device" => inode::InodeType::CharDevice,
                    "fifo" => inode::InodeType::Fifo,
                    inode_type => panic!("Unknown inode type: {inode_type}"),
                },
                //inode_type: match row.inode_type {
                //    0 => inode::InodeType::File,
                //    1 => inode::InodeType::Directory,
                //    2 => inode::InodeType::SymbolicLink,
                //    3 => inode::InodeType::Socket,
                //    4 => inode::InodeType::BlockDevice,
                //    5 => inode::InodeType::CharDevice,
                //    6 => inode::InodeType::Fifo,
                //    _ => panic!("Unknown inode type"),
                //},
                depth: row.depth as usize,
                size: row.size as u64,
                permissions: row.permissions as u32,
                modified: row.modified as i128,
                accessed: row.accessed as i128,
                created: row.created as i128,
                file_name: row.file_name,
                file_stem: row.file_stem,
            }
        })
        .fetch_all(&self.pool)
        .await?;
        Ok(inodes)
    }
}

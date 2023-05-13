CREATE VIRTUAL TABLE IF NOT EXISTS inodes USING fts5 (
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
)

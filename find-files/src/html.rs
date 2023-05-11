use std::fs;
use std::fmt;
use std::io::Write as _;
use std::fmt::Write as _;



// =================
// === Constants ===
// =================

/// Name of the HTML file to write.
const FIND_FILES_HTML: &str = "find-files.html";



// ======================
// === Write Optional ===
// ======================

macro_rules! write_opt {
    ($buf:ident, $field:ident) => {
        if let Some($field) = &$field {
            write!(
                $buf,
                r#"<td>{}</td>"#,
                $field,
            )?;
        } else {
            write!(
                $buf,
                r#"<td></td>"#,
            )?;
        }
    };
}


// ==============
// === Render ===
// ==============

/// Renders a struct as HTML.
trait Render {
    /// Renders a struct as HTML.
    fn render(&self, buf: &mut String) -> fmt::Result;
}


// === Trait `impl`s ===

impl Render for crate::inode::Inode {
    fn render(&self, buf: &mut String) -> fmt::Result {
        let Self {
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
        } = self;
        write!(buf, "<tr>")?;
        write!(buf, "<td>{path}</td>")?;
        write_opt!(buf, file_extension);
        write!(buf, "<td>{inode_type:?}</td>")?;
        write!(buf, "<td>{depth:?}</td>")?;
        write!(buf, "<td>{size}</td>")?;
        write!(buf, "<td>{permissions:o}</td>")?;
        write!(buf, "<td>{modified:?}</td>")?;
        write!(buf, "<td>{accessed:?}</td>")?;
        write!(buf, "<td>{created:?}</td>")?;
        write!(buf, "<td>{file_name}</td>")?;
        write_opt!(buf, file_stem);
        write!(buf, "</tr>")?;
        Ok(())
    }
}



// ==============
// === Inodes ===
// ==============

/// An HTML component for rendering a table of [`Inode`]s.
#[derive(Debug)]
struct Inodes {
    /// The [`Inode`]s to render.
    inodes: Vec<crate::inode::Inode>,
}


// === Trait `impl`s ===

impl Render for Inodes {
    fn render(&self, buf: &mut String) -> fmt::Result {
        let Self { inodes } = self;
        write!(
            buf,
            r#"<table>
              <thead>
                <tr>
                <th>path</th>
                <th>file_extension</th>
                <th>inode_type</th>
                <th>depth</th>
                <th>size</th>
                <th>permissions</th>
                <th>modified</th>
                <th>accessed</th>
                <th>created</th>
                <th>file_name</th>
                <th>file_stem</th>
                </tr>
              </thead>
              <tbody>"#
        )?;
        for inode in inodes {
            inode.render(buf)?;
        }
        write!(
            buf,
            r#"</tbody>
            </table>"#)?;
        Ok(())
    }
}



// ============
// === html ===
// ============

/// Renders a list of inodes as HTML.
pub(crate) fn html(inodes: Vec<crate::inode::Inode>) -> anyhow::Result<String> {
    let inodes = Inodes { inodes };
    let mut string = String::new();
    let buf = &mut string;
    write!(buf, r#"<html><head><title>find-files</title></head><body>"#)?;
    inodes.render(buf)?;
    string.push_str(r#"</body></html>"#);
    Ok(string)
}

/// Writes HTML to a file.
pub(crate) fn write_html(html: &str) -> anyhow::Result<()> {
    let mut file = fs::File::create(FIND_FILES_HTML)?;
    let html = html.as_bytes();
    file.write_all(html)?;
    Ok(())
}

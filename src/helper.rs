use std::{path::Path, fs, io};


/**
 * Copy a directory recursively.
 * This is used to copy the template vault directory to the new vault directory.
 */
pub fn copy_dir_rec(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            copy_dir_rec(entry.path(), dest.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dest.as_ref().join(entry.file_name()))?;
        }
    }
    return Ok(());
}

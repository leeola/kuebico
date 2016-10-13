use std::io;
use std::fs::DirBuilder;
use std::path::Path;

/// Make all of the given directories.
///
/// A convenience function around DirBuilder
pub fn mkdir_all(p: &Path) -> io::Result<()> {
    DirBuilder::new().recursive(true).create(p)
}

/// Create all of the directories for the given path, except the final component.
///
/// Similar to mk_dir_all, mkdir_just_parents creates all of the needed directories
/// for the given path to exist in, but *not* the final path component. Intended
/// to simply create the dirs needed for a file to be created in.
///
/// If there are no parents available to create, no directories are created.
pub fn mkdir_just_parents(p: &Path) -> io::Result<()> {
    let d = match p.parent() {
        Some(d) => d,
        None => return Ok(()),
    };

    mkdir_all(&d)
}

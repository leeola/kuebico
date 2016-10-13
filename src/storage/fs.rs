use std::io::{self, Read};
use std::fs::File;
use std::path::PathBuf;
// use mdmatter;
use storage::{self, Storage, PageResult, Page, Metadata};
use storage::Error::{self as StorageError, ImplError};

pub enum Error {
    StorageDirNotFound,
    IgnoringHiddenName,

    IoError(io::Error),
}

pub type FsResult = Result<Page, StorageError<Error>>;

/// A kuebico storage backend on the filesystem.
///
/// The Fs struct implements a kuebico storage where the pages are stored on
/// the filesystem as <page_name>.md and formatted via markdown with
/// frontmatter headers.
///
/// This backend is primarily useful for simple and/or low load environmens and
/// ones where the content can be read and edited in a human friendly format.
pub struct Fs {
    /// The path to use as the backend storage.
    pub path: PathBuf,

    /// Whether or not to ignore hidden files
    pub ignore_hidden: bool,

    /// The file extension for the storage content files.
    pub extension: Option<String>,
}

impl Fs {
    /// Create a new Fs storage.
    pub fn new(path: PathBuf) -> Fs {
        Fs {
            path: path,
            // Defaulting the rest of the values.
            ignore_hidden: true,
            extension: Some(String::from("md")),
        }
    }
}

impl Storage<Error> for Fs {
    fn read(&self, name: String) -> FsResult {
        let mut path = PathBuf::from(&self.path);
        path.push(&*name);

        if let Some(ref ext) = self.extension {
            path.set_extension(ext);
        }

        let mut file = try!(File::open(&path));

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut source = String::new();
        if let Err(err) = file.read_to_string(&mut source) {
            return Err(ImplError(Error::IoError(err)));
        }

        // TODO(leeola): replace with the actual parsed frontmatter
        let meta = Metadata {
            title: None,
            template: String::from("page"),
        };

        Ok(Page {
            name: name,
            source: source,
            metadata: meta,
        })
    }

    fn write(&self, name: &str, data: &str) -> Result<(), StorageError<Error>> {
        Ok(())
    }
}

impl From<io::Error> for StorageError<Error> {
    fn from(err: io::Error) -> StorageError<Error> {
        ImplError(Error::IoError(err))
    }
}

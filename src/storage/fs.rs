use std::fmt::{self, Debug};
use std::error;
use std::io::{self, Read};
use std::fs::File;
use std::path::{self, PathBuf};
// use mdmatter;
use walkdir::{self, DirEntry, WalkDir, WalkDirIterator};
use storage::{self, Storage, StorageIter, Error as StorageError, PageResult, Page, Metadata};

#[derive(Debug)]
pub enum Error {
    StorageDirNotFound,
    IgnoringHiddenName,

    // Returned by path::PathBuf.to_str() if the path is not valid unicode.
    PathNotValidUnicode,
    IoError(io::Error),
    WalkdirError(walkdir::Error),
    StripPrefixError(path::StripPrefixError),
}

pub type FsResult = Result<Page, StorageError>;

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

impl Storage for Fs {
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
            return Err(StorageError::ImplError(Box::new(Error::IoError(err))));
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

    fn write(&self, name: &str, data: &str) -> Result<(), StorageError> {
        Ok(())
    }
}

impl IntoIterator for Fs {
    type Item = FsResult;
    type IntoIter = FsIter;

    fn into_iter(self) -> FsIter {
        // TODO(leeola): Can Fs not be moved into FsIter? Ie, to avoid the clone.
        // Not sure if it's possible, i may have to move the fields by hand.
        FsIter::new(self.clone())
    }
}

impl Clone for Fs {
    fn clone(&self) -> Fs {
        Fs {
            path: self.path.clone(),
            ignore_hidden: self.ignore_hidden,
            extension: self.extension.clone(),
        }
    }
}

pub struct FsIter {
    // Fs is used for the read and write operations.
    fs: Fs,

    // An object to iterate through the files on the filesystem.
    //
    // Note that i'm using this iterator because the library returns two types
    // depending on whether the iterator is filtered or not.
    filesystem_walker: Box<Iterator<Item = walkdir::Result<DirEntry>>>,
}

impl FsIter {
    pub fn new(fs: Fs) -> FsIter {
        let iter = WalkDir::new(fs.path.clone()).into_iter();

        // TODO(leeola): Generate this on the fly, as needed. Eg, store it as
        // an Option and if it's None, generate it.
        //
        // We're using a match here because a filtered walker and
        //
        // Note that we need to ignore dot hidden here, instead of just ignoring it
        // during read/write, because we have to ignore hidden directories as well.
        let walker: Box<Iterator<Item = walkdir::Result<DirEntry>>> = match fs.ignore_hidden {
            true => {
                Box::new(iter.filter_entry(|entry| {
                    !entry.file_name().to_str().map(|s| s.starts_with(".")).unwrap_or(false)
                }))
            }
            false => Box::new(iter),
        };

        FsIter {
            fs: fs,
            filesystem_walker: walker,
        }
    }
}

impl StorageIter for FsIter {}

impl Iterator for FsIter {
    type Item = Result<Page, StorageError>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.filesystem_walker.next() {
            Some(e) => e,
            None => return None,
        };

        let entry = try_some!(res);
        let mut path = entry.path();

        // if the path is a dir we can skip it, as there is nothing Kuebico Storage
        // does for directories.
        if path.is_dir() {
            debug!("Skipping directory {}", path.display());
            return self.next();
        }

        // We must trim the base off of the path. Since we just want to use
        // ID from the storage.
        path = try_some!(path.strip_prefix(&self.fs.path));

        let mut path_str = match path.to_str() {
            Some(p) => p,
            // Looks a bit weird, but using try_some! here makes this more clean,
            // and the verboseness is handled via From::, just like the other
            // try_some! calls.
            None => try_some!(Err(Error::PathNotValidUnicode)),
        };

        // If there's an extension, remove it from the path_str. Note that
        // we're not using file_stem() because it returns just the filename.
        // What we want, is the full path, without the last segment's
        // extension.
        if let Some(os_ext) = path.extension() {
            if let Some(ext) = os_ext.to_str() {
                path_str = path_str.trim_right_matches(&*format!(".{}", ext));
            }
        }

        let name = String::from(path_str);
        Some(self.fs.read(name))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "foo"
    }
}

impl From<io::Error> for StorageError {
    fn from(err: io::Error) -> StorageError {
        StorageError::ImplError(Box::new(Error::IoError(err)))
    }
}

impl From<walkdir::Error> for StorageError {
    fn from(err: walkdir::Error) -> StorageError {
        StorageError::ImplError(Box::new(Error::WalkdirError(err)))
    }
}

impl From<path::StripPrefixError> for StorageError {
    fn from(err: path::StripPrefixError) -> StorageError {
        StorageError::ImplError(Box::new(Error::StripPrefixError(err)))
    }
}

impl From<Error> for StorageError {
    fn from(err: Error) -> StorageError {
        StorageError::ImplError(Box::new(err))
    }
}

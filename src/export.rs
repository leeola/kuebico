use std::path::PathBuf;
use storage::{self, StorageIter, Error as StorageError, PageResult};
use storage::fs::Fs;
use util::mkdirall;

pub enum Error {
    StorageError(StorageError),
}

pub enum StorageType {
    Fs(FsSettings),
}

/// A series of settings for the Fs storage type.
pub struct FsSettings {
    pub storage_dir: PathBuf,
}

/// A series of settings for an Exporter to be created from and run.
///
/// Primarily existing for easily converting CLI arguments to a usable Exporter,
/// but also can be used to easily construct an exporter.
pub struct ExportSettings {
    /// The type of storage to use.
    pub storage: StorageType,

    /// An optional static directory to copy to the output directory.
    pub static_dir: Option<PathBuf>,

    /// The target directory to export to.
    pub export_dir: PathBuf,
}

type BoxedStorageIter = Box<StorageIter>;

pub struct Exporter {
    storage: BoxedStorageIter,
    static_dir: Option<PathBuf>,
    export_dir: PathBuf,
}

impl Exporter {
    pub fn new(storage: BoxedStorageIter,
               export_dir: PathBuf,
               static_dir: Option<PathBuf>)
               -> Exporter {
        Exporter {
            storage: storage,
            export_dir: export_dir,
            static_dir: static_dir,
        }
    }

    // pub fn new_from_settings(settings: ExportSettings) -> Result<Exporter, Error> {

    //     let storage = match settings.storage {
    //         StorageType::Fs(fs_settings) => {
    //             Box::new(Fs::new(PathBuf::from(fs_settings.storage_dir)).into_iter())
    //         }
    //     };

    //     Ok(Exporter::new(storage, settings.export_dir, settings.static_dir))
    // }
}

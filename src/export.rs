use std::path::PathBuf;
use std::marker::PhantomData;
use storage::{self, Storage, Error as StorageError};

pub enum Error<E> {
    StorageError(E),
}

pub struct ExporterSettings {
    static_dir: Option<PathBuf>,
    export_dir: PathBuf,
}

pub struct Exporter<S, E>
    where S: Storage<E>
{
    _phantom_data: PhantomData<E>,
    storage: S,
    static_dir: Option<PathBuf>,
    export_dir: PathBuf,
}

impl<S, E> Exporter<S, E>
    where S: Storage<E>
{
    pub fn new(storage: S, export_dir: PathBuf, static_dir: Option<PathBuf>) -> Exporter<S, E> {
        Exporter {
            storage: storage,
            export_dir: export_dir,
            static_dir: static_dir,
            _phantom_data: PhantomData,
        }
    }

    // TODO(leeola): Probably convert this to a TryFrom?
    // pub fn new_from_settings(settings: ExporterSettings) -> Exporter<S> {
    // }
}

// impl<S, E> IntoIterator for Exporter<S> where S: Storage {
//     type Item = Result<String, Error<E>>
// }

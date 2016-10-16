use std::error;
use std::fmt::{self, Debug};

// #[cfg(feature = "fs-storage")]
pub mod fs;

pub enum Error {
    /// The given Page name could not be found.
    PageNotFound(String),

    /// A generic error type provided by the storage implementor.
    ImplError(Box<error::Error>),
}

pub type PageResult = Result<Page, Error>;

/// The backend used for basic page crud operations.
pub trait Storage {
    fn read(&self, name: String) -> PageResult;

    fn write(&self, name: &str, data: &str) -> Result<(), Error>;

    // not enabled yet. Many storage methods are to be added, for a complete backend.
    // fn search(query: &str) -> Result<Vec<Page>, E>
}

/// A Storage that can be iterated over, reading *all* pages.
pub trait StorageIter: Iterator<Item = PageResult> {}

/// A page, visible at a URL
pub struct Page {
    // the identifier for this page. Note that this can contain a path if the
    // page is to be grouped within a page. Eg, `faq/long-answer`
    pub name: String,

    // the markdown source of the page, representing the state of the page after
    // the latest entry.
    //
    // TODO(leeola): Change source into some type of streaming IO trait,
    // for efficient streaming returns. Ie, Golang's Reader or ReadCloser.
    pub source: String,

    // the metadata for the page, for data like template.
    pub metadata: Metadata,
}

/// Metadata is the data that is passed into the page, along with the actual page
/// content.
///
/// In the future this may become a map, as to accomodate user frontmatter fields.
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub title: Option<String>,

    // #[serde(default = "default_metadata_template")]
    pub template: String,
}

fn default_metadata_template() -> String {
    String::from("page")
}

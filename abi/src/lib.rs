mod config;
mod error;
mod pb;
mod types;

pub use config::*;
pub use error::Error;
pub use pb::*;
pub use types::*;

pub type DocumentId = String;
pub type UserId = String;

/// validate the data structure, raise an error if invalid
pub trait Validator {
    fn validate(&self) -> Result<(), Error>;
}

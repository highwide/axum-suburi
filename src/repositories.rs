pub mod label;
pub mod todo;

use thiserror::Error;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("unexpected Error: [{0}]")]
    Unexpected(String),
    #[error("NotFound, id is {0}")]
    NotFound(i32),
    #[error("Duplicate, id is {0}")]
    Duplicate(i32),
}

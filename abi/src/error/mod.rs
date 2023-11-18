#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error")]
    DbError(#[from] sqlx::Error),

    #[error("Failed to read configuration file")]
    ConfigReadError,

    #[error("Failed to parse configuration file")]
    ConfigParseError,

    #[error("Invalid user id: {0}")]
    InvalidUserId(String),

    #[error("Invalid start or end time range")]
    InvalidTime,

    #[error("No document found by the given id")]
    NotFound,

    #[error("unknown error")]
    Unknown,
}

impl From<Error> for tonic::Status {
    fn from(e: Error) -> Self {
        match e {
            Error::ConfigReadError | Error::ConfigParseError => {
                tonic::Status::internal(e.to_string())
            }
            Error::DbError(e) => tonic::Status::internal(e.to_string()),
            Error::InvalidTime | Error::InvalidUserId(_) => {
                tonic::Status::invalid_argument(e.to_string())
            }
            Error::NotFound => tonic::Status::not_found("No document found by the given id"),
            Error::Unknown => tonic::Status::unknown("unknown error"),
        }
    }
}

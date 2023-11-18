use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("Invalid URL: {0}")]
    InvalidURL(#[from] url::ParseError),

    #[error("File \"{0}\" not found")]
    RemoteFileNotFound(String),

    #[error("Unable to obtain progress")]
    ProgressError,

    #[error("Server unavailable")]
    ServerError,

    #[error("An unexpected error happened: {0}")]
    Unknown(reqwest::StatusCode),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    Http(#[from] reqwest::Error),
}

impl serde::Serialize for DownloadError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

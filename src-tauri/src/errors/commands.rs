use serde_json::{json, Value};
use thiserror::Error;

use super::{ApiError, AuthError, DownloadError, RegistryError, TokensError};

#[derive(Error, Debug)]
pub enum CommandError {
    #[error(transparent)]
    Auth(#[from] AuthError),

    #[error(transparent)]
    Download(#[from] DownloadError),

    #[error(transparent)]
    Registry(#[from] RegistryError),

    #[error(transparent)]
    Semver(#[from] semver::Error),

    #[error(transparent)]
    Api(#[from] ApiError),

    #[error(transparent)]
    Tokens(#[from] TokensError),
}

impl serde::Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let error = json!({ "error": &self.to_string() });

        error.serialize(serializer)
    }
}

pub type CommandResult<T> = Result<T, CommandError>;

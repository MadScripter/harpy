use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("initialize the registry first")]
    NotInitialized,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("unable to update the registry: {0}")]
    Encode(String),

    #[error("unable to decode the registry: {0}")]
    Decode(String),
}

impl serde::Serialize for RegistryError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

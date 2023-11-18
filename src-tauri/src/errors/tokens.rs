use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokensError {
    #[error(transparent)]
    Keyring(#[from] keyring::error::Error),
}

impl serde::Serialize for TokensError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

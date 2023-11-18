use sha256::try_digest;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HashError {
    #[error("Hash mismatch: expected: {0}, got: {1}")]
    Mismatch(String, String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub struct Hasher;

impl Hasher {
    pub fn validate(file: &str, expected_hash: &str) -> Result<(), HashError> {
        let file = Path::new(file);
        let hash = try_digest(file).map_err(HashError::IoError)?;

        if !hash.eq(expected_hash) {
            return Err(HashError::Mismatch(expected_hash.to_string(), hash));
        }

        Ok(())
    }
}

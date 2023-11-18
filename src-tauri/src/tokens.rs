use crate::errors::TokensError;

#[derive(Default, Clone)]
pub struct Tokens {
    refresh_token: Option<String>,
    access_token: Option<String>,
}

impl Tokens {
    pub fn new() -> Self {
        Self {
            refresh_token: None,
            access_token: None,
        }
    }

    pub fn set_access_token(&mut self, token: String) -> &mut Self {
        self.access_token = Some(token);

        self
    }

    pub fn set_refresh_token(&mut self, token: String) -> &mut Self {
        self.refresh_token = Some(token);

        self
    }

    pub fn get_access_token(&self) -> &str {
        self.access_token.as_ref().unwrap()
    }

    pub fn get_refresh_token(&self) -> &str {
        self.refresh_token.as_ref().unwrap()
    }

    pub fn has_refresh_token(&self) -> bool {
        self.refresh_token.is_some()
    }

    pub fn load(&mut self) -> Result<(), TokensError> {
        let entry = keyring::Entry::new("Harpy.Auth.RefreshToken", whoami::username().as_str())
            .map_err(TokensError::Keyring)?;

        self.refresh_token = entry.get_password().ok();

        Ok(())
    }

    pub fn store(&self) -> Result<(), TokensError> {
        if let Some(ref token) = self.refresh_token {
            let entry = keyring::Entry::new("Harpy.Auth.RefreshToken", &whoami::username())
                .map_err(TokensError::Keyring)?;

            entry.set_password(token).map_err(TokensError::Keyring)?;
        }

        Ok(())
    }

    pub fn delete(&mut self) -> Result<(), TokensError> {
        let entry = keyring::Entry::new("Harpy.Auth.RefreshToken", &whoami::username())
            .map_err(TokensError::Keyring)?;

        entry.delete_password().map_err(TokensError::Keyring)?;

        self.access_token = None;
        self.refresh_token = None;

        Ok(())
    }
}

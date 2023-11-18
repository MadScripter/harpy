use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid request")]
    InvalidRequest,

    #[error("Wrong application")]
    WrongApplication,

    #[error("Invalid API token")]
    InvalidApiToken,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Invalid auth token")]
    InvalidAuthToken,

    #[error("The refresh token has expired")]
    ExpiredRefreshToken,

    #[error("A password change is required")]
    PasswordChangeRequired,

    #[error("Login prevented")]
    LoginPrevented,

    #[error("User account expired")]
    AccountExpired,

    #[error("Account locked")]
    AccountLocked,

    #[error("Server unavailable")]
    ServerError,

    #[error("An unexpected error happened: {0}")]
    Unknown(StatusCode),

    #[error("Unknown error :: {}", .0)]
    Http(#[from] reqwest::Error),
}

impl serde::Serialize for AuthError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

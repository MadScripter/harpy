use crate::{constants, errors::AuthError, models::LoginResponse, service::Service};
use reqwest::StatusCode;
use serde_json::json;

pub struct Auth;

impl Auth {
    pub async fn login(
        service: &mut Service,
        email: &str,
        password: &str,
    ) -> Result<LoginResponse, AuthError> {
        let body = json!({
            "applicationId": constants::APPLICATION_ID,
            "loginId": email,
            "password": password
        });

        let response = service
            .with_header("Authorization", constants::AUTH_API_KEY)
            .post("api/login", Some(&body))
            .send()
            .await
            .map_err(AuthError::Http)?;

        match response.status() {
            StatusCode::OK => {
                let data = response.json::<LoginResponse>().await?;

                service.set_token(&data.token);

                Ok(data)
            }
            StatusCode::ACCEPTED => Err(AuthError::WrongApplication),
            StatusCode::NON_AUTHORITATIVE_INFORMATION => Err(AuthError::PasswordChangeRequired),
            StatusCode::BAD_REQUEST => Err(AuthError::InvalidRequest),
            StatusCode::UNAUTHORIZED => Err(AuthError::InvalidApiToken),
            StatusCode::NOT_FOUND => Err(AuthError::InvalidCredentials),
            StatusCode::CONFLICT => Err(AuthError::LoginPrevented),
            StatusCode::GONE => Err(AuthError::AccountExpired),
            StatusCode::LOCKED => Err(AuthError::AccountLocked),
            StatusCode::INTERNAL_SERVER_ERROR => Err(AuthError::ServerError),
            _ => Err(AuthError::Unknown(response.status())),
        }
    }

    pub async fn validate(service: &mut Service) -> Result<(), AuthError> {
        let response = service
            .with_auth()
            .get("auth-proxy/api/v1/auth/validate")
            .send()
            .await
            .map_err(AuthError::Http)?;

        match response.status() {
            StatusCode::OK => Ok(()),
            StatusCode::UNAUTHORIZED => Err(AuthError::InvalidAuthToken),
            StatusCode::INTERNAL_SERVER_ERROR => Err(AuthError::ServerError),
            _ => Err(AuthError::Unknown(response.status())),
        }
    }

    pub async fn refresh(service: &mut Service, token: &str) -> Result<(), AuthError> {
        let body = json!({ "refreshToken": token });

        let response = service
            .post("/api/jwt/refresh", Some(&body))
            .send()
            .await
            .map_err(AuthError::Http)?;

        match response.status() {
            StatusCode::OK => Ok(()),
            StatusCode::BAD_REQUEST => Err(AuthError::ExpiredRefreshToken),
            StatusCode::INTERNAL_SERVER_ERROR => Err(AuthError::ServerError),
            _ => Err(AuthError::Unknown(response.status())),
        }
    }

    pub async fn revoke(service: &mut Service) -> Result<(), AuthError> {
        Ok(())
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub refresh_token: String,
    pub refresh_token_id: String,
    pub token: String,
    pub token_expiration_instant: i64,
    pub user: User,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub active: bool,
    pub birth_date: String,
    pub breached_password_last_checked_instant: i64,
    pub breached_password_status: String,
    pub connector_id: String,
    pub data: UserData,
    pub email: String,
    pub id: String,
    pub insert_instant: i64,
    pub last_login_instant: i64,
    pub last_update_instant: i64,
    pub memberships: Vec<Membership>,
    pub password_change_required: bool,
    pub password_last_update_instant: i64,
    pub preferred_languages: Vec<String>,
    pub registrations: Vec<Registration>,
    pub tenant_id: String,
    pub two_factor: TwoFactor,
    pub unique_username: String,
    pub username: String,
    pub username_status: String,
    pub verified: bool,
}

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub country: String,
    pub kws: Kws,
    pub subscribed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Kws {
    pub is_graduated: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Membership {
    pub data: TokensClass,
    pub group_id: String,
    pub id: String,
    pub insert_instant: i64,
}

#[derive(Serialize, Deserialize)]
pub struct TokensClass {}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Registration {
    pub application_id: String,
    pub data: TokensClass,
    pub id: String,
    pub insert_instant: i64,
    pub last_login_instant: i64,
    pub last_update_instant: i64,
    pub preferred_languages: Vec<Option<serde_json::Value>>,
    pub roles: Vec<String>,
    pub tokens: TokensClass,
    pub username_status: String,
    pub verified: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwoFactor {
    pub methods: Vec<Option<serde_json::Value>>,
    pub recovery_codes: Vec<Option<serde_json::Value>>,
}

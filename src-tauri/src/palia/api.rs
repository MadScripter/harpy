use crate::{errors::ApiError, models::Channel};

pub async fn get_channel() -> Result<Channel, ApiError> {
    let response = reqwest::get("https://dl.palia.com:443/bundle/Palia/channel/live")
        .await
        .map_err(ApiError::Http)?;

    let channel = response.json::<Channel>().await.map_err(ApiError::Http)?;

    Ok(channel)
}

pub fn get_manifest(channel: &Channel) -> String {
    format!(
        "https://dl.palia.com:443/bundle/{}/v/{}/windows/manifest",
        channel.bundle(),
        channel.version()
    )
}

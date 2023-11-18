use std::path::Path;

use crate::{errors::CommandResult, palia, requisites, state::AppState, utils};
use semver::Version;
use serde::Serialize;
use sysinfo::{DiskExt, RefreshKind, System, SystemExt};
use tauri::{AppHandle, State};

#[derive(Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum GameStatus {
    Missing,
    UpdateRequired,
    UpToDate,
    Unknown,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InitSummary {
    status: GameStatus,
    requisites_installed: bool,
    current_version: String,
    latest_version: String,
}

impl InitSummary {
    pub fn new() -> Self {
        Self {
            status: GameStatus::Unknown,
            requisites_installed: false,
            current_version: "N/A".to_string(),
            latest_version: "N/A".to_string(),
        }
    }
}

#[tauri::command]
pub async fn init(state: State<'_, AppState>) -> CommandResult<InitSummary> {
    let mut summary = InitSummary::new();

    // Load installation information if there is one
    let palia = state.load_registry()?.to_owned();

    // Get latest information from channel
    let channel = palia::get_channel().await?;

    summary.latest_version = channel.version().to_string();
    state.update_channel(channel.clone());

    // Check if pre-requisites are installed
    summary.requisites_installed = requisites::installed()?;

    // Check if game is installed
    if !palia.installed() {
        summary.status = GameStatus::Missing;

        return Ok(summary);
    }

    // Check if game needs to be updated
    let latest_version = Version::parse(channel.version())?;
    let current_version = Version::parse(palia.current_version())?;

    summary.latest_version = latest_version.to_string();
    summary.current_version = current_version.to_string();

    if latest_version > current_version {
        summary.status = GameStatus::UpdateRequired;

        return Ok(summary);
    }

    summary.status = GameStatus::UpToDate;

    Ok(summary)
}

#[tauri::command]
pub async fn login(email: &str, password: &str, state: State<'_, AppState>) -> CommandResult<()> {
    Ok(())
}

#[tauri::command]
pub async fn logout(state: State<'_, AppState>) -> CommandResult<()> {
    Ok(())
}

#[tauri::command]
pub async fn install(
    handle: AppHandle,
    path: &str,
    with_requisites: bool,
    state: State<'_, AppState>,
) -> CommandResult<()> {
    Ok(())
}

#[tauri::command]
pub async fn update(handle: AppHandle, state: State<'_, AppState>) -> CommandResult<()> {
    Ok(())
}

#[tauri::command]
pub async fn pause() -> CommandResult<()> {
    Ok(())
}

#[tauri::command]
pub async fn resume() -> CommandResult<()> {
    Ok(())
}

#[tauri::command]
pub async fn cancel(state: State<'_, AppState>) -> CommandResult<()> {
    Ok(())
}

#[tauri::command]
pub async fn launch() -> CommandResult<()> {
    Ok(())
}

#[tauri::command]
pub async fn change_speed_limit(state: State<'_, AppState>, speed: f64) -> CommandResult<()> {
    Ok(())
}

#[tauri::command]
pub async fn clear_speed_limit(state: State<'_, AppState>) -> CommandResult<()> {
    Ok(())
}

#[tauri::command]
pub fn query_disk_space(path: &str) -> CommandResult<String> {
    let space = utils::query_available_space(path);

    Ok(human_bytes::human_bytes(space as f64))
}

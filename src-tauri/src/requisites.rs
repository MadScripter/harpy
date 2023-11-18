use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

use crate::errors::RegistryError;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct InstalledApp {
    DisplayName: Option<String>,
    DisplayVersion: Option<String>,
    UninstallString: Option<String>,
}

type Apps = HashMap<String, InstalledApp>;

pub fn installed() -> Result<bool, RegistryError> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let key = hklm
        .open_subkey(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall")
        .map_err(RegistryError::Io)?;

    let apps: Apps = key
        .decode()
        .map_err(|e| RegistryError::Decode(e.to_string()))?;

    let exists = apps.values().any(|app| {
        app.DisplayName
            .as_ref()
            .map(|name| name.contains("UE Prerequisites (x64)"))
            .unwrap_or(false)
    });

    Ok(exists)
}

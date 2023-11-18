use std::ffi::OsStr;

use crate::{constants, errors::RegistryError};
use chrono::{NaiveDate, Utc};
use lcid::LanguageId;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sys_locale::get_locale;
use winreg::{enums::HKEY_CURRENT_USER, types::ToRegValue, RegKey};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "PascalCase")]
pub struct PaliaInstallation {
    #[serde(rename = "")]
    default_key: String,
    display_icon: String,
    display_name: String,
    display_version: String,
    estimated_size: u32,

    #[serde(
        serialize_with = "serialize_date",
        deserialize_with = "deserialize_date"
    )]
    install_date: NaiveDate,
    install_location: String,

    #[serde(
        serialize_with = "serialize_language",
        deserialize_with = "deserialize_language"
    )]
    language: String,
    no_modify: u32,
    no_repair: u32,
    publisher: String,
    uninstall_string: String,

    #[serde(rename = "X-BaseVersion")]
    base_version: String,

    #[serde(rename = "X-Entry")]
    entry: String,

    #[serde(rename = "X-NdaAcceptedVersion")]
    nda_accepted_version: u32,

    #[serde(rename = "X-PatchMethod")]
    patch_method: String,
}

impl PaliaInstallation {
    pub fn current_version(&self) -> &str {
        &self.display_version
    }

    pub fn path(&self) -> &str {
        &self.install_location
    }

    pub fn installed(&self) -> bool {
        !self.path().is_empty()
    }

    pub fn install_at(&mut self, path: &str) -> &mut Self {
        self.install_location = path.into();

        self
    }

    pub fn version(&mut self, version: &str) -> &mut Self {
        self.display_version = version.into();
        self.base_version = version.into();

        self
    }

    pub fn size(&mut self, size: u32) -> &mut Self {
        self.estimated_size = size;

        self
    }

    pub fn icon(&mut self, path: &str) -> &mut Self {
        self.display_icon = path.into();

        self
    }

    pub fn uninstall(&mut self, path: &str) -> &mut Self {
        self.uninstall_string = path.into();

        self
    }

    pub fn nda(&mut self, accept: bool) -> &mut Self {
        self.nda_accepted_version = accept.into();

        self
    }

    pub fn patching_method(&mut self, method: &str) -> &mut Self {
        self.patch_method = method.into();

        self
    }
}

impl Default for PaliaInstallation {
    fn default() -> Self {
        Self {
            default_key: "".into(),
            display_icon: "".into(),
            display_name: "Palia".into(),
            display_version: "0.0.0".into(),
            estimated_size: 1,
            install_date: Utc::now().date_naive(),
            install_location: "".into(),
            language: get_locale().unwrap_or("en-US".into()),
            no_modify: 1,
            no_repair: 1,
            publisher: "Singularity 6 Corporation".into(),
            uninstall_string: "".into(),
            base_version: "0.0.0".into(),
            entry: "Palia.exe".into(),
            nda_accepted_version: 0,
            patch_method: "pak".into(),
        }
    }
}

pub struct Registry {
    key: Option<RegKey>,
    value: PaliaInstallation,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            key: None,
            value: PaliaInstallation::default(),
        }
    }

    pub fn load(&mut self) -> Result<(), RegistryError> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);

        let (key, _) = hkcu
            .create_subkey(constants::PALIA_REGISTRY_PATH)
            .map_err(RegistryError::Io)?;

        let value: PaliaInstallation = key.decode().map_err(|e| {
            println!("{:?}", e);
            RegistryError::Decode(e.to_string())
        })?;

        self.key = Some(key);
        self.value = value;

        Ok(())
    }

    pub fn entry(&self) -> &PaliaInstallation {
        &self.value
    }

    pub fn set<N, V>(&self, name: N, value: &V) -> Result<(), RegistryError>
    where
        N: AsRef<OsStr>,
        V: ToRegValue,
    {
        if let Some(ref key) = self.key {
            return key.set_value(name, value).map_err(RegistryError::Io);
        }

        Err(RegistryError::NotInitialized)
    }

    pub fn update(&self) -> Result<(), RegistryError> {
        if let Some(ref key) = self.key {
            return key
                .encode(&self.value)
                .map_err(|e| RegistryError::Encode(e.to_string()));
        }

        Err(RegistryError::NotInitialized)
    }

    pub fn update_from(&self, value: &PaliaInstallation) -> Result<(), RegistryError> {
        if let Some(ref key) = self.key {
            return key
                .encode(value)
                .map_err(|e| RegistryError::Encode(e.to_string()));
        }

        Err(RegistryError::NotInitialized)
    }
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let value: String = Deserialize::deserialize(deserializer).map_err(Error::custom)?;

    NaiveDate::parse_from_str(&value, "%Y%m%d").map_err(Error::custom)
}

fn serialize_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&date.format("%Y%m%d").to_string())
}

fn deserialize_language<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let lcid: u32 = Deserialize::deserialize(deserializer).map_err(Error::custom)?;
    let lang_id: &LanguageId = lcid.try_into().map_err(Error::custom)?;

    Ok(lang_id.name.into())
}

fn serialize_language<S>(language: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::Error;

    let lang_id: &LanguageId = language.try_into().map_err(Error::custom)?;

    serializer.serialize_u32(lang_id.lcid)
}

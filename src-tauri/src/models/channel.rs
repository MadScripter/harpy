use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    ok: bool,
    #[serde(rename = "as")]
    channel_as: String,
    v: Meta,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub version: String,
    pub is_public: bool,
    pub bundle: String,
    pub channel: String,
}

impl Channel {
    pub fn ok(&self) -> bool {
        self.ok
    }

    pub fn version(&self) -> &str {
        &self.v.version
    }

    pub fn is_public(&self) -> bool {
        self.v.is_public
    }

    pub fn bundle(&self) -> &str {
        &self.v.bundle
    }
}

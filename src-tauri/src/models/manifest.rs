use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};

use flexbuffers::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chunk(
    u64,
    u64,
    #[serde(deserialize_with = "deserialize_hash")] String,
);

impl Chunk {
    pub fn offset(&self) -> u64 {
        self.0
    }

    pub fn size(&self) -> u64 {
        self.1
    }

    pub fn hash(&self) -> &str {
        self.2.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManifestData {
    pub bundle: String,
    pub contents: Contents,
    pub platform: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contents {
    pub files: Vec<Files>,

    #[serde(deserialize_with = "deserialize_hash")]
    pub hash: String,
    pub path: String,
    pub size: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Files {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunks: Option<Vec<Chunk>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<Files>>,

    #[serde(deserialize_with = "deserialize_hash")]
    pub hash: String,
    pub path: String,
    pub size: u64,
}

fn deserialize_hash<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let blob: &'de [u8] = Deserialize::deserialize(deserializer)?;
    let hex = hex::encode(blob);

    Ok(hex)
}

impl ManifestData {
    pub fn list(&self) -> Vec<&Files> {
        let mut result = Vec::new();
        let mut stack = VecDeque::new();

        for file in &self.contents.files {
            stack.push_back(file);

            while let Some(current) = stack.pop_back() {
                if current.chunks.is_some() {
                    result.push(current);
                }

                if let Some(files) = &current.files {
                    for child in files.iter() {
                        stack.push_back(child);
                    }
                }
            }
        }

        result
    }

    pub async fn from(path: PathBuf) -> Self {
        let buffer = tokio::fs::read(path).await.unwrap();

        let reader = Reader::get_root(&buffer[..]).unwrap();

        Self::deserialize(reader).unwrap()
    }
}

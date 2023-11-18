use async_speed_limit::Limiter;
use bytes::Bytes;
use chrono::Duration;
use futures_util::{stream, StreamExt};
use reqwest::{Client, StatusCode};
use serde::Serialize;
use std::path::PathBuf;
use std::{io::Cursor, path::Path};
use tokio::io::AsyncReadExt;
use tokio::time::Instant;

use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncSeekExt, AsyncWriteExt},
};
use tokio_util::sync::CancellationToken;
use url::Url;

use crate::errors::DownloadError;
use crate::hasher::{HashError, Hasher};
use crate::progress::ProgressTracker;
use human_bytes::human_bytes;

#[derive(Default, Clone, Serialize)]
pub struct DownloadProgress<'a> {
    current: &'a str,
    total: &'a str,
    percentage: f64,
    speed: &'a str,
    eta: &'a str,
}

#[derive(Default)]
pub struct RequestMeta {
    etag: String,
    length: u64,
}

impl RequestMeta {
    pub fn size(&self) -> u64 {
        self.length
    }
}

pub struct Downloader;

impl Downloader {
    pub async fn download(
        client: Client,
        cache_dir: PathBuf,
        limiter: Limiter,
        item: DownloadItem<'_>,
        tracker: &ProgressTracker,
        cancel_token: CancellationToken,
    ) -> Result<(), DownloadError> {
        tracker.start();

        let start_time = Instant::now();

        let url = Url::parse(item.source).map_err(DownloadError::InvalidURL)?;

        // Get filename from URL
        let filename = url
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap();

        // Get remote file size
        let meta = Self::get_meta(&client, item.source).await?;

        // check if the file has been completely downloaded
        if Self::is_complete(&cache_dir, filename, meta.length).await? {
            /* TODO:
            - check hash to make sure
            - move file to destination
            */
            return Ok(());
        }

        // Create or open destination file
        let cache_file = cache_dir.join(format!("{}.part", filename));

        let mut file = match OpenOptions::new()
            .append(true)
            .create(true)
            .open(&cache_file)
            .await
        {
            Ok(file) => file,
            Err(e) => return Err(DownloadError::IoError(e)),
        };

        // Get file size to check if we need to resume or download from scratch
        let mut downloaded = match file.metadata().await {
            Ok(metadata) => metadata.len(),
            Err(e) => return Err(DownloadError::IoError(e)),
        };

        file.seek(std::io::SeekFrom::Start(downloaded)).await?;

        let range = format!("bytes={}-", downloaded);

        let response = client
            .get(item.source)
            .header("Range", range)
            .send()
            .await
            .map_err(DownloadError::Http)?;

        match response.status() {
            StatusCode::OK => (),
            StatusCode::PARTIAL_CONTENT => (),
            StatusCode::NOT_FOUND => {
                return Err(DownloadError::RemoteFileNotFound(filename.into()))
            }
            StatusCode::INTERNAL_SERVER_ERROR => return Err(DownloadError::ServerError),
            other_status => return Err(DownloadError::Unknown(other_status)),
        };

        let mut stream = response.bytes_stream();

        while let Some(item) = stream.next().await {
            if cancel_token.is_cancelled() {
                file.flush().await.map_err(DownloadError::IoError)?;

                return Ok(());
            }

            let chunk = match item {
                Ok(chunk) => chunk.to_vec(),
                Err(e) => return Err(DownloadError::Http(e)),
            };

            let mut limited_buffer = limiter.clone().limit(Cursor::new(&chunk));
            let mut target_buffer = vec![];

            limited_buffer.read_to_end(&mut target_buffer).await?;

            let mut output = Bytes::from(target_buffer);

            // Write chunk to file
            match file.write_all(&mut output).await {
                Ok(_) => (),
                Err(e) => return Err(DownloadError::IoError(e)),
            };

            file.flush().await.map_err(DownloadError::IoError)?;

            // Increment download size
            downloaded += chunk.len() as u64;

            // calculate download speed
            let elapsed_time = start_time.elapsed().as_secs_f64();
            let download_speed = downloaded as f64 / elapsed_time;

            // calculate remaining time until download is done
            let remaining_time = (meta.length as f64 - downloaded as f64) / download_speed;

            // Send notification to frontend
            let progress = DownloadProgress {
                current: &human_bytes(downloaded as f64),
                total: &human_bytes(meta.length as f64),
                percentage: ((downloaded as f64 / meta.length as f64) * 100.0).round(),
                speed: &human_bytes(download_speed.floor()),
                eta: &Self::format_time(remaining_time),
            };

            tracker.update(progress);
        }

        file.flush().await.map_err(DownloadError::IoError)?;

        tokio::fs::rename(cache_file, item.destination)
            .await
            .map_err(DownloadError::IoError)?;

        tracker.finish();

        // item.finalize().await?;

        Ok(())
    }

    fn format_time(duration: f64) -> String {
        let duration = Duration::seconds(duration as i64);

        let days = duration.num_days();
        let hours = duration.num_hours() % 24;
        let minutes = duration.num_minutes() % 60;
        let seconds = duration.num_seconds() % 60;

        let mut formatted_time = String::new();

        let time_units = [
            ("day", days),
            ("hour", hours),
            ("minute", minutes),
            ("second", seconds),
        ];

        let mut first = true;

        for (unit, value) in time_units.iter() {
            if *value > 0 {
                if !first {
                    formatted_time.push_str(", ");
                }

                formatted_time.push_str(&format!("{} {}", value, unit));

                if *value > 1 {
                    formatted_time.push('s');
                }

                first = false;
            }
        }

        if formatted_time.is_empty() {
            formatted_time.push_str("0 seconds");
        }

        formatted_time
    }

    async fn is_complete(
        cache: &Path,
        filename: &str,
        total_size: u64,
    ) -> Result<bool, DownloadError> {
        let path = cache.join(filename);

        if path.exists() {
            let file = File::open(&path).await.map_err(DownloadError::IoError)?;

            let file_size = match file.metadata().await {
                Ok(meta) => meta.len(),
                Err(e) => return Err(DownloadError::IoError(e)),
            };

            return Ok(file_size == total_size);
        }

        Ok(false)
    }

    pub async fn get_meta(client: &Client, source: &str) -> Result<RequestMeta, DownloadError> {
        let mut meta = RequestMeta::default();

        let response = client
            .head(source)
            .header("Accept-Encoding", "*")
            .send()
            .await
            .map_err(DownloadError::Http)?;

        let headers = response.headers();

        let etag = headers.get("etag").unwrap().to_str().unwrap();

        meta.etag = etag.into();

        if let Some(value) = headers.get("content-length") {
            let length = match value.is_empty() {
                true => 0,
                false => value.to_str().unwrap().parse::<u64>().unwrap(),
            };

            meta.length = length;

            return Ok(meta);
        }

        if let Some(value) = headers.get("entity-length") {
            let length = match value.is_empty() {
                true => 0,
                false => value.to_str().unwrap().parse::<u64>().unwrap(),
            };

            meta.length = length;

            return Ok(meta);
        };

        Ok(meta)
    }
}

pub struct DownloadItem<'a> {
    pub source: &'a str,
    pub destination: &'a str,
    pub total_size: u64,
    pub hash: &'a str,
}

impl<'a> DownloadItem<'a> {
    pub fn new(source: &'a str, destination: &'a str, total_size: u64, hash: &'a str) -> Self {
        Self {
            source,
            destination,
            total_size,
            hash,
        }
    }

    pub fn validate(&self) -> Result<(), HashError> {
        Hasher::validate(self.destination, self.hash)
    }

    pub async fn finalize(&self) -> Result<(), DownloadError> {
        tokio::fs::rename(self.source, self.destination)
            .await
            .map_err(DownloadError::IoError)
    }
}

use async_speed_limit::Limiter;
use parking_lot::Mutex;
use reqwest::{Client, ClientBuilder};
use tauri::async_runtime::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::{
    errors::{DownloadError, RegistryError},
    models::{Channel, ManifestData},
    registry::{PaliaInstallation, Registry},
    tokens::Tokens,
};
pub struct AppState {
    pub tokens: Mutex<Tokens>,
    pub registry: Mutex<Registry>,
    pub client: Client,
    pub limiter: Limiter,
    pub channel: Mutex<Option<Channel>>,
    pub cancellation_token: Mutex<CancellationToken>,
    pub download_task: Mutex<Option<JoinHandle<Result<(), DownloadError>>>>,
    pub installation: Mutex<Option<PaliaInstallation>>,
    pub manifest: Option<ManifestData>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            tokens: Mutex::new(Tokens::new()),
            registry: Mutex::new(Registry::new()),
            client: ClientBuilder::new()
                .use_rustls_tls()
                .gzip(true)
                .build()
                .unwrap(),
            limiter: <Limiter>::new(f64::INFINITY),
            channel: Mutex::new(None),
            cancellation_token: Mutex::new(CancellationToken::new()),
            download_task: Mutex::new(None),
            installation: Mutex::new(None),
            manifest: None,
        }
    }

    pub fn load_registry(&self) -> Result<PaliaInstallation, RegistryError> {
        let mut guard = self.registry.lock();

        guard.load()?;

        let installation = guard.entry().to_owned();

        *self.installation.lock() = Some(installation);

        Ok(installation)
    }

    pub fn update_channel(&self, channel: Channel) {
        let mut guard = self.channel.lock();

        *guard = Some(channel);
    }

    pub fn get_channel(&self) -> Option<Channel> {
        let guard = self.channel.lock();

        guard.clone()
    }

    pub fn get_token(&self) -> CancellationToken {
        let guard = self.cancellation_token.lock();

        guard.child_token()
    }
}

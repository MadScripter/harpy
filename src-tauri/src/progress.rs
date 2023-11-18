use serde::Serialize;
use tauri::Window;

pub struct ProgressTracker {
    window: Window,
}

impl ProgressTracker {
    pub fn new(window: Window) -> Self {
        Self { window }
    }

    pub fn start(&self) {
        self.window.emit("download:init", ()).unwrap();
    }

    pub fn update<T: Serialize + Clone>(&self, progress: T) {
        self.window.emit("download:progress", progress).unwrap();
    }

    pub fn finish(&self) {
        self.window.emit("download:done", ()).unwrap();
    }
}

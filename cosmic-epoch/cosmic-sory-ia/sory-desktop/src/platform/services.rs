// SPDX-License-Identifier: GPL-3.0-only

#[derive(Debug, Clone, Default)]
pub struct PlatformServices;

impl PlatformServices {
    pub fn notifications(&self) -> NotificationService {
        NotificationService
    }
    pub fn clipboard(&self) -> ClipboardService {
        ClipboardService
    }
    pub fn files(&self) -> FileManagerService {
        FileManagerService
    }
    pub fn terminal(&self) -> TerminalService {
        TerminalService
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NotificationService;

impl NotificationService {
    pub fn notify(&self, title: &str, body: &str) {
        log::info!("notification: {title}: {body}");
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ClipboardService;

impl ClipboardService {
    pub fn copy_text(&self, text: &str) {
        log::info!("copy text requested: {} bytes", text.len());
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FileManagerService;

impl FileManagerService {
    pub fn reveal(&self, path: &str) {
        log::info!("reveal path requested: {path}");
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TerminalService;

impl TerminalService {
    pub fn open_at(&self, path: &str) {
        log::info!("open terminal requested: {path}");
    }
}

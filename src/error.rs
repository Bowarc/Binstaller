#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Ui related error: {0}")]
    Ui(#[from] eframe::Error),

    #[error("Download related error: {0}")]
    Download(#[from] DownloaderError),
}

#[derive(Debug, thiserror::Error)]
pub enum DownloaderError {}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Ui related error: {0}")]
    Ui(#[from] eframe::Error),
}

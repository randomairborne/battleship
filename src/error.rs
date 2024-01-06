#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Ship placement overflowed or overlapped!")]
    InvalidShipState,
    #[error("I/O error")]
    IoFailed(#[from] tokio::io::Error),
    #[error("Failed to parse int: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

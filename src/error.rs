#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Ship placement overflowed or overlapped!")]
    InvalidShipState,
    #[error("Board rendering output failed")]
    StdoutWriteFailed(#[from] crossterm::ErrorKind),
}

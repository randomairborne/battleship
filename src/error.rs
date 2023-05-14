#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Ship placement overflowed or overlapped!")]
    InvalidShipState,
    #[error("Board rendering output failed")]
    StdoutWriteFailed(#[from] crossterm::ErrorKind),
    #[error("Socket address parsing error: {0}")]
    SocketAddrParseError(#[from] std::net::AddrParseError),
    #[error("Failed to parse int: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

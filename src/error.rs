#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Ship placement overflowed or overlapped!")]
    InvalidShipState,
}

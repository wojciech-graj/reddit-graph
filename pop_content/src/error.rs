#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Send error")]
    SendError,
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError{
    #[error("Data Collection Error: {0}")]
    CollectionError(String),

    #[error("Channel send error: {0}")]
    ChannelSendError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}
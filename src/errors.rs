use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImapAttachmentDaemonError {
    #[error("Failed to load configuration")]
    ConfigError(#[from] envy::Error),
    #[error("IMAP error: {0}")]
    ImapError(#[from] imap::error::Error),
    #[error("Failed to create directory")]
    IoError(#[from] std::io::Error),
    #[error("Failed to load .env file")]
    DotenvError(#[from] dotenvy::Error),
    #[error("Could not find UID in message")]
    UidMissing,
    #[error("Could not find body in message")]
    BodyMissing,
    #[error("Could not find header in message")]
    HeaderMissing,
    #[error("Could not find attachment filename")]
    FilenameMissing,
    #[error("Extension missing")]
    ExtensionMissing,
    #[error("Failed to convert extension to string")]
    ExtensionConvertError,
    #[error("Could not find sender in email")]
    SenderMissing,
    #[error("Could not find destinations in email")]
    DestinationsMissing,
    #[error("Could not send message to channel")]
    SendError(#[from] std::sync::mpsc::SendError<()>),
    #[error("Could not receive message from channel")]
    ReceiveError(#[from] std::sync::mpsc::RecvError),
    #[error("Could not parse email")]
    ParsingError,
}

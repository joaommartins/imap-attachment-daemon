use thiserror::Error;

/// Error type for the `ImapAttachmentDaemon`.
#[derive(Error, Debug)]
pub enum ImapAttachmentDaemonError {
    /// Error when loading configuration from environment variables fails.
    #[error("Failed to load configuration")]
    ConfigError(#[from] envy::Error),
    /// Error when connection with the IMAP server fails.
    #[error("IMAP error: {0}")]
    ImapError(#[from] imap::error::Error),
    /// Error when creating the attachments directory fails.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    /// Error when loading environment variables from the `.env` file fails.
    #[error("Failed to load .env file")]
    DotenvError(#[from] dotenvy::Error),
    /// Error when expected UID not in message.
    #[error("Could not find UID in message")]
    UidMissing,
    /// Error when expected body not in message.
    #[error("Could not find body in message")]
    BodyMissing,
    /// Error when expected header not in message.
    #[error("Could not find header in message")]
    HeaderMissing,
    /// Error when attachment does not have filename.
    #[error("Could not find attachment filename")]
    FilenameMissing,
    /// Error when attachment filename does not have extension.
    #[error("Extension missing")]
    ExtensionMissing,
    /// Error when attachment extension cannot be converted to string.
    #[error("Failed to convert extension to string")]
    ExtensionConvertError,
    /// Error when sender address is missing in email header.
    #[error("Could not find sender in email")]
    SenderMissing,
    /// Error when destination addresses are missing in email header.
    #[error("Could not find destinations in email")]
    DestinationsMissing,
    /// Error when sending message to channel fails.
    #[error("Could not send message to channel")]
    SendError(#[from] std::sync::mpsc::SendError<()>),
    /// Error when receiving message from channel fails.
    #[error("Could not receive message from channel")]
    ReceiveError(#[from] std::sync::mpsc::RecvError),
    /// Error when parsing email fails.
    #[error("Could not parse email")]
    ParsingError,
    /// Error when email is not in expected format.
    #[error(
        "Failed to create directory: {msg:?}. You may need to set the `CWA_ATTACHMENTS_DIR` environment variable."
    )]
    DirectoryCreationError {
        /// Error message.
        msg: String,
        /// error source.
        source: std::io::Error,
    },
}

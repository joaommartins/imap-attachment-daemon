//! # Imap Attachment Daemon
//!
//! The Imap Attachment Daemon is a Rust application that listens for new emails on an IMAP server and processes
//!  them based on the sender and recipient addresses. Attachments from whitelisted senders are saved to a specified
//!  directory and the emails are moved to trash, while other emails are kept unread.

mod errors;
pub(crate) mod imap_ops;
pub(crate) mod mail_parsing;
pub(crate) mod mail_searching;
mod models;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

pub use errors::ImapAttachmentDaemonError;
use imap_ops::{open_session, process_idle_update};
use log::log_enabled;
use mail_searching::{idle_update_email_search, startup_email_search};
use models::AppConfig;

use env_logger::{Builder, Env};

/// Initialises the application by setting up environment variables, logging, configuration,
/// attachments directory, and IMAP session.
///
/// # Returns
///
/// * `Ok(AppConfig)` - Application configuration.
/// * `Err(ImapAttachmentDaemonError)` - If any step in the initialisation process fails.
///
/// # Errors
///
/// This function will return an error in the following cases:
///
/// * If loading environment variables from the `.env` file fails.
/// * If initialising the logging system fails.
/// * If reading the configuration from environment variables fails.
/// * If creating the attachments directory fails.
/// * If connecting to the IMAP server fails.
/// * If logging into the IMAP server fails.
/// * If selecting the "INBOX" folder on the IMAP server fails.
pub fn init_app() -> Result<AppConfig, ImapAttachmentDaemonError> {
    // Load environment variables from .env file
    if dotenvy::dotenv().is_err() {
        log::warn!("No .env file found, using environment variables");
    } else {
        log::info!("Loaded environment variables from .env file");
    };

    // Initialize logging, default to info level
    let env = Env::new().filter_or("RUST_LOG", "info");
    Builder::from_env(env).init();

    let config = envy::prefixed("CWA_").from_env::<AppConfig>()?;

    // Create attachments directory if it doesn't exist
    std::fs::create_dir_all(&config.attachments_dir).map_err(|err| {
        ImapAttachmentDaemonError::DirectoryCreationError {
            source: err,
            msg: config.attachments_dir.clone(),
        }
    })?;

    Ok(config)
}

/// Fetches and processes emails from the IMAP server.
///
/// This function will start the daemon, fetching emails from the IMAP server and processing them. It will then enter
/// IDLE mode to listen for changes in the mailbox and process new emails as they arrive. All emails are checked for
/// destination address and sender address, with only emails from the whitelist being processed. Attachments are saved
/// to the specified directory and the email is marked as read or moved to the trash.
///
/// If an email does not contain any attachments, it will be marked as unread.
///
/// # Arguments
/// * `config` - The application configuration.
///
/// # Returns
/// * `Ok(())` - If the emails are successfully fetched and processed.
/// * `Err(ImapAttachmentDaemonError)` - If any step in the process fails.
///
/// # Errors
/// This function will return an error in the following cases:
/// * If the initial email search on startup fails.
/// * If the IDLE mode fails to process an update.
/// * If the email search after an update fails.
/// * If the email parsing and processing fails.
/// * If moving an email to the trash fails.
/// * If marking an email as unread fails.
/// * If logging out of the IMAP session fails.
/// * If the channel receiver fails to receive a message.
/// * If the channel sender fails to send a message.
/// * If the email UID is missing.
/// * If the email body is missing.
///
pub fn run_daemon(config: &AppConfig) -> Result<(), ImapAttachmentDaemonError> {
    log::info!(
        "Daemon started on account: {}",
        config.target_address.as_ref().unwrap_or(&config.username)
    );
    log::info!(
        "Emails whitelist: {}",
        config.whitelist.iter().cloned().collect::<Vec<String>>().join(", ")
    );

    // Check for unread emails on startup
    startup_email_search(config)?;

    let (sender, receiver): (Sender<u32>, Receiver<u32>) = channel();

    let mut idle_imap_session = open_session(config)?;
    if log_enabled!(log::Level::Debug) {
        idle_imap_session.debug = true;
    }

    // Spawn a thread for IDLE mode
    let _ = thread::spawn(move || -> Result<(), ImapAttachmentDaemonError> {
        loop {
            // Enter IDLE mode
            let _ = idle_imap_session
                .idle()
                .timeout(Duration::from_secs(300))
                .wait_while(|response| process_idle_update(response, &sender))?;
        }
    });

    loop {
        let response = receiver.recv()?;
        idle_update_email_search(response, config)?;
    }
}

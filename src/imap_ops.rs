use std::collections::HashSet;
use std::sync::mpsc::Sender;

use crate::{errors::ImapAttachmentDaemonError, AppConfig};

use imap::types::{Fetches, UnsolicitedResponse};
use imap::{ImapConnection, Session};
use secrecy::ExposeSecret;

pub(crate) fn open_session(config: &AppConfig) -> Result<Session<Box<dyn ImapConnection>>, ImapAttachmentDaemonError> {
    let mut session = imap::ClientBuilder::new(&config.imap_server, 993)
        .connect()?
        .login(&config.username, config.password.expose_secret())
        .map_err(|e| e.0)?;
    let _ = session.select("INBOX")?;
    Ok(session)
}
pub(crate) fn move_email_to_trash(
    uid: &str,
    imap_session: &mut Session<Box<dyn ImapConnection>>,
) -> Result<(), ImapAttachmentDaemonError> {
    imap_session.uid_mv(uid, "Trash")?;
    log::debug!("Moved email to Trash");
    Ok(())
}

pub(crate) fn mark_email_as_unread(
    uid: &str,
    imap_session: &mut Session<Box<dyn ImapConnection>>,
) -> Result<(), ImapAttachmentDaemonError> {
    let _ = imap_session.uid_store(uid, "-FLAGS \\Seen")?;
    log::debug!("Email marked as unread");
    Ok(())
}

/// A convenience function to always cause the IDLE handler to exit on any change.
/// Always returns false to release the thread so a new connection can be made, workaround for
/// <https://github.com/jonhoo/rust-imap/issues/300>
pub(crate) fn process_idle_update(response: UnsolicitedResponse, sender: &Sender<u32>) -> bool {
    match response {
        UnsolicitedResponse::Fetch { id, attributes } => {
            // If the email is not marked as seen, send the ID to the main thread
            if attributes.iter().clone().any(|attr| {
                matches!(attr, imap::types::AttributeValue::Flags(vals) if
                !vals.contains(&std::borrow::Cow::Borrowed("\\Seen")))
            }) {
                sender.send(id).expect("Send failed, channel is closed");
            }
        }
        // New emails are marked as EXISTS, without any flags in the unsolicited response
        // It also comes as a UnsolicitedResponse::Recent, but we don't need to handle it because the id is not useful
        UnsolicitedResponse::Exists(id) => {
            sender.send(id).expect("Send failed, channel is closed");
        }
        UnsolicitedResponse::Bye {
            ref code,
            ref information,
        } => {
            log::error!("Server disconnected: {:?} {:?}", code, information);
        }
        _ => {}
    }
    false
}

pub(crate) fn imap_search(
    search_criteria: impl AsRef<str>,
    imap_session: &mut Session<Box<dyn ImapConnection>>,
) -> Result<HashSet<u32>, ImapAttachmentDaemonError> {
    imap_session.search(search_criteria).map_err(Into::into)
}

pub(crate) fn imap_fetch_uids(
    sequence_set: impl AsRef<str>,
    imap_session: &mut Session<Box<dyn ImapConnection>>,
) -> Result<Vec<String>, ImapAttachmentDaemonError> {
    imap_fetch_by_seq(sequence_set, &"UID", imap_session)?
        .iter()
        .map(|query_response| {
            query_response
                .uid
                .ok_or(ImapAttachmentDaemonError::UidMissing)
                .map(|uid| uid.to_string())
        })
        .collect::<Result<Vec<String>, ImapAttachmentDaemonError>>()
}

pub(crate) fn imap_fetch_rfc822(
    sequence_set: impl AsRef<str>,
    imap_session: &mut Session<Box<dyn ImapConnection>>,
) -> Result<Fetches, ImapAttachmentDaemonError> {
    imap_fetch_by_uid(sequence_set, &"RFC822", imap_session)
}

pub(crate) fn imap_fetch_headers(
    sequence_set: impl AsRef<str>,
    imap_session: &mut Session<Box<dyn ImapConnection>>,
) -> Result<Fetches, ImapAttachmentDaemonError> {
    imap_fetch_by_uid(sequence_set, &"RFC822.HEADER", imap_session)
}

fn imap_fetch_by_seq(
    sequence_set: impl AsRef<str>,
    query: &impl AsRef<str>,
    imap_session: &mut Session<Box<dyn ImapConnection>>,
) -> Result<Fetches, ImapAttachmentDaemonError> {
    imap_session.fetch(sequence_set, query).map_err(Into::into)
}

fn imap_fetch_by_uid(
    sequence_set: impl AsRef<str>,
    query: &impl AsRef<str>,
    imap_session: &mut Session<Box<dyn ImapConnection>>,
) -> Result<Fetches, ImapAttachmentDaemonError> {
    imap_session.uid_fetch(sequence_set, query).map_err(Into::into)
}

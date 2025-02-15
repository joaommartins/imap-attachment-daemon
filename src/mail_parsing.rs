use crate::imap_ops::{mark_email_as_unread, move_email_to_trash};
use crate::models::MessageMetadata;
use crate::{AppConfig, ImapAttachmentDaemonError};
use imap::types::{Fetch, Fetches};
use mail_parser::{Message, MessageParser, MessagePart, MimeHeaders};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use imap::{ImapConnection, Session};

pub(crate) fn parse_and_process_emails(
    config: &AppConfig,
    fetched_emails: &Fetches,
    imap_session: &mut Session<Box<dyn ImapConnection>>,
) -> Result<(), ImapAttachmentDaemonError> {
    for message in fetched_emails.iter() {
        let uid = message.uid.ok_or(ImapAttachmentDaemonError::UidMissing)?.to_string();
        let parsed_email = parse_body(message)?;
        let message_metadata = extract_descriptors(&parsed_email)?;
        let saved_attachments = parsed_email
            .attachments()
            .map(|x| check_and_save_attachment(x, &message_metadata, config))
            .collect::<Result<Vec<bool>, ImapAttachmentDaemonError>>()?;
        if saved_attachments.iter().any(|&saved| saved) {
            move_email_to_trash(&uid, imap_session)?;
            continue;
        }
        log::info!(
            "No accepted attachments found in email {}, marking as unread",
            format_email_metadata_message(&message_metadata)
        );
        mark_email_as_unread(&uid, imap_session)?;
    }
    log::info!("All emails processed, waiting for new emails");
    Ok(())
}

pub(crate) fn parse_body<'a>(fetch: &'a Fetch) -> Result<Message<'a>, ImapAttachmentDaemonError> {
    let raw_bytes = fetch.body().ok_or(ImapAttachmentDaemonError::BodyMissing)?;
    MessageParser::default()
        .parse(raw_bytes)
        .ok_or(ImapAttachmentDaemonError::ParsingError)
}

pub(crate) fn filter_messages_by_source_and_whitelist(
    messages_headers: &Fetches,
    config: &AppConfig,
) -> Result<Vec<u32>, ImapAttachmentDaemonError> {
    let mut accepted_messages = Vec::new();
    for message_header in messages_headers.iter() {
        let message = parse_header(message_header)?;
        let message_descriptors = extract_descriptors(&message)?;
        if message_descriptors
            .to()
            .contains(&config.target_address.as_ref().unwrap_or(&config.username).as_str())
            && config.whitelist.contains(message_descriptors.from())
        {
            accepted_messages.push(message_header.uid.ok_or(ImapAttachmentDaemonError::UidMissing)?);
        }
    }
    Ok(accepted_messages)
}

pub(crate) fn parse_header<'a>(fetch: &'a Fetch) -> Result<Message<'a>, ImapAttachmentDaemonError> {
    let raw_bytes = fetch.header().ok_or(ImapAttachmentDaemonError::HeaderMissing)?;
    MessageParser::default()
        .parse(raw_bytes)
        .ok_or(ImapAttachmentDaemonError::ParsingError)
}

fn format_email_metadata_message(message_metadata: &MessageMetadata) -> String {
    format!(
        "from {} {}",
        message_metadata.from(),
        if message_metadata.subject().is_some() {
            format!("with subject {:?}", message_metadata.subject().unwrap())
        } else {
            String::new()
        }
    )
}

pub(crate) fn check_and_save_attachment(
    part: &MessagePart,
    message_metadata: &MessageMetadata,
    config: &AppConfig,
) -> Result<bool, ImapAttachmentDaemonError> {
    if part.is_message() {
        return Ok(false);
    }

    let filename = part
        .attachment_name()
        .ok_or(ImapAttachmentDaemonError::FilenameMissing)?;
    let filename = PathBuf::from(filename.to_lowercase());
    let extension = filename
        .extension()
        .ok_or(ImapAttachmentDaemonError::ExtensionMissing)?
        .to_str()
        .ok_or(ImapAttachmentDaemonError::ExtensionConvertError)?;
    if !config.accepted_file_types.contains(extension) {
        log::debug!("Unsupported file type: {}", extension);
        return Ok(false);
    }
    let filepath = Path::new(&config.attachments_dir).join(&filename);
    let mut file = File::create(&filepath)?;
    file.write_all(part.contents())?;
    log::info!(
        "Attachment {:?} in email {} saved at: {:?}",
        filename,
        format_email_metadata_message(message_metadata),
        filepath
    );
    Ok(true)
}

pub(crate) fn extract_descriptors<'x>(message: &'x Message) -> Result<MessageMetadata<'x>, ImapAttachmentDaemonError> {
    let from = message
        .from()
        .ok_or(ImapAttachmentDaemonError::SenderMissing)?
        .first()
        .ok_or(ImapAttachmentDaemonError::SenderMissing)?
        .address()
        .ok_or(ImapAttachmentDaemonError::SenderMissing)?;
    let destinations = message
        .to()
        .ok_or(ImapAttachmentDaemonError::DestinationsMissing)?
        .as_list()
        .ok_or(ImapAttachmentDaemonError::DestinationsMissing)?
        .iter()
        .map(|x| x.address().ok_or(ImapAttachmentDaemonError::DestinationsMissing))
        .collect::<Result<Vec<&str>, ImapAttachmentDaemonError>>()?;

    let subject = message.subject();
    Ok(MessageMetadata::new(from, destinations, subject))
}

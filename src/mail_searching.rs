use crate::errors::ImapAttachmentDaemonError;
use crate::imap_ops::{imap_fetch_headers, imap_fetch_rfc822, imap_fetch_uids, imap_search, open_session};
use crate::mail_parsing::{filter_messages_by_source_and_whitelist, parse_and_process_emails};
use crate::AppConfig;

use imap::types::Fetches;
use imap::{ImapConnection, Session};

use std::collections::HashSet;

pub(crate) fn startup_email_search(config: &AppConfig) -> Result<(), ImapAttachmentDaemonError> {
    let mut imap_session = open_session(config)?;
    log::info!("Checking for unread emails at startup");

    // Search for unread emails
    let search_result = whitelist_imap_search(&mut imap_session, config)?;
    if search_result.is_empty() {
        log::info!("No unread emails from whitelist found, waiting for new emails");
        return Ok(());
    }

    log::info!("Found {} unread emails from whitelist, processing", search_result.len());
    let bodies = fetch_bodies_by_seq(search_result, &mut imap_session)?;
    parse_and_process_emails(config, &bodies, &mut imap_session)?;
    Ok(())
}

pub(crate) fn idle_update_email_search(response: u32, config: &AppConfig) -> Result<(), ImapAttachmentDaemonError> {
    let mut imap_session = open_session(config)?;
    log::info!("Change detected in inbox, checking for new emails");
    let search_result = recent_unseen_imap_search(&mut imap_session)?;
    if !search_result.contains(&response) {
        log::info!("Email isn't unread and recent, waiting for new emails");
        return Ok(());
    }
    let query = search_result
        .into_iter()
        .map(|arg0: u32| arg0.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let fetched_uids = imap_fetch_uids(query, &mut imap_session)?;
    let messages_headers = fetch_headers(fetched_uids, &mut imap_session)?;
    let messages_to_process = filter_messages_by_source_and_whitelist(&messages_headers, config)?;
    let bodies = fetch_bodies_by_uid(messages_to_process, &mut imap_session)?;
    parse_and_process_emails(config, &bodies, &mut imap_session)?;
    imap_session.logout()?;
    Ok(())
}

fn fetch_bodies_by_uid(
    search_result: impl IntoIterator<Item = u32>,
    imap_session: &mut Session<Box<dyn ImapConnection>>,
) -> Result<Fetches, ImapAttachmentDaemonError> {
    let query = search_result
        .into_iter()
        .map(|arg0: u32| arg0.to_string())
        .collect::<Vec<String>>()
        .join(",");
    imap_fetch_rfc822(query, imap_session).map_err(Into::into)
}

fn fetch_bodies_by_seq(
    search_result: impl IntoIterator<Item = u32>,
    imap_session: &mut Session<Box<dyn ImapConnection>>,
) -> Result<Fetches, ImapAttachmentDaemonError> {
    let query = search_result
        .into_iter()
        .map(|arg0: u32| arg0.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let fetched_uids = imap_fetch_uids(&query, imap_session)?;
    imap_fetch_rfc822(fetched_uids.join(","), imap_session).map_err(Into::into)
}

fn fetch_headers(
    query: impl IntoIterator<Item = String>,
    imap_session: &mut Session<Box<dyn ImapConnection>>,
) -> Result<Fetches, ImapAttachmentDaemonError> {
    imap_fetch_headers(query.into_iter().collect::<Vec<_>>().join(","), imap_session)
}

fn recent_unseen_imap_search(
    imap_session: &mut Session<Box<dyn ImapConnection>>,
) -> Result<HashSet<u32>, ImapAttachmentDaemonError> {
    // Used when responding to an idle update, must not filter by whitelist or target address, because IMAP search
    // indexing is too slow and will return empty results.
    imap_search("RECENT UNSEEN", imap_session)
}

fn whitelist_imap_search(
    imap_session: &mut Session<Box<dyn ImapConnection>>,
    config: &AppConfig,
) -> Result<HashSet<u32>, ImapAttachmentDaemonError> {
    let search_criteria = generate_search_criteria(config);
    imap_search(&search_criteria, imap_session)
}

fn generate_search_criteria(config: &AppConfig) -> String {
    let from_criteria = config
        .whitelist
        .iter()
        .map(|addr| format!("FROM {addr:?}"))
        .collect::<Vec<String>>()
        .join(" ");
    format!(
        "UNSEEN TO {:?} ({} {})",
        config.target_address.as_ref().unwrap_or(&config.username),
        if config.whitelist.len() > 1 { "OR" } else { "" },
        from_criteria
    )
}

#[cfg(test)]
#[path = "test_mail_searching.rs"]
mod test_mail_searching;

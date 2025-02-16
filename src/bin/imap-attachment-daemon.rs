//! The main entry point for the `imap-attachment-daemon` binary.

use imap_attachment_daemon::{init_app, run_daemon};

fn main() -> anyhow::Result<()> {
    // Initialize the application
    let app_data = init_app()?;

    // Start the daemon
    run_daemon(&app_data)?;

    Ok(())
}

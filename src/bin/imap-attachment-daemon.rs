use imap_attachment_daemon::{init_app, run_daemon};
use std::process::ExitCode;

fn main() -> anyhow::Result<ExitCode> {
    // Initialize the application
    let app_data = init_app()?;

    // Start the daemon
    // Fetch and process emails
    run_daemon(&app_data)?;

    Ok(ExitCode::SUCCESS)
}

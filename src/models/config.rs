use std::collections::BTreeSet;

use secrecy::SecretString;
use serde::Deserialize;

// Default accepted file types for attachments. Mirrors CWA accepted file types.
fn default_accepted_file_types() -> BTreeSet<String> {
    BTreeSet::from_iter([
        "azw".to_string(),
        "azw3".to_string(),
        "azw4".to_string(),
        "mobi".to_string(),
        "cbz".to_string(),
        "cbr".to_string(),
        "cb7".to_string(),
        "cbc".to_string(),
        "chm".to_string(),
        "djvu".to_string(),
        "docx".to_string(),
        "epub".to_string(),
        "fb2".to_string(),
        "fbz".to_string(),
        "html".to_string(),
        "htmlz".to_string(),
        "lit".to_string(),
        "lrf".to_string(),
        "odt".to_string(),
        "pdf".to_string(),
        "prc".to_string(),
        "pdb".to_string(),
        "pml".to_string(),
        "rb".to_string(),
        "rtf".to_string(),
        "snb".to_string(),
        "tcr".to_string(),
        "txtz".to_string(),
    ])
}

fn default_attachments_dir() -> String {
    "/attachments".to_string()
}

// Application configuration extracted from environment variables.
#[derive(Deserialize, Debug, Clone, Default)]
pub struct AppConfig {
    pub imap_server: String,
    pub username: String,
    pub password: SecretString,
    pub target_address: Option<String>,
    #[serde(default)]
    pub whitelist: BTreeSet<String>,
    #[serde(default = "default_attachments_dir")]
    pub attachments_dir: String,
    #[serde(default = "default_accepted_file_types")]
    pub accepted_file_types: BTreeSet<String>,
}

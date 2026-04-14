pub use std::collections::HashMap;
pub use tracing::{debug, info, warn, error};
pub use serde::{Serialize, Deserialize};
pub use anyhow::anyhow;
pub use owo_colors::OwoColorize;

use strum::Display;
use std::path::PathBuf;

#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum Environment {
    Development,
    Production,
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `development` or `production`.",
                other
            )),
        }
    }
}

/// Intelligently finds the base path for config files.
/// In development (cargo run), it uses the project root.
/// In production (direct execution), it uses the folder containing the executable.
pub fn get_base_path() -> PathBuf {
    // Are we running via `cargo run`? If so, use the project root
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        return PathBuf::from(manifest_dir);
    }

    // Otherwise, we are in production. Use the directory the executable is in
    std::env::current_exe()
        .expect("Failed to determine executable path")
        .parent()
        .expect("Executable has no parent directory")
        .to_path_buf()
}
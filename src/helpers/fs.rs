//! File system helper utilities for Seyuna CLI.
//!
//! This module provides async functions for reading, writing, and loading configuration files,
//! as well as path construction helpers.

use crate::config::types::Config;
use crate::ui::default::UI_CONFIGURATION;
use anyhow::{Context, Result};
use serde_json::Value;
use std::path::{Path, PathBuf};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

/// Constructs a full path from a file name and output directory.
///
/// # Arguments
/// * `file_name` - The name of the file.
/// * `output_dir` - The directory where the file should be located.
///
/// # Returns
/// `Result<PathBuf>` - The full path including the filename.
pub fn create_path_from_file_name(file_name: &str, output_dir: &str) -> Result<PathBuf> {
    // Path to the output directory.
    let directory = Path::new(output_dir);

    // Check if file_name is empty
    if file_name.trim().is_empty() {
        return Err(anyhow::anyhow!("File name cannot be empty"));
    }

    // Check if output_dir is empty
    if output_dir.trim().is_empty() {
        return Err(anyhow::anyhow!("Output directory cannot be empty"));
    }

    // Path including the filename.
    let path = directory.join(file_name);

    // Return the full path.
    Ok(path)
}

/// Reads the contents of a file asynchronously and returns it as a `String`.
///
/// # Arguments
/// * `path` - The path to the file.
///
/// # Returns
/// `Result<String>` - The contents of the file.
pub async fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    // Return file the contents.
    fs::read_to_string(&path)
        .await
        .with_context(|| format!("Failed to read file: {:?}", path.as_ref()))
}

/// Saves content to a file asynchronously.
///
/// # Arguments
/// * `path` - The path to the file.
/// * `content` - The content to write as a byte vector.
///
/// # Returns
/// `Result<()>` - No value.
pub async fn save_file<P: AsRef<Path>>(path: P, content: &[u8]) -> Result<()> {
    let path_ref = path.as_ref();

    // Ensure the parent directory exists
    if let Some(parent) = path_ref.parent() {
        fs::create_dir_all(parent)
            .await
            .with_context(|| format!("Failed to create directory: {:?}", parent))?;
    }

    // Create a new file.
    let mut file = File::create(&path_ref)
        .await
        .with_context(|| format!("Failed to create file: {:?}", path_ref))?;

    // Write the given contents to the file.
    file.write_all(&content)
        .await
        .with_context(|| format!("Failed to write to file: {:?}", path_ref))?;

    Ok(())
}

/// Loads the user's Seyuna configuration from `seyuna.json` and merges it with the default configuration.
///
/// If the file is missing or invalid, prints an error and returns the error.
///
/// # Returns
/// `Result<Config>` - The Seyuna configuration object.
pub async fn load_seyuna_user_config() -> Result<Config> {
    // Find and read the 'seyuna.json' file in the current directory.
    read_file("seyuna.json")
        .await
        .context(
            "Could not find seyuna.json file. Please make sure the file exists in the directory.",
        )
        .and_then(|content| {
            // Parse the content as JSON.
            let json: Value =
                serde_json::from_str(&content).context("Failed to parse seyuna.json as JSON")?;

            // Map the parsed JSON to the Seyuna configuration type.
            let user_config: Config = serde_json::from_value(json)
                .context("seyuna.json file does not match the Seyuna configuration type.")?;

            // Load the default Seyuna configuration.
            let default_configuration = Config {
                license: None,
                ui: Some(UI_CONFIGURATION.clone()),
            };

            // Return the merged Seyuna configuration.
            Ok(default_configuration.merge(user_config))
        })
}

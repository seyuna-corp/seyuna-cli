use std::path::Path;
use std::time::{Duration, Instant};

use crate::{
    config::types::Config,
    helpers::{
        cli::{spinner_done, spinner_progress, spinner_start},
        fs::{create_path_from_file_name, load_seyuna_user_config, save_file},
    },
};
use anyhow::{Context, Result};
use clap::ArgMatches;
use lightningcss::{
    printer::PrinterOptions,
    stylesheet::{MinifyOptions, ParserOptions, StyleSheet},
    targets::{Browsers, Targets},
};
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc;
use tokio::time::sleep;

/// Entrypoint for the `compile` CLI command.
/// Loads the user's configuration and triggers CSS compilation.
///
/// # Arguments
/// * `_matches` - CLI argument matches (unused).
///
/// # Returns
/// * `Result<Config>` - The loaded configuration or an error.
pub async fn compile(matches: &ArgMatches) -> Result<Config> {
    // Load the user's configuration file asynchronously
    let config = load_seyuna_user_config().await?;

    // Compile CSS based on the loaded configuration
    compile_css(&config).await?;

    // If the --watch flag is set, start watching for changes to seyuna.json
    if matches.get_flag("watch") {
        println!("Seyuna watcher active...");

        // Create a channel for receiving file change events
        let (tx, mut rx) = mpsc::channel(1);

        // Initialize the file watcher with a callback that sends events to the channel
        let mut watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.blocking_send(res);
            },
            notify::Config::default(),
        )?;

        // Watch the seyuna.json file for changes (non-recursive)
        watcher.watch(Path::new("seyuna.json"), RecursiveMode::NonRecursive)?;

        // Track the last event time for debouncing
        let mut last_event = Instant::now();

        // Listen for file change events asynchronously
        while let Some(res) = rx.recv().await {
            match res {
                Ok(event) => {
                    // Only handle Modify events
                    if let EventKind::Modify(_) = event.kind {
                        // Debounce: ignore events within 100ms
                        if last_event.elapsed() > Duration::from_millis(100) {
                            // Wait a bit to ensure the file write is finished
                            sleep(Duration::from_millis(500)).await;

                            // Reload the user's configuration file asynchronously
                            let config = load_seyuna_user_config().await?;

                            compile_css(&config).await?;
                            last_event = Instant::now();
                        }
                    }
                }
                // Print any errors from the watcher
                Err(e) => println!("Watch error: {:?}", e),
            }
        }
    }

    // Return the loaded configuration
    Ok(config)
}

/// Compiles CSS based on the provided configuration.
///
/// This function calculates the scaling for responsive font sizing between
/// two breakpoints (_2XL and _4XL), which are used for generating the
/// required media queries for the upscale in CSS.
///
/// # Arguments
/// * `config` - Reference to the loaded Seyuna configuration.
///
/// # Returns
/// * `Result<()>` - No value.
pub async fn compile_css(config: &Config) -> Result<()> {
    // Start spinner
    let spinner = spinner_start("Compiling CSS").await;

    // Extract the UI configuration from the main config
    let ui_config = config
        .ui
        .clone()
        .context("UI configuration missing in Seyuna config")?;

    // CSS upscale string
    let upscale_string = css_upscale(&config).unwrap();

    // CSS variables string
    let variables_string = css_variables(&config).unwrap();

    // Merge the css strings into a static string before passing it to the Stylesheet
    let merged_css_string: &'static str =
        Box::leak((variables_string + &upscale_string).into_boxed_str());

    // Parse the merged css as Stylesheet
    let mut stylesheet = StyleSheet::parse(&merged_css_string, ParserOptions::default())
        .with_context(|| {
            format!(
                "Failed to parse generated CSS content as a StyleSheet. \
                Content length: {}. First 200 chars: {:?}",
                merged_css_string.len(),
                &merged_css_string[..merged_css_string.len().min(200)]
            )
        })?;

    // Minify the stylesheet
    stylesheet.minify(MinifyOptions::default()).context(
        "Failed to minify StyleSheet. The CSS may contain invalid or unsupported syntax.",
    )?;

    // Define file path
    let file_path =
        create_path_from_file_name("seyuna-global.css", &ui_config.output_dir.unwrap())?;

    // Convert stylesheet in to formatted css code string
    let css_code = stylesheet.to_css(PrinterOptions {
        targets: Targets {
            browsers: Some(Browsers {
                chrome: Some(80),
                ..Browsers::default()
            }),
            ..Targets::default()
        },
        ..PrinterOptions::default()
    })?;

    // Display progress
    spinner_progress(&spinner, &format!("Saving {:?}...", &file_path)).await;

    // Save seyuna-global.css file
    save_file(file_path, css_code.code.as_bytes()).await?;

    // Display completed progress
    spinner_done(&spinner, "Successfully compiled Seyuna!").await;

    Ok(())
}

pub fn css_upscale(config: &Config) -> Result<String> {
    // Extract the UI configuration from the main config
    let ui_config = config
        .ui
        .clone()
        .context("UI configuration missing in Seyuna config")?;

    // Get the viewport breakpoints from the UI config
    let viewport_breakpoints = ui_config
        .breakpoints
        .context("Breakpoints missing in UI configuration")?
        .viewport;

    // Get the starting breakpoint value for the upscale (_2XL)
    let upscale_starting_point = viewport_breakpoints
        .get(&super::types::Breakpoint::_2XL)
        .context("Missing _2XL breakpoint in viewport configuration")?
        .as_f64()
        .context("_2XL breakpoint value is not a valid number")?;

    // Get the ending breakpoint value for the upscale (_4XL)
    let upscale_ending_point = viewport_breakpoints
        .get(&super::types::Breakpoint::_4XL)
        .context("Missing _4XL breakpoint in viewport configuration")?
        .as_f64()
        .context("_4XL breakpoint value is not a valid number")?;

    // The base factor for the upscale (also referred to as the "root font-size" - 1rem)
    let base = 1.0;

    // Calculate the scaling factor: how much the font size should scale between the breakpoints
    let scaling_factor = upscale_ending_point / upscale_starting_point - base;

    // Calculate the increment factor: the number of 100px steps between the breakpoints
    let increment_factor = (upscale_ending_point - upscale_starting_point) / 100.0;

    // Calculate the increment step value: how much the font size should change per 100px of viewport width
    let increment_value = scaling_factor / increment_factor;

    // Define a mutable variable to keep track of the current base when looping through it
    let mut current_base = base;

    // CSS result string
    let mut result = String::new();

    // Loop through the starting and ending breakpoints in steps of 100(px)
    for i in (1920..=3840).step_by(100) {
        // Add the increment value to the current base
        current_base += increment_value;

        // Add the generated media query to the css
        result += &format!(
            r#"
            @media (min-width: {}px) {{
                html {{
                    font-size: {:.4}rem;
                }}
            }}
            "#,
            i, current_base
        );
    }

    Ok(result)
}

pub fn css_variables(config: &Config) -> Result<String> {
    // Extract the UI configuration from the main config
    let ui_config = config
        .ui
        .clone()
        .context("UI configuration missing in Seyuna config")?;

    // CSS result string
    let mut result = String::new();

    // Root variables
    result += ":root {";
    // Loop through the palette colors
    for color in &ui_config.theme.colors {
        // Add the color variable
        result += &format!("--{}:{};", color.0, color.1);
    }
    result += "}";

    // Light mode variables
    result += "[data-mode=\"light\"] {";
    result += &format!(
        "--background: oklch({} {} {});",
        ui_config.theme.light.background.lightness,
        ui_config.theme.light.background.chroma,
        ui_config.theme.light.background.hue
    );
    result += &format!(
        "--text: oklch({} {} {});",
        ui_config.theme.light.text.lightness,
        ui_config.theme.light.text.chroma,
        ui_config.theme.light.text.hue
    );
    result += &format!("--chroma: {};", ui_config.theme.light.chroma,);
    result += &format!("--lightness: {};", ui_config.theme.light.lightness,);
    result += "}";

    // Dark mode variables
    result += "[data-mode=\"dark\"] {";
    result += &format!(
        "--background: oklch({} {} {});",
        ui_config.theme.dark.background.lightness,
        ui_config.theme.dark.background.chroma,
        ui_config.theme.dark.background.hue
    );
    result += &format!(
        "--text: oklch({} {} {});",
        ui_config.theme.dark.text.lightness,
        ui_config.theme.dark.text.chroma,
        ui_config.theme.dark.text.hue
    );
    result += &format!("--chroma: {};", ui_config.theme.dark.chroma,);
    result += &format!("--lightness: {};", ui_config.theme.dark.lightness,);
    result += "}";

    // Default html css settings
    result += "html {";
    result += "color: var(--text);";
    result += "background-color: var(--background);";
    result += "}";

    Ok(result)
}

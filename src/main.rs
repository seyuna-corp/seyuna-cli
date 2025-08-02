mod config;
mod helpers;
mod schema;
mod ui;

use clap::{
    Arg, ArgAction, Command,
    builder::{
        Styles,
        styling::{Color, RgbColor, Style},
    },
    crate_version,
};
use owo_colors::OwoColorize;

use crate::{
    config::handle_config_args,
    helpers::cli::{primary_text, secondary_text},
    schema::handle_schema_args,
    ui::handle_ui_args,
};

/// The async entry point for the Seyuna CLI application.
///
/// # Subcommands
/// - config
/// - ui
/// - generate-json-schema (hidden)
#[tokio::main]
async fn main() {
    // Define custom CLI styles for better visual formatting
    let clap_styles = Styles::styled()
        .header(Style::new().bold())
        .usage(Style::new().bold())
        .literal(Style::new().fg_color(Some(Color::Rgb(RgbColor(0, 255, 135)))));

    // Create and configure the root CLI command
    let matches = Command::new("Seyuna CLI")
        .bin_name("seyuna")
        .about(format!(
            "{} {} {}",
            "Visit",
            primary_text("https://seyuna.com"),
            "for more information on usage.",
        ))
        .version(crate_version!())
        .styles(clap_styles)
        .before_help(
            primary_text(&format!("{} | {} ", "Seyuna CLI", crate_version!()))
                .bold()
                .to_string(),
        )
        // `config` subcommand
        .subcommand(
            Command::new("config").about("Configure Seyuna").arg(
                Arg::new("init")
                    .short('i')
                    .long("init")
                    .help("Initialize Seyuna configuration")
                    .action(ArgAction::SetTrue),
            ),
        )
        // `ui` subcommand
        .subcommand(
            Command::new("ui")
                .about(secondary_text("Seyuna UI"))
                .arg(
                    Arg::new("compile")
                        .short('c')
                        .long("compile")
                        .help("Compile Seyuna UI styles")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("watch")
                        .short('w')
                        .long("watch")
                        .help("Watch Seyuna UI styles")
                        .action(ArgAction::SetTrue),
                ),
        )
        // Hidden subcommand for generating the JSON schema
        .subcommand(
            Command::new("generate-json-schema")
                .about(secondary_text("Generates the JSON schema for the cli"))
                .hide(true),
        )
        .get_matches();

    // Dispatch to the appropriate handler based on the subcommand
    handle_schema_args(matches.clone()).await;
    handle_config_args(matches.clone()).await;
    handle_ui_args(matches.clone()).await;
}

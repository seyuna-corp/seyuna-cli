use clap::ArgMatches;

use crate::helpers::cli::error_text;

pub mod compile;
pub mod default;
pub mod types;

pub async fn handle_ui_args(matches: ArgMatches) {
    if let Some(ui_matches) = matches.subcommand_matches("ui") {
        if ui_matches.get_flag("compile") {
            if let Err(e) = compile::compile(ui_matches).await {
                eprintln!("{}", error_text(&e.to_string()));
                std::process::exit(1); // exit with error
            }
        }
    }
}

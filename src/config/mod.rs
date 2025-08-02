use clap::ArgMatches;

use crate::config::init::init;

pub mod init;
pub mod types;

pub async fn handle_config_args(matches: ArgMatches) {
    if let Some(config_matches) = matches.subcommand_matches("config") {
        if config_matches.get_flag("init") {
            init(config_matches).await;
        }
    }
}

use clap::ArgMatches;
use std::path::Path;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{
    config::types::Config,
    helpers::cli::{spinner_done, spinner_progress, spinner_start},
    ui,
};

pub async fn init(_matches: &ArgMatches) {
    let spinner = spinner_start("Initializing configuration");

    let config = Config {
        license: None,
        ui: Some(ui::default::UI_CONFIGURATION.clone()),
    };

    spinner_progress(&spinner, "Ensuring output directory exists");
    // let dist_dir = Path::new(config.ui.as_ref().unwrap().output_dir.as_ref().unwrap());
    // fs::create_dir_all(dist_dir).await.unwrap();

    spinner_progress(&spinner, "Creating configuration file");
    let json = serde_json::to_string_pretty(&config).unwrap();

    spinner_progress(&spinner, "Saving Seyuna.json file");
    let file_name = "seyuna.json";
    let dest_path = Path::new(file_name);
    let mut file = File::create(&dest_path).await.unwrap();
    file.write_all(json.as_bytes()).await.unwrap();

    spinner_done(&spinner, "Successfully initialized Seyuna!");
}

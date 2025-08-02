use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use clap::ArgMatches;
use schemars::schema_for;

use crate::config::types::Config;

pub async fn handle_schema_args(matches: ArgMatches) {
    // Check if the 'generate-json-schema' subcommand was invoked
    if let Some(_args) = matches.subcommand_matches("generate-json-schema") {
        // Get the current crate version from the environment
        let version = env!("CARGO_PKG_VERSION");

        // Define the output directory for the schema files
        let dist_dir = Path::new("schema");

        // Create the directory if it doesn't already exist
        fs::create_dir_all(dist_dir).unwrap();

        // Generate the JSON schema for the Config struct
        let schema = schema_for!(Config);

        // Serialize the schema into a pretty-printed JSON string
        let json = serde_json::to_string_pretty(&schema).unwrap();

        // Construct the filename using the current version, e.g., "v-1.2.3.schema.json"
        let file_name = format!("v-{}.schema.json", version);
        let dest_path = dist_dir.join(file_name);

        // Write the JSON schema to the file
        let mut file = File::create(&dest_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();

        // Print a cargo warning to indicate where the schema was written
        println!("cargo:warning=JSON schema written to {:?}", dest_path);
    }
}

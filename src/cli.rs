use clap::{Arg, Command};
use std::path::PathBuf;

pub struct CliArgs {
    pub source_path: PathBuf,
    pub excludes: Vec<String>,
    pub output_format: String,
}

impl CliArgs {
    pub fn parse() -> Self {
        let matches = Command::new("Code Duplication Detector")
            .version("1.0")
            .author("Your Name")
            .about("Detects code duplication across multiple files")
            .arg(
                Arg::new("source-path")
                    .short('s')
                    .long("source-path")
                    .value_name("SOURCE")
                    .help("Path to the source code directory")
                    .required(true),
            )
            .arg(
                Arg::new("excludes")
                    .short('e')
                    .long("excludes")
                    .value_name("EXCLUDES")
                    .help("Glob patterns to exclude directories or files")
                    .action(clap::ArgAction::Append), // Allow multiple occurrences
            )
            .arg(
                Arg::new("output-format")
                    .short('o')
                    .long("output-format")
                    .value_name("FORMAT")
                    .help("Output format (e.g., json)")
                    .default_value("json"),
            )
            .get_matches();

        Self {
            source_path: matches.get_one::<String>("source-path").unwrap().into(),
            excludes: matches
                .get_many::<String>("excludes")
                .map(|values| values.map(ToString::to_string).collect())
                .unwrap_or_default(),
            output_format: matches
                .get_one::<String>("output-format")
                .unwrap()
                .to_string(),
        }
    }
}
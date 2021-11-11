use anyhow::Result;
use clap::{crate_authors, crate_version, App, AppSettings, Arg, ArgMatches};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

mod utils;

use utils::{find_yaml_block, ValidatorError};

#[derive(Debug, Deserialize, Serialize)]
/// [`HipHeader`] is the structure that our Yaml front matter block should have.
struct HipHeader {
    hip: String,
    title: String,
    description: String,
    author: String,
    status: HIPState,
    created: String,
}

#[derive(Debug, Deserialize, Serialize)]
/// [`HIPState`] is an enum of all possible state of an HIP.
enum HIPState {
    Draft,
    Review,
    LastCall,
    Accepted,
    Withdrawn,
}

fn main() {
    // Create CLI matches
    let matches = App::new("HIP Validator")
        .bin_name("hip-validator")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Validate a HIP proposal file")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::new("file")
                .about("Markdown file to validate")
                .required(true),
        )
        .get_matches();

    // Get file data
    let res_file_data = get_file_data(&matches);
    if res_file_data.is_err() {
        eprintln!("error: {:?}", res_file_data.err().unwrap());
        std::process::exit(1);
    }
    let file_data = res_file_data.unwrap();

    // Fetch Yaml front matter block
    match find_yaml_block(file_data.as_str()) {
        Some((fm_start, fm_end)) => {
            let fm_str = &file_data[fm_start..fm_end];

            let res_validator = validate_fm(fm_str);
            if res_validator.is_err() {
                eprintln!("error: {:?}", res_validator.err().unwrap());
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("error: markdown file should contain a YAML front matter block");
            std::process::exit(1);
        }
    }

    std::process::exit(0)
}

/// [`get_file_data`] takes user specified path and try to read data from it.
fn get_file_data(matches: &ArgMatches) -> Result<String> {
    // Match file path
    let file_path = PathBuf::from(matches.value_of("file").ok_or(ValidatorError::NoFilePath)?);

    // Get file full path
    let file_full_path = std::fs::canonicalize(file_path)?;

    // Retrieve file data and sanitize line break
    Ok(std::fs::read_to_string(file_full_path)?.replace("\r\n", "\n"))
}

/// [`validate_fm`] will verify that the font matter block corresponds to our specifications
fn validate_fm(fm_str: &str) -> Result<()> {
    // Deserializing will check for us that all fields are filled
    let yaml: HipHeader = serde_yaml::from_str(&fm_str)?;

    // Check data format
    let res_date = iso_8601::Date::from_str(yaml.created.as_str());
    if res_date.is_err() {
        return Err(ValidatorError::InvalidDateFormat.into());
    }

    Ok(())
}

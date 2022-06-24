use std::fs::File;
use std::io::{BufReader};
use crate::config::app_config::{Configuration};

mod app_config;

const WORK_DIRECTORY_NAME: &str = "WORK_DIRECTORY";

pub fn load_config() -> Result<Configuration, Box<dyn std::error::Error>> {
    let work_directory = std::env::var(WORK_DIRECTORY_NAME)?;
    let config_file_path = work_directory + "/config.yml";
    let file = File::open(config_file_path)?;
    let reader = BufReader::new(file);
    let configuration: Configuration = serde_yaml::from_reader(reader)?;
    return Ok(configuration);
}
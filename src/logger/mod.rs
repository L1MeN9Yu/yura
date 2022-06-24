extern crate log;

use fast_log::Config;
use fast_log::plugin::file_split::RollingType;
use fast_log::consts::LogSize;
use fast_log::plugin::packer::LogPacker;
use log::LevelFilter::{Info};
use super::config;

pub fn init_logger(directory_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let work_directory = config::work_directory_path()?;
    let logs_directory_path = work_directory + "/" + &*directory_path + "/";
    fast_log::init(
        Config::new()
            .console()
            .file_split(logs_directory_path.as_str(), LogSize::MB(1), RollingType::KeepNum(10000), LogPacker {})
            .level(Info)
    )?;
    return Ok(());
}
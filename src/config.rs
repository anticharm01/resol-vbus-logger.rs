use std::fs::File;
use std::io::Read;

use anyhow::{anyhow, Context};
use resol_vbus::SpecificationFile;

use crate::error::Result;

#[derive(Deserialize)]
pub struct Config {
    pub path: Option<String>,

    pub address: Option<String>,
    pub via_tag: Option<String>,
    pub password: Option<String>,
    pub channel: Option<u8>,
    pub timeout: Option<i64>,
    pub known_packet_ids: Vec<String>,

    pub png_tick_interval: i64,
    pub png_input_filename: String,
    pub png_output_filename: String,

    pub csv_tick_interval: i64,
    pub csv_output_filename_pattern: String,

    pub live_data_text_tick_interval: i64,
    pub live_data_text_output_filename: String,

    pub sqlite_tick_interval: i64,
    pub sqlite_filename: String,
    pub sqlite_datasets_table: Option<String>,
    pub sqlite_fields_table: Option<String>,
    pub sqlite_statement: Option<String>,
    pub sqlite_fields: Option<Vec<String>>,

    pub vsf_filename: Option<String>,
}

impl Config {
    pub fn load() -> Result<Config> {
        let mut file = File::open("config.toml")?;

        let mut config_string = String::new();

        file.read_to_string(&mut config_string)?;

        let config = toml::from_str(&config_string)?;

        Ok(config)
    }

    pub fn load_spec_file(&self) -> Result<SpecificationFile> {
        let spec_file = match &self.vsf_filename {
            Some(filename) => {
                let bytes = std::fs::read(filename).context("Unable to read VSF file")?;
                SpecificationFile::from_bytes(&bytes)
                    .map_err(|err| anyhow!("Unable to parse VSF file: {:?}", err))?
            }
            None => SpecificationFile::new_default(),
        };
        Ok(spec_file)
    }
}

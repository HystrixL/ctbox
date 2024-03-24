use confy::ConfyError;
use ctbox::network::entity::User;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, default, path::PathBuf};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Root {
    pub general: General,
    pub network: Network,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct General {
    pub retry_times: i32,
}

impl Default for General {
    fn default() -> Self {
        Self { retry_times: 3 }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Network {
    pub connect_check: bool,
}

impl Default for Network {
    fn default() -> Self {
        Self {
            connect_check: true,
        }
    }
}

pub fn read() -> Result<Root, ConfyError> {
    confy::load("ctbox", "config")
}

pub fn write(root: Root) -> Result<(), ConfyError> {
    confy::store("ctbox", "config", root)
}

pub fn path() -> PathBuf{
    confy::get_configuration_file_path("ctbox", "config").unwrap()
}
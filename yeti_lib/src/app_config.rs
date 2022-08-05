use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::Saveable;
#[derive(Deserialize,Serialize,Debug)]
pub struct AppConfig{
    pub loaded_config_name: String,
    pub hack_config_path: PathBuf,
    pub signatures_path: PathBuf,
    pub signatures_config_path: PathBuf,
    pub log_config_path: PathBuf,

}
impl Saveable for AppConfig{
    const CONFIG_NAME:&'static str = "App Config";
}
impl Default for AppConfig{
    fn default() -> Self {
        Self { 
            hack_config_path: PathBuf::from("./default.json"),
            signatures_path:  PathBuf::from("./signatures.json"),
            log_config_path: PathBuf::from("./log-config.yaml"),
            loaded_config_name: "default".to_string(),
            signatures_config_path: PathBuf::from("./haze_dumper_config.json"), }
    }
}
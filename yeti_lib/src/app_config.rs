use std::path::PathBuf;

use glium::glutin;
use serde::{Deserialize, Serialize};

use crate::Saveable;
#[derive(Deserialize,Serialize,Debug)]
pub struct AppConfig{
    pub loaded_config_name: String,
    pub hack_config_path: PathBuf,
    pub signatures_path: PathBuf,
    pub signatures_config_path: PathBuf,
    pub log_config_path: PathBuf,
    pub key_binds: KeyBinds

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
            signatures_config_path: PathBuf::from("./haze_dumper_config.json"),
            key_binds: KeyBinds::default(), }
    }
}
#[derive(Deserialize,Serialize,Default,Debug)]
pub struct KeyBinds{
    pub glow: Option<KeyBindType>,
    pub bhop: Option<KeyBindType>,
}
#[derive(Deserialize,Serialize,Debug)]
pub enum KeyBindType{
    Toggle(glutin::event::VirtualKeyCode),
    Hold(glutin::event::VirtualKeyCode),
    None
}
impl PartialEq for KeyBindType{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Toggle(l0), Self::Toggle(r0)) => l0 == r0,
            (Self::Hold(l0), Self::Hold(r0)) => l0 == r0,
            (KeyBindType::Toggle(l0), KeyBindType::Toggle(r0)) => {l0 == r0},
            (KeyBindType::Toggle(_), KeyBindType::Hold(_)) => {return false},
            (KeyBindType::Hold(_), KeyBindType::Toggle(_)) => {return false},
            (KeyBindType::Hold(l0), KeyBindType::Hold(r0)) => {l0 == r0},
            (KeyBindType::Toggle(_), KeyBindType::None) => {return false},
            (KeyBindType::Hold(_), KeyBindType::None) => {return false},
            (KeyBindType::None, KeyBindType::Toggle(_)) => {return false},
            (KeyBindType::None, KeyBindType::Hold(_)) =>{return false},
            (KeyBindType::None, KeyBindType::None) => {return true},
        }
    }
}
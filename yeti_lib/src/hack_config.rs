use std::{fs::File, io::{Write, Read}, path::PathBuf};
use serde::{Deserialize, Serialize};
use crate::Saveable;

use self::{bhop::BhopConfig, glow::GlowConfig, aim::AimConfig};

pub mod aim;
pub mod bhop;
pub mod glow;

#[derive(Deserialize,Serialize,Debug,Copy,Clone)]
pub struct YetiHackConfig{

    //hack settings
    pub toggle: HackToggle,
    pub bhop:BhopConfig,
    pub glow:GlowConfig,
    pub aim: AimConfig
}
impl Saveable for YetiHackConfig{
    const CONFIG_NAME:&'static str = "Hack Config";
}
impl Default for YetiHackConfig{
    fn default() -> Self {
        Self {
            bhop: BhopConfig{},
            toggle: HackToggle {
                bhop: false,
                glow: false,
                aim: false, 
            },
            glow: GlowConfig::default(),
            aim: AimConfig::default(),
        }
    }
}
#[derive(Deserialize,Serialize,Debug,Copy,Clone)]
pub struct HackToggle{
    pub bhop: bool,
    pub glow: bool,
    pub aim: bool
}




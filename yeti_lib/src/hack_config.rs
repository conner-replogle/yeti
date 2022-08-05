use std::{fs::File, io::{Write, Read}, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::Saveable;

#[derive(Deserialize,Serialize,Debug,Copy,Clone)]
pub struct YetiHackConfig{

    //hack settings
    pub toggle: HackToggle,
    pub bhop:BhopConfig,
    pub glow:GlowConfig
}
impl Saveable for YetiHackConfig{
    const CONFIG_NAME:&'static str = "Hack Config";
}
impl Default for YetiHackConfig{
    fn default() -> Self {
        Self {
            bhop: BhopConfig{
                a: 2
            },
            toggle: HackToggle {
                bhop: false,
                glow: false, 
            },
            glow: GlowConfig { 
                enemy_glow: GlowSettings { 
                    enabled: true, 
                    glow_rgba: [1.0,0.,0.,0.4], 
                    entity_color: [255,255,255,255], 
                    render_when_occluded: true, 
                    render_when_unoccluded: true, 
                    full_bloom: false 
                }, 
                team_glow: GlowSettings { 
                    enabled: false, 
                    glow_rgba: [0.0,0.,1.,0.3], 
                    entity_color: [255,255,255,255], 
                    render_when_occluded: true, 
                    render_when_unoccluded: true, 
                    full_bloom: false 
                } 
            },
        }
    }
}
#[derive(Deserialize,Serialize,Debug,Copy,Clone)]
pub struct HackToggle{
    pub bhop: bool,
    pub glow: bool
}

#[derive(Deserialize,Serialize,Debug,Copy,Clone)]
pub struct BhopConfig{
    a: u32
}
#[derive(Deserialize,Serialize,Debug,Copy,Clone)]
pub struct GlowConfig{
    pub enemy_glow: GlowSettings,
    pub team_glow: GlowSettings,
}
#[derive(Deserialize,Serialize,Debug,Copy,Clone)]
pub struct GlowSettings{
    pub enabled: bool,
    pub glow_rgba:[f32;4],
    pub entity_color: [u8;4],
    pub render_when_occluded: bool,
    pub render_when_unoccluded: bool,
    pub full_bloom: bool,

}
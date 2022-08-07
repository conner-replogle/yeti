use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug,Copy,Clone)]
pub struct GlowConfig{
    pub enemy_glow: GlowSettings,
    pub team_glow: GlowSettings,
}
impl Default for GlowConfig {
    fn default() -> Self {
        Self { enemy_glow: GlowSettings { 
            enabled: true, 
            glow_rgba: [1.0,0.,0.,0.4], 
            entity_color: [255,255,255,255], 
            render_when_occluded: true, 
            render_when_unoccluded: true, 
            full_bloom: false,
            health_affect: false,
            bomb_defusal_affect: false, 
        }, team_glow: GlowSettings { 
            enabled: false, 
            glow_rgba: [0.0,0.,1.,0.3], 
            entity_color: [255,255,255,255], 
            render_when_occluded: true, 
            render_when_unoccluded: true, 
            full_bloom: false ,
            health_affect: false,
            bomb_defusal_affect: false, 
        }  }
    }
}




#[derive(Deserialize,Serialize,Debug,Copy,Clone)]
pub struct GlowSettings{
    pub enabled: bool,
    pub glow_rgba:[f32;4],
    pub entity_color: [u8;4],
    pub render_when_occluded: bool,
    pub render_when_unoccluded: bool,
    pub full_bloom: bool,
    pub health_affect: bool,
    pub bomb_defusal_affect: bool

}
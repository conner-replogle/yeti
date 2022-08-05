use arc_swap::access::{Map, DynAccess, DynGuard};
use yeti_lib::hack_config::{YetiHackConfig, GlowConfig};

use crate::api::Game;
use crate::api::datatypes::GlowStruct;
use crate::api::player::Player;
use crate::yeti_hack_manager::{ThreadSafeGame, ThreadSafeConfig};
use crate::hacks::Hack;
use std::sync::Arc;
use super::ThreadSafeBool;


type ThreadSafeConfigAccess = Box<dyn DynAccess<GlowConfig> + Send+ Sync>;

pub struct Glow{
    config: ThreadSafeConfigAccess,
    game: ThreadSafeGame,
}
impl Glow{
    fn set_glow_color(&self,glow: &mut GlowStruct,entity: &Player){
        let defusing = entity.m_bIsDefusing();
        if defusing
        {
            glow.r = 1.0f32;
            glow.g = 1.0f32;
            glow.b = 1.0f32;
        }
        else
        {
            let health = *entity.health();
            glow.r = health as f32 * -0.01f32 + 1.0f32;
            glow.g = health as f32 * 0.01f32;
        }
        

    }

    fn set_enemy_glow(&self,game: &Game,entity: &Player, glow_index: u32)
    {
        let e_glow = game.client.get_glowstruct(glow_index);
        let mut temp = e_glow.clone();
        let glow_set = self.config.load().enemy_glow;
        temp.r = glow_set.glow_rgba[0] * 2.0;
        temp.g = glow_set.glow_rgba[1] * 2.0;
        temp.b = glow_set.glow_rgba[2] * 2.0;
        temp.a = glow_set.glow_rgba[3] * 2.0;
        temp.render_when_occluded = glow_set.render_when_occluded;
        temp.render_when_unoccluded = glow_set.render_when_unoccluded;
        temp.full_bloom = glow_set.full_bloom;
        self.set_glow_color(&mut temp,entity);
        *e_glow = temp;

    }
    fn set_team_glow(&self,game:&Game,entity: &Player, glow_index: u32)
    {
        let mut e_glow = game.client.get_glowstruct(glow_index);
        let mut temp = e_glow.clone();
        let glow_set = self.config.load().team_glow;
        temp.r = glow_set.glow_rgba[0] * 2.0;
        temp.g = glow_set.glow_rgba[1] * 2.0;
        temp.b = glow_set.glow_rgba[2] * 2.0;
        temp.a = glow_set.glow_rgba[3] * 2.0;
        temp.render_when_occluded = glow_set.render_when_occluded;
        temp.render_when_unoccluded = glow_set.render_when_unoccluded;
        temp.full_bloom = glow_set.full_bloom;
        *e_glow = temp;
    }
}
impl Hack for Glow{
    fn get_toggle(main_config: ThreadSafeConfig)->ThreadSafeBool{
        let toggle = Box::new(Map::new(Arc::clone(&main_config), |config: &YetiHackConfig| &config.toggle.glow));
        return toggle;
    }
    fn start(main_config:ThreadSafeConfig,game: ThreadSafeGame) -> Self {
        let config = Box::new(Map::new(Arc::clone(&main_config), |config: &YetiHackConfig| &config.glow));
        return Self { config, game}
    }
    fn update(&mut self) {
        let game: DynGuard<Game> = self.game.load();
        if game.l_player.is_none() || !game.in_game(){
            return;
        }
        let l_player = game.l_player.as_ref().unwrap();
        let my_team: u32 = l_player.m_iTeamNum();
        let config = self.config.load();
        //println!("Team: {}",my_team);
        for i in 0..64{
            let entity_offset: u32 = game.client.dwEntityList(i);
            if entity_offset == 0{
                continue;
            }
            let entity = Player(entity_offset,game.sig.clone());
            let team = entity.m_iTeamNum();
            let glow_index = entity.m_iGlowIndex();
            //println!("index:{} ent_offset:{} glowIndex:{}",i,entity_offset,glow_index);
            if team == my_team && config.team_glow.enabled{
                self.set_team_glow(&game,&entity,glow_index);

            }
            if team != my_team && config.enemy_glow.enabled{
                self.set_enemy_glow(&game,&entity,glow_index);
            }
        }
    }
    fn exit(&mut self) {
        
    }
}
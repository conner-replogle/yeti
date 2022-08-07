use arc_swap::access::DynGuard;
use cgmath::MetricSpace;
use log::debug;
use yeti_lib::hack_config::aim::{Bones, TargetingMethod, AimConfig};

use crate::api::{Game, player::Player};



pub struct TargetingManager{
    current_targeting: TargetingMethod,
    value: Option<Box<dyn FindTarget>>,
}
impl TargetingManager{
    pub fn new(config: &DynGuard<AimConfig>) -> Self{
        debug!("Initiated targeting manager with {:?}",config.targeting_mode);
        let mut a = Self {value:None,current_targeting: TargetingMethod::FOV_ALGO };
        a.update_targeting(config);
        a
        
    }
    pub fn update_targeting(&mut self,config: &DynGuard<AimConfig>,){
        if self.current_targeting as u32 != config.targeting_mode as u32{
            match config.targeting_mode{
                TargetingMethod::CLOSEST => {
                    debug!("Changed targeting to Closest");
                    self.value = Some(Box::new(ClosestEnt::new()));
                },
                TargetingMethod::FOV_ALGO => {

                },
            }
        }
        
    }
    pub fn find(&self,game: &Game,l_player: &Player,config: &DynGuard<AimConfig>) -> Option<Player>{
        if let Some(algo) =  &self.value{
            return algo.find(game,l_player,config);
        }
        None
    }
}

pub trait FindTarget{
    fn new()-> Self where Self: Sized;
    fn find(&self,game: &Game,l_player:&Player,config: &DynGuard<AimConfig>) -> Option<Player>;
}


struct ClosestEnt{
    previous_target: Option<Player>
}

impl FindTarget for ClosestEnt{
    fn find(&self,game: &Game,l_player:&Player,config: &DynGuard<AimConfig>) -> Option<Player>{
        let mut pos = l_player.m_vecOrigin();
        debug!("player position {:?}",pos);
        //pos += l_player.m_dwBoneMatrix(Bones::HEAD);
        let mut lowest_score = f32::INFINITY;
        let mut best_ent: Option<Player> = None;
        let team = l_player.m_iTeamNum();
        let l_player_id:u32 = game.engine.dwClientState_GetLocalPlayer();
        
        let mut closest = f32::INFINITY;
        let mut closest_ent = None;
        for i in 0..64{
            let entity_offset: u32 = game.client.dwEntityList(i);
            if entity_offset == 0{
                continue;
            }
            
            let entity = Player(entity_offset,game.sig.clone());
            if entity.m_iTeamNum() == team{
                continue;
            }
            
            let ent_pos = entity.m_vecOrigin() + entity.m_dwBoneMatrix(Bones::HEAD);
            debug!("Ent {} on team {} at pos {:?}",i,entity.m_iTeamNum(),ent_pos);
            let spotted= entity.m_bSpottedByMask();
            debug!("spotted {} player id {}",spotted,l_player_id);
            if (spotted << l_player_id) == 0{
                continue;
            }
            let distance = pos.distance(ent_pos);
            debug!("distance {} closet {}",distance,closest);
            
            
            if distance < closest{
                closest = distance;
                closest_ent = Some(entity);
            }
        }
        return closest_ent;
    }

    fn new()-> Self where Self: Sized {
        Self { previous_target: None }
    }

}
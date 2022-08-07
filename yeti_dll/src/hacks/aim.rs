use std::{sync::Arc, f32::consts::PI};

use arc_swap::access::{DynAccess, Map, DynGuard};
use cgmath::Vector3;
use yeti_lib::hack_config::{YetiHackConfig, aim::{AimConfig, AimMode, TargetingMethod}};

use crate::{yeti_hack_manager::ThreadSafeGame, api::{Game, player::Player}};

use self::find_target::TargetingManager;

use super::Hack;
mod find_target;

type ThreadSafeConfigAccess = Box<dyn DynAccess<AimConfig> + Send+ Sync>;

pub struct Aim{
    config: ThreadSafeConfigAccess,
    game: ThreadSafeGame,
    targeting_algo: TargetingManager
}
impl Hack for Aim{
    fn get_toggle(main_config:crate::yeti_hack_manager::ThreadSafeConfig)-> super::ThreadSafeBool {
        let toggle = Box::new(Map::new(Arc::clone(&main_config), |config: &YetiHackConfig| &config.toggle.aim));
        return toggle;
    }

    fn start(main_config: crate::yeti_hack_manager::ThreadSafeConfig,game: ThreadSafeGame) -> Self {
        let config = Box::new(Map::new(Arc::clone(&main_config), |config: &YetiHackConfig| &config.aim));
        let targeting_algo = TargetingManager::new(&config.load());
        return Self { config, game,targeting_algo}
    }

    fn update(&mut self) {
        let game: DynGuard<Game> = self.game.load();
        if game.l_player.is_none() || !game.in_game(){
            return;
        }
        let config: DynGuard<AimConfig> = self.config.load();
        self.targeting_algo.update_targeting(&config);
        let l_player = game.l_player.as_ref().unwrap();
        let target = self.targeting_algo.find(&game, l_player, &config);
        if let Some(target) = target{
            

        }else{
            println!("No Target Found");
        }


        

    }

    fn exit(&mut self) {
        
    }
}

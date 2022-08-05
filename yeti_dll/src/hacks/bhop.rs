use std::sync::Arc;

use arc_swap::access::{DynAccess, DynGuard, Map};
use log::debug;
use windows::{Win32::{System::LibraryLoader::{GetModuleHandleA, GetModuleHandleW}, Foundation::HINSTANCE, UI::Input::KeyboardAndMouse::GetAsyncKeyState}, core::{PCSTR, PCWSTR, HSTRING}};
use yeti_lib::{hack_config::{BhopConfig, YetiHackConfig}, signatures::Signatures};
use crate::{yeti_hack_manager::{ThreadSafeSignature, ThreadSafeGame, ThreadSafeConfig}, api::{player::Player, client::Client, Game, engine::ClientState}};
use super::{Hack, ThreadSafeBool};

type ThreadSafeConfigAccess = Box<dyn DynAccess<BhopConfig> + Send+ Sync>;

pub struct Bhop{
    config: ThreadSafeConfigAccess,
    game: ThreadSafeGame
}
impl Hack for Bhop{
    fn get_toggle(main_config: ThreadSafeConfig)->ThreadSafeBool{
        let toggle = Box::new(Map::new(Arc::clone(&main_config), |config: &YetiHackConfig| &config.toggle.bhop));
        return toggle;
    }
    fn start(main_config:ThreadSafeConfig,game: ThreadSafeGame) -> Self {
        let config = Box::new(Map::new(Arc::clone(&main_config), |config: &YetiHackConfig| &config.bhop));
        

        Bhop { config,game}
    }
    fn update(&mut self) {
        let game: DynGuard<Game> = self.game.load();
        if !game.in_game(){
            return;
        }
    
        if let Some(player) = &game.l_player{

            let on_ground  = (player.m_fFlags() & (1 << 0)) != 0;
            //println!("Flag {}",on_ground);
            if on_ground  && (unsafe{GetAsyncKeyState(0x20) as u32 & 0x8000}) != 0 {
                let jump = game.client.dwForceJump();
                *jump = 6;
            }
        }else{
            println!("Cant find lPlayer");
        }
        
        
    }

    fn exit(&mut self) {
        
    }
}
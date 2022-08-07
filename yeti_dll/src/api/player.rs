#![allow(non_snake_case)]
use cgmath::Vector3;
use log::debug;

use yeti_lib::hack_config::aim::Bones;


use crate::yeti_hack_manager::ThreadSafeSignature;
#[derive(Clone,Debug)]
pub struct Player(pub u32,pub ThreadSafeSignature);
impl Player{
    pub fn get_lplayer(client_module: u32,sig: ThreadSafeSignature) -> Option<Self>{
        let s = sig.load();
        let l_player_addy = unsafe{*((client_module + s.get_s("dwLocalPlayer") as u32) as *mut u32)};
        if l_player_addy != 0{
            Some(Player(l_player_addy,sig))
        }else{
            None
        }

    }
}
impl Player{
    pub fn health(&self) -> &u32{
        unsafe{& *((self.0 + self.1.load().get_n("m_iHealth") as u32) as *mut u32)}
    }
    pub fn m_fFlags(&self) -> &u32{
        unsafe{& *((self.0 + self.1.load().get_n("m_fFlags") as u32) as *mut u32)}
    }
    pub fn m_iTeamNum(&self) -> u32{
        unsafe{*((self.0 + self.1.load().get_n("m_iTeamNum") as u32) as *mut u32)}
    }
    pub fn m_iGlowIndex(&self) -> u32{
        unsafe{*((self.0 + self.1.load().get_n("m_iGlowIndex") as u32) as *mut u32)}
    }
    pub fn m_bIsDefusing(&self) -> bool{
        return unsafe{*((self.0 + self.1.load().get_n("m_bIsDefusing") as u32) as *mut u32)} != 0;
    }
    pub fn m_vecOrigin(&self) -> Vector3<f32>{
        return unsafe{*((self.0 + self.1.load().get_n("m_vecOrigin") as u32) as *mut Vector3<f32>)};
        
    }
    pub fn m_dwBoneMatrix(&self,bone_id:Bones) -> Vector3<f32>{
        return unsafe{*((self.0 + self.1.load().get_n("m_dwBoneMatrix") as u32 + 0x30 * bone_id as u32) as *mut Vector3<f32>)};
    } 
    pub fn m_bSpottedByMask(&self) -> u32{
        unsafe{*((self.0 + self.1.load().get_n("m_bSpottedByMask") as u32) as *mut u32)}
    }
}
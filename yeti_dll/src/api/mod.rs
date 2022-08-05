use std::sync::Arc;

use crate::yeti_hack_manager::ThreadSafeSignature;

use self::{client::Client, player::Player, engine::{Engine, ClientState}};

pub mod player;
pub mod client;
pub mod engine;
pub mod datatypes;
#[derive(Clone,Debug)]
pub struct Game{
    pub sig: ThreadSafeSignature,
    pub l_player: Option<Player>,
    pub client: Client,
    pub engine: Engine
}
impl Game{
    pub fn new(sig: ThreadSafeSignature) -> Self {
        let client = Client::new(Arc::clone(&sig));
        let engine = Engine::new(Arc::clone(&sig));
        Self{
            client,
            l_player: None,
            sig,
            engine,
        }

    }
    pub fn update(&mut self){
        
        self.l_player = Player::get_lplayer(self.client.0,Arc::clone(&self.sig));
    }
    pub fn in_game(&self) -> bool{
        if let Some(state) =  self.engine.dwClientState_State(){
            if let ClientState::IN_GAME = state{
                return true
            }
        }
        return false

    }
}


use std::sync::Arc;

use crate::yeti_hack_manager::ThreadSafeSignature;

use self::{client::Client, player::Player, engine::{Engine, ClientState}, window::Window};

pub mod player;
pub mod client;
pub mod engine;
pub mod datatypes;
pub mod window;



//TODO have the players in game be stored in this to reduce computations
#[derive(Clone,Debug)]
pub struct Game{
    pub sig: ThreadSafeSignature,
    pub l_player: Option<Player>,
    pub client: Client,
    pub engine: Engine,
    pub window: Window
}
impl Game{
    pub fn new(sig: ThreadSafeSignature) -> Self {
        let client = Client::new(Arc::clone(&sig));
        let engine = Engine::new(Arc::clone(&sig));
        let window = Window::start();
        Self{
            client,
            l_player: None,
            sig,
            engine,
            window,
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


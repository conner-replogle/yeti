

use std::{sync::{Arc, atomic::{AtomicBool, Ordering}, Mutex}, thread::{self, JoinHandle}, collections::HashMap};

use arc_swap::{ArcSwap, ArcSwapAny, access::{DynAccess, Map}};
use ipmpsc::{Receiver, SharedRingBuffer};
use log::{error, debug, info};
use windows::Win32::Foundation::RPC_NT_DUPLICATE_ENDPOINT;
use yeti_lib::{hack_config::{YetiHackConfig, HackToggle}, imc::{Packet, MessageType, STATUS_MESSAGE}, signatures::Signatures};

use crate::{error::Error, hacks::{bhop::Bhop, Hack, HackRunnable, glow::Glow,aim::Aim}, api::Game};

pub type ThreadSafeSignature = Arc<ArcSwapAny<Arc<Signatures>>>;
pub type ThreadSafeConfig = Arc<ArcSwapAny<Arc<YetiHackConfig>>>;
pub type ThreadSafeGame = Arc<ArcSwapAny<Arc<Game>>>;
pub struct Yeti{
    rx: Receiver,
    config: ThreadSafeConfig,
    game: ThreadSafeGame,
    signature: ThreadSafeSignature,
    toggle_list: Box<dyn DynAccess<HackToggle> + Send + Sync>,
    threads: HashMap<&'static str,HackRunnable>
}
impl Yeti{
    pub fn new(map_file:&str) -> Result<Self,Error>{
        let shared = SharedRingBuffer::open(map_file);
        if let Ok(buffer) = shared{
            info!("Shared Memory Buffer Connected");
            let rx = Receiver::new(buffer);
            let config = Arc::new(ArcSwap::from_pointee(YetiHackConfig::default()));
            let signature = Arc::new(ArcSwap::from_pointee(Signatures::default()));
            let game = Arc::new(ArcSwap::from_pointee(Game::new(Arc::clone(&signature))));
            return Ok(Yeti{
                rx,
                config: Arc::clone(&config),
                game,
                signature,
                toggle_list: Box::new(Map::new(Arc::clone(&config), |config: &YetiHackConfig| &config.toggle)),
                threads: HashMap::new(),

            });
        }else{
            return Err(Error::InitSharedMemory(shared.err().unwrap()));
        }
        
        
    }
    fn handle_packet(&mut self,packet: Packet) -> Result<(),Error>{
        if let MessageType::STATUS = packet.message_type{
            let status_message = self.rx.recv::<STATUS_MESSAGE>();
            if let Ok(message) = status_message{
                match message{
                    STATUS_MESSAGE::STOP => {
                        self.kill_threads();
                        return Err(Error::KILL);
                    }

                }
            }else{
                return Err(Error::MissingPacket(status_message.unwrap_err()));
            }
        }
        else 
        if let MessageType::CONFIG = packet.message_type{
            let config = self.rx.recv::<YetiHackConfig>();
            if let Ok(config) = config{
                self.config.store(Arc::new(config));
                debug!("Config updated");
            }else{
                return Err(Error::MissingPacket(config.unwrap_err()));
            }
        }else 
        if let MessageType::SIGNATURE = packet.message_type{
            let sigs = self.rx.recv::<Signatures>();
            if let Ok(sigs) = sigs{
                self.signature.store(Arc::new(sigs));
                debug!("Signature updated");
            }else{
                return Err(Error::MissingPacket(sigs.unwrap_err()));
            }
        }
        Ok(())
    }
    pub fn kill_threads(&mut self){
        for (k, v) in self.threads.drain() {
            v.stop();
            println!("Killed {}",k)
        }
    }
    fn start_hack<T:Hack>(&mut self)
        where T: Hack + 'static{
        let game= Arc::clone(&self.game);
        let config = Arc::clone(&self.config);
        
        let a = HackRunnable::start( move |on|{
            let name = std::any::type_name::<T>(); 
            debug!("{} thread has been started",name); 
            let toggle = T::get_toggle(Arc::clone(&config));
            let mut hack = T::start(Arc::clone(&config),Arc::clone(&game));
            while *toggle.load() && on.load(Ordering::Relaxed){
                hack.update();
            }
            debug!("{} Killed",name);
            hack.exit();
        });
        self.threads.insert(std::any::type_name::<T>(), a);
    }
    fn thread_manager(&mut self){
        let toggle_list = self.toggle_list.load();
        if toggle_list.bhop && !self.threads.contains_key(std::any::type_name::<Bhop>()){
            self.start_hack::<Bhop>();
        }else if !toggle_list.bhop && self.threads.contains_key(std::any::type_name::<Bhop>()){
            let a = self.threads.remove(std::any::type_name::<Bhop>()).unwrap();
            a.stop();
        }

        if toggle_list.glow && !self.threads.contains_key(std::any::type_name::<Glow>()){
            self.start_hack::<Glow>();
        }else if !toggle_list.glow && self.threads.contains_key(std::any::type_name::<Glow>()){
            let a = self.threads.remove(std::any::type_name::<Glow>()).unwrap();
            a.stop();
        }
        if toggle_list.aim && !self.threads.contains_key(std::any::type_name::<Aim>()){
            self.start_hack::<Aim>();
        }else if !toggle_list.aim && self.threads.contains_key(std::any::type_name::<Aim>()){
            let a = self.threads.remove(std::any::type_name::<Aim>()).unwrap();
            a.stop();
        }
    }
    pub fn update(&mut self) -> Result<(),Error>{
        let packet = self.rx.try_recv::<Packet>();
        if let Ok(packet) = packet{
            if let Some(packet) = packet{
                debug!("Obtained packet for {:?}",packet.message_type);
                self.handle_packet(packet)?;
            }
        }else{
            return Err(Error::MissingPacket(packet.unwrap_err()))
        }
        let a:arc_swap::access::DynGuard<Game> = self.game.load();
        let mut a = a.clone();
        a.update();
        self.game.store(Arc::new(a));
        
        
        self.thread_manager();
        Ok(())
    }
}
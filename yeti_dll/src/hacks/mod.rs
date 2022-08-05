use std::{thread::{JoinHandle, self}, sync::{Arc, atomic::{AtomicBool, Ordering}}, marker::PhantomData, any::Any};

use arc_swap::access::{DynAccess, Map};
use log::debug;
use yeti_lib::hack_config::YetiHackConfig;

use crate::yeti_hack_manager::{ThreadSafeSignature, ThreadSafeGame, ThreadSafeConfig};

pub mod bhop;
pub mod glow;
pub type ThreadSafeBool = Box<dyn DynAccess<bool>>;

pub trait Hack{

    fn get_toggle(a:ThreadSafeConfig)-> ThreadSafeBool;
    fn start(config: ThreadSafeConfig,game: ThreadSafeGame) -> Self;
    fn update(&mut self);
    fn exit(&mut self);
}

pub struct HackRunnable{
    thread: JoinHandle<()>,
    enabled: Arc<AtomicBool>,
}
impl HackRunnable{
    pub fn start<F>(mut fun: F) -> Self
    where F: 'static + Send + FnMut(Arc<AtomicBool>) -> (){
        let _override = Arc::new(AtomicBool::new(true)); 
        let on = Arc::clone(&_override);
        let handle = thread::spawn(move || {
            
            fun(on);
        });
        Self { thread: handle, enabled:_override }
    }

    pub fn stop(self){
        self.enabled.store(false, Ordering::Relaxed);
        self.thread.join();
    }

}
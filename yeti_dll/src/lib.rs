use std::{time::Duration, process::exit, thread};

use ipmpsc::{SharedRingBuffer, Receiver};
use log::{debug, error, info};
use windows::Win32::System::Console::AllocConsole;
use yeti_lib::{sized_string::SizedString, imc::Packet};
extern crate num;
#[macro_use]
extern crate num_derive;
use crate::yeti_hack_manager::Yeti;
mod yeti_hack_manager;
mod error;
mod hacks;
mod api;


//RULES
//this dll should never panic always log and cleanly exit.
//speed is main priority
//hack loop is number one priority
//no hiccups -> nothing should holdup the thread 100% throughput
//? make each specfic hack a thread ps. for the reason above
//? Config is optimized so each hack only has a copy of the Config it needs
#[mem::dll_main]
fn main() {
    unsafe{AllocConsole()};
    
    // This is a fully functional DLL ready for injection!
    let (map_file,logging_path) = unsafe{*(lp_reserved as *const (SizedString::<155>,SizedString::<155>))}.into();
    let map_file:String = map_file.into();
    let logging_path:String = logging_path.into();
    println!("Loggin config path is {}",logging_path);
    log4rs::init_file(logging_path,Default::default()).unwrap();
    info!("Inside process finish intialization");
    debug!("Map File is {:?}",map_file); 
    let yeti_ok= Yeti::new(&map_file);
    if let Err(err) = yeti_ok{
        error!("Could not initialize Yeti returned with error:{}",err);
        return;
    }
    let mut yeti = yeti_ok.unwrap();
    info!("Intialization done starting hack loop");
    loop{
        let result = yeti.update();
        if let Err(err) = result{
            debug!("Yeti exiting with err {}",err);
            break ;
        }

    }
    debug!("Good bye world");


}


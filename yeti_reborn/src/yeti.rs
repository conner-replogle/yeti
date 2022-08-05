use std::{error::Error, path::PathBuf};

use dll_needle::target_process::TargetProcess;
use hazedumper::haze_dumper;
use ipmpsc::{Sender, SharedRingBuffer};
use log::{info, debug};
use yeti_lib::{sized_string::SizedString, imc::Packet, hack_config::YetiHackConfig, signatures::Signatures};
use serde::ser::Serialize;
use crate::error::YetiError;




pub struct Yeti{
    process: Option<TargetProcess>,
    sender: Option<Sender>,
    pub injected: bool,
    pub signatures: Option<Signatures>,

}
impl Yeti{
    pub fn new() -> Result<Yeti,YetiError>{
        match TargetProcess::new("csgo.exe"){
            Ok(process) => {
                Ok(Yeti{
                    process: Some(process),
                    sender: None,
                    injected: false,
                    signatures: None,
                })
            },
            Err(err) => {Err(YetiError::ProcessAcquisitionError(err))},
        }
    }
}
impl Yeti{
    pub fn signatures_scan(config_file: &str,sig_file:&str) -> Signatures {
        let mut sig = Signatures::new(haze_dumper(config_file.to_string()));
        sig.save(sig_file).unwrap();
        return sig;
    }
    pub fn start(&mut self,dll_path: &PathBuf,config_file:&PathBuf) -> Result<(),YetiError >{
        if self.injected{
            return Err(YetiError::AlreadyInjected);
        }
        if let Some(process) = &self.process{
            let (name,buffer) = SharedRingBuffer::create_temp(bincode::serialized_size(self.signatures.as_ref().unwrap()).unwrap() as u32 +64).unwrap();
            self.sender = Some(Sender::new(buffer));
            match process.inject_dll(dll_path.canonicalize().unwrap().to_str().unwrap(),(SizedString::<155>::try_from(name.as_str()).unwrap(),SizedString::<155>::try_from(config_file.canonicalize().unwrap().to_str().unwrap()).unwrap())){
                Ok(_) => {
                    
                    info!("Yeti Started and injected");

                    if let Some(sender) = &self.sender{
                        sender.send(&Packet{
                            message_type: yeti_lib::imc::MessageType::SIGNATURE,
                        }).unwrap();
                    sender.send(self.signatures.as_ref().unwrap()).unwrap();
                    }
                    
                    self.injected = true;
                    return Ok(());
                }
                Err(err) => {return Err(YetiError::InjectionError(err))},
            }
        }
        Err(YetiError::ProcessNotAcquired)
    }

    pub fn update_config(&mut self,config: &YetiHackConfig){
        if let Some(sender) = &self.sender{
            sender.send(&Packet{
                message_type: yeti_lib::imc::MessageType::CONFIG,
            }).unwrap();
            sender.send(&config).unwrap();
            debug!("Config sent to be updated");
        }
    }
    pub fn update_signatures(&mut self,sig: Signatures){
        if let Some(sender) = &self.sender{
            sender.send(&Packet{
                message_type: yeti_lib::imc::MessageType::SIGNATURE,
            }).unwrap();
            sender.send(&sig).unwrap();
            
        }
        debug!("Signatures sent to be updated");
        self.signatures = Some(sig);
    }
    pub fn stop(&mut self){
        if let Some(sender) = &self.sender{
            sender.send(&Packet{
                message_type: yeti_lib::imc::MessageType::STATUS,
            }).unwrap();
            sender.send(&yeti_lib::imc::STATUS_MESSAGE::STOP).unwrap();
            debug!("Stopped thread");
            self.injected = false;
            self.sender = None;
        }
    }
}
impl  Drop for Yeti {
    fn drop(&mut self){
        self.stop()
    }
}
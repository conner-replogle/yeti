

use std::fs;

use chrono::{DateTime, Utc, TimeZone};
use hazedumper::output::{self, Map, Results};
use serde::{Deserialize, Serialize};
#[derive(Deserialize,Serialize,Debug)]
pub struct Signatures{
    sigs: Map<usize>,
    netvars: Map<isize>,
    pub timestamp: DateTime<Utc>
}
impl Default for Signatures{
    fn default() -> Self {
        Self { sigs: Default::default(), netvars: Default::default(), timestamp: Utc.timestamp(61, 0) }
    }
}
impl Signatures{
    pub fn new(result: output::Results) -> Signatures{
        return Self{
            sigs: result.signatures,
            netvars: result.netvars.as_ref().unwrap().clone(),
            timestamp: result.timestamp
        }

    }
    pub fn load(sig_file:&str) -> Option<Signatures>{
        match fs::read_to_string(sig_file){
            Ok(contents) => {
                let result: Self = Self::new(serde_json::from_str(&contents).unwrap());
                println!("Loaded Signatures.");
                return Some(result);

            },
            Err(err) => {
                println!("error loading sigs(no sig file possibly??) run sig scan in menu: {}",err);
                
            }
        }
        None

    }
    
}
impl Signatures{
    pub fn get_s(&self,offset_name: &str) -> usize{
        match self.sigs.get(offset_name){
            Some(offset) => {return *offset},
            None => {
                println!("Error finding offset {}",offset_name);
                return 0;
            },
        
        }
    }
    pub fn get_n(&self,netvar_name: &str) -> usize{
        match self.netvars.get(netvar_name){
            Some(offset ) => {return *offset as usize}
            None => {
                println!("Error finding netvar {}",netvar_name);
                return 0;
            },
        
        }
    }
    pub fn save(&mut self,sig_file: &str) -> Result<(),std::io::Error>{
        output::Results::new(self.sigs.clone(), Some(self.netvars.clone())).dump_json(sig_file)
    }
}
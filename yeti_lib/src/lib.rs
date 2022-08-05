pub mod imc;
pub mod hack_config;
pub mod app_config;
pub mod sized_string;
pub mod signatures;
pub mod error;
use std::{fs::{File, OpenOptions}, io::{Error, Read, Write}, path::PathBuf};

use serde::{Deserialize, Serialize};
pub trait Saveable:Serialize {
    const CONFIG_NAME:&'static str;
    fn open_file<'a>(config_path:&PathBuf,buf:&'a mut Vec<u8>) -> Result<Self,error::Error > where Self: Sized,Self: Deserialize<'a>{
        let opt = File::open(config_path);
        if let Ok(mut file) = opt{
            file.read_to_end(buf).unwrap();
            let config:Result<Self, serde_json::Error> = serde_json::from_slice(buf);
            if let Ok(config) = config{
                Ok(config)
            }else{
                Err(error::Error::LoadingConfigFailed(Self::CONFIG_NAME))
            }
            
        }else{
            return Err(error::Error::LoadingConfigFailed(Self::CONFIG_NAME));
        }
    }
    fn open_file_or_create<'a>(config_path:&PathBuf,buf:&'a mut Vec<u8>) -> Self where Self: Sized, Self: Deserialize<'a>,Self: Default,Self: Serialize{
        let app = Self::open_file(config_path,buf);
        if let Ok(app) = app{
            app
        }else{
            let config = Self::default();
            //TODO remove unwraps??
            let mut file = File::create(config_path).unwrap();
            
            let buf = serde_json::to_vec_pretty(&config).unwrap();
            file.write(&buf).unwrap();


            config
        }
    }
    fn save<'a>(&self,config_path:&PathBuf){
        let file = OpenOptions::new().write(true).open(config_path);
        if let Ok(mut file) = file{
            let buf = serde_json::to_vec_pretty(&self).unwrap();
            file.write(&buf).unwrap();
            
        }else{
            let mut file = File::create(config_path).unwrap();
            let buf = serde_json::to_vec_pretty(&self).unwrap();
            file.write(&buf).unwrap();

        }
        
        

    }
}
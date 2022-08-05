
use serde::Deserialize;


#[derive(Deserialize,serde::Serialize,Debug)]
pub struct Packet{
    pub message_type: MessageType
}

#[derive(Deserialize,serde::Serialize,Debug)]
pub enum MessageType{
    CONFIG,
    STATUS,
    SIGNATURE
    

}

#[derive(Deserialize,serde::Serialize,Debug)]
pub enum STATUS_MESSAGE{
    STOP
    

}
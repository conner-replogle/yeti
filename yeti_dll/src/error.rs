use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error{
    #[error("Intializing Shared Memory error: {0}")]
    InitSharedMemory(ipmpsc::Error),

    #[error("Missing expected packet error: {0}")]
    MissingPacket(ipmpsc::Error),
    #[error("Was told to die")]
    KILL
}
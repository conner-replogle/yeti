use thiserror::Error;




#[derive(Error,Debug)]
pub enum Error{
    #[error("Error loading the config: {0}")]
    LoadingConfigFailed(&'static str)
}
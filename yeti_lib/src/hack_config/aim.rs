use serde::{Deserialize, Serialize};
#[derive(Deserialize,Serialize,Debug,Copy,Clone)]
pub struct AimConfig{
    pub mode: AimMode,
    pub targeting_mode: TargetingMethod,

}
impl Default for AimConfig{
    fn default() -> Self {
        Self { mode: AimMode::FOV ,targeting_mode: TargetingMethod::CLOSEST}
    }
}


#[derive(Deserialize,Serialize,Debug,Copy,Clone)]
pub enum AimMode{
    FOV,
    SNAP,
}

#[derive(Deserialize,Serialize,Debug,Copy,Clone)]
pub enum TargetingMethod{
    CLOSEST,
    FOV_ALGO,

}
impl PartialEq for TargetingMethod{
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

pub enum Bones{
    HEAD =8,
    //etcccc
}


#[derive(Deserialize,Serialize,Debug,Copy,Clone)]
pub struct AimFovConfig{
    fov: f32,
    //other settings.... calc angle func

}
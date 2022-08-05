use windows::{Win32::{System::LibraryLoader::GetModuleHandleW, Foundation::HINSTANCE}, core::{HSTRING, PCWSTR}};


use crate::yeti_hack_manager::ThreadSafeSignature;
#[derive(Clone,Debug)]
pub struct Engine(pub u32,pub ThreadSafeSignature);
impl Engine{
    pub fn new(sig: ThreadSafeSignature) -> Self{
        let engine = unsafe{(*((&mut GetModuleHandleW(PCWSTR::from(&HSTRING::from("engine.dll"))).unwrap()) as *mut HINSTANCE as *mut u32) as *mut u32) as u32};
        Engine(engine,sig)
    }
        

}
impl Engine{
    pub fn dwClientState(&self) -> &mut u32{
        unsafe{&mut *((self.0 + self.1.load().get_s("dwClientState") as u32) as *mut u32)}
    }
    pub fn dwClientState_State(&self) -> Option<ClientState>{
       let a =  unsafe{*((*self.dwClientState()+self.1.load().get_s("dwClientState_State") as u32) as *const u32) };
       num::FromPrimitive::from_u32(a)

    }
}
#[derive(Copy, Clone,FromPrimitive,Debug)]
pub enum ClientState{
    LOBBY = 0,
    LOADING = 1,
    CONNECTING = 2,
    CONNECTED = 5,
    IN_GAME = 6
}



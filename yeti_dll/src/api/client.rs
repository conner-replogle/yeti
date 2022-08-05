use windows::{Win32::{System::LibraryLoader::GetModuleHandleW, Foundation::HINSTANCE}, core::{HSTRING, PCWSTR}};

use crate::yeti_hack_manager::ThreadSafeSignature;

use super::datatypes::GlowStruct;
#[derive(Clone,Debug)]
pub struct Client(pub u32,pub ThreadSafeSignature);
impl Client{
    pub fn new(sig: ThreadSafeSignature) -> Self{
        let client = unsafe{(*((&mut GetModuleHandleW(PCWSTR::from(&HSTRING::from("client.dll"))).unwrap()) as *mut HINSTANCE as *mut u32) as *mut u32) as u32};
        Client(client,sig)
    }
        

}
impl Client{
    pub fn dwForceJump(&self) -> &mut u32{
        unsafe{&mut *((self.0 + self.1.load().get_s("dwForceJump") as u32) as *mut u32)}
    }
    pub fn dwGlowObjectManager(&self) -> u32{
        unsafe{*((self.0 + self.1.load().get_s("dwGlowObjectManager") as u32) as *mut u32)}
    }
    pub fn get_glowstruct(&self,glow_index: u32) -> &mut GlowStruct{
        unsafe{&mut *((self.dwGlowObjectManager() + (glow_index * 0x38)+0x8) as *mut GlowStruct)}
    }
    pub fn dwEntityList(&self,i:u32) -> u32{
        unsafe{*((self.0 + self.1.load().get_s("dwEntityList") as u32 + (i * 0x10)) as *mut u32)}
    }
}
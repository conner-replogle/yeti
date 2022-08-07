use log::debug;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;
use windows::Win32::{Foundation::HANDLE, UI::WindowsAndMessaging::FindWindowW};
use windows::core::{PCWSTR,HSTRING};
#[derive(Clone,Debug)]
pub struct Window{
    handle: HWND,
    pub window_width: f32,
    pub window_height: f32,
}
impl Window{
    pub fn start() -> Self{
        let handle = unsafe{FindWindowW(PCWSTR::null(),PCWSTR::from(&HSTRING::from("Counter-Strike: Global Offensive - Direct3D 9")))};
        let (window_w,window_h) = Self::get_window_size(&handle);
        debug!("Window Size: {}, {} ",window_w,window_h);
        Self { handle: handle, window_width: window_w, window_height: window_h }
    }
    pub fn get_window_size(handle: &HWND) -> (f32,f32){
        let mut rect = RECT::default();
        unsafe{GetClientRect(*handle, &mut rect as *mut RECT)};

        let window_w:f32 = (rect.right - rect.left) as f32;
        let window_h:f32 = (rect.bottom - rect.top) as f32;
        return (window_w,window_h);
    }
}
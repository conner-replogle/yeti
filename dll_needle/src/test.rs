#[cfg(test)]
mod tests {
    use crate::target_process::TargetProcess;


    #[test]
    fn inject_dll() {
        unsafe{
        let process = TargetProcess::new("Target.exe",).unwrap();
        let a = 1234565;
        process.inject_dll("C:\\Users\\allep\\dev\\Rust\\manual-map-injector\\test\\SampleDll.dll",a).unwrap();
        }
    
    }
}
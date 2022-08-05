
#[derive(Debug)]
pub enum YetiError{
    InjectionError(dll_needle::mmierror::InjectionError),
    ProcessNotAcquired,
    ProcessAcquisitionError(dll_needle::mmierror::ProcessError),
    AlreadyInjected,

}
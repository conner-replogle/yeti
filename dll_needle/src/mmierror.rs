use std::{fmt, io::Error};

use winapi::{um::winnt::HANDLE, shared::minwindef::DWORD};



#[derive(Debug)]
pub enum ProcessError {
    ProcessNotFound,
    InvalidHandle(HANDLE)

}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProcessError::ProcessNotFound =>
                write!(f, "Process with that name could not be found"),
            // The wrapped error contains additional information and is available
            // via the source() method.
            ProcessError::InvalidHandle(e) =>
                write!(f, "Invalid Handle {:?}",e),
        }
    }
}

#[derive(Debug)]
pub enum InjectionError {
    ReadingDLL(Error),
    DLLCompatibility(String),
    AllocationError(DWORD)

}

impl fmt::Display for InjectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            InjectionError::ReadingDLL(a) =>
                write!(f, "Could not load DLL with error {}",a),
            // The wrapped error contains additional information and is available
            // via the source() method.
            InjectionError::DLLCompatibility(e) =>
                write!(f, "Dll was incompatible {:?}",e),
            InjectionError::AllocationError(e) =>
                write!(f, "Allocating memory in target process failed with error: {:?}",e),
        }
    }
}
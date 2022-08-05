use std::{fs::File, io::{BufReader, self, Write}, ptr, mem::{self, transmute_copy}, fmt::Error, intrinsics::transmute};

use target_process::{TargetProcess};
use winapi::{um::{winnt::{HANDLE, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, IMAGE_NT_HEADERS32, IMAGE_SECTION_HEADER, IMAGE_DOS_HEADER, IMAGE_FILE_HEADER, IMAGE_FILE_MACHINE_I386, PROCESS_ALL_ACCESS, PIMAGE_NT_HEADERS32, PIMAGE_BASE_RELOCATION, PIMAGE_IMPORT_DESCRIPTOR, PCSTR, PVOID, IMAGE_DIRECTORY_ENTRY_BASERELOC, IMAGE_DIRECTORY_ENTRY_IMPORT, PIMAGE_SECTION_HEADER, PIMAGE_DOS_HEADER, MEM_RELEASE}, memoryapi::{VirtualAllocEx, WriteProcessMemory, VirtualFreeEx}, tlhelp32::{CreateToolhelp32Snapshot, PROCESSENTRY32, TH32CS_SNAPPROCESS, Process32First, Process32Next}, handleapi::{INVALID_HANDLE_VALUE, CloseHandle}, processthreadsapi::{OpenProcess, CreateRemoteThread}, synchapi::WaitForSingleObject, errhandlingapi::GetLastError, libloaderapi::{GetProcAddress, LoadLibraryA}, minwinbase::LPTHREAD_START_ROUTINE}, ctypes::c_void, shared::minwindef::{HINSTANCE, FARPROC, LPVOID, LPBYTE}, };
mod test;
pub mod target_process;
pub mod mmierror;





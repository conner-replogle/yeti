use std::{mem, fs::File, io::{BufReader, self}, ptr, intrinsics::transmute};
use log::{error, info, warn,debug, trace};
use winapi::{um::{winnt::{HANDLE, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, IMAGE_NT_HEADERS32, IMAGE_SECTION_HEADER, IMAGE_DOS_HEADER, IMAGE_FILE_HEADER, IMAGE_FILE_MACHINE_I386, PROCESS_ALL_ACCESS, PIMAGE_NT_HEADERS32, PIMAGE_BASE_RELOCATION, PIMAGE_IMPORT_DESCRIPTOR, PCSTR, PVOID, IMAGE_DIRECTORY_ENTRY_BASERELOC, IMAGE_DIRECTORY_ENTRY_IMPORT, PIMAGE_SECTION_HEADER, PIMAGE_DOS_HEADER, MEM_RELEASE, RtlZeroMemory, PAGE_READWRITE}, memoryapi::{VirtualAllocEx, WriteProcessMemory, VirtualFreeEx}, tlhelp32::{CreateToolhelp32Snapshot, PROCESSENTRY32, TH32CS_SNAPPROCESS, Process32First, Process32Next}, handleapi::{INVALID_HANDLE_VALUE, CloseHandle}, processthreadsapi::{OpenProcess, CreateRemoteThread, GetExitCodeProcess}, synchapi::WaitForSingleObject, errhandlingapi::GetLastError, libloaderapi::{GetProcAddress, LoadLibraryA}, minwinbase::LPTHREAD_START_ROUTINE}, ctypes::c_void, shared::minwindef::{HINSTANCE, FARPROC, LPVOID, LPBYTE}, };

use crate::mmierror::{ProcessError, InjectionError};
#[repr(C)]
#[derive(Debug)]
struct LoaderData{
    image_base: *mut c_void,
    nt_headers: PIMAGE_NT_HEADERS32,
    base_reloc: PIMAGE_BASE_RELOCATION,
    import_directory: PIMAGE_IMPORT_DESCRIPTOR,
    fn_get_proc_address: LPVOID,
    fn_load_library_a: LPVOID,
    extra_data_ptr: LPVOID

}

extern "C"{
    fn write_loader_data(data:*mut LoaderData);
}
extern "stdcall" {
    fn library_loader(Memory:LPVOID);
    fn stub();

}



unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}
#[derive(Debug)]
pub struct TargetProcess{
    process_handle: HANDLE,

}
impl TargetProcess{
    pub fn new(process_name:&str,) -> Result<TargetProcess,ProcessError>{
        let process = unsafe{TargetProcess::find_process(process_name).unwrap()};
        
        Ok(TargetProcess{
            process_handle: process,
            
        })
    }
    unsafe fn find_process(process_name:&str) -> Result<HANDLE, ProcessError>{
        let mut process_info = PROCESSENTRY32{
            dwSize: 0,
            cntUsage: 0,
            th32ProcessID: 0,
            th32DefaultHeapID: 0,
            th32ModuleID: 0,
            cntThreads: 0,
            th32ParentProcessID: 0,
            pcPriClassBase: 0,
            dwFlags: 0,
            szExeFile: [0i8;260],
        };
        
        process_info.dwSize = mem::size_of_val(&process_info) as u32;
    
        let process_snapshot: HANDLE = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if process_snapshot == INVALID_HANDLE_VALUE{
            return Err(ProcessError::InvalidHandle(process_snapshot));
        }
            
        Process32First(process_snapshot, &mut process_info as *mut PROCESSENTRY32);
        
        let curr_proc_name = process_info.szExeFile.iter().map(|c| *c as u8 as char ).collect::<String>();
        
        
        trace!("Checking Process: {}",curr_proc_name);
        if curr_proc_name.contains(process_name)
        {
            debug!("Found process {}",process_name);
            CloseHandle(process_snapshot);
            let h_process = OpenProcess(PROCESS_ALL_ACCESS, 0, process_info.th32ProcessID);
            return Ok(
                h_process
            );
        }
    
        while Process32Next(process_snapshot, &mut process_info as *mut PROCESSENTRY32) != 0
        {
            let curr_proc_name = process_info.szExeFile.iter().map(|c| *c as u8 as char ).collect::<String>();
            trace!("Checking Process: {}",curr_proc_name);
            if curr_proc_name.contains(process_name)
            {
                debug!("Found process {}",process_name);
                CloseHandle(process_snapshot);
                let h_process = OpenProcess(PROCESS_ALL_ACCESS, 0, process_info.th32ProcessID);
                return Ok(h_process
                );
            }
            process_info.szExeFile = [0i8;260];
    
        }
    
        CloseHandle(process_snapshot);
        return Err(ProcessError::ProcessNotFound);
    }
}
impl TargetProcess{
    pub fn inject_dll<A: std::fmt::Debug>(&self,dll_name: &str,extra_data:A) -> Result<(),InjectionError>{
        unsafe{
        let f = match File::open(dll_name) {
            Ok(a) => {a},
            Err(err) => {
                return Err(InjectionError::ReadingDLL(err));
            },
        };
        trace!("Opened File");
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        
        // Read file into vector.
        if let Err(err) = io::Read::read_to_end(&mut reader, &mut buffer){
            return Err(InjectionError::ReadingDLL(err));
        }
        debug!("Read File");
        let buffer_slice = buffer.as_slice();
        let ptr = buffer_slice.as_ptr() as *const c_void;
    
        /* Check file signature */
        let pimage_dos_header = *(ptr as PIMAGE_DOS_HEADER);
        if pimage_dos_header.e_magic != 0x5A4D
        {
            return Err(InjectionError::DLLCompatibility(format!("file signature e_magic={} supposed to be 0x5A4D",pimage_dos_header.e_magic)));
        }
    
        let p_old_nt_header = (ptr as LPBYTE).add(pimage_dos_header.e_lfanew as usize) as PIMAGE_NT_HEADERS32;
        let p_old_opt_header = &(*p_old_nt_header).OptionalHeader;
        let p_old_file_header = &(*p_old_nt_header).FileHeader;
        
        if p_old_file_header.Machine != IMAGE_FILE_MACHINE_I386{//ARCH OF MACHINE 32Bit
            return Err(InjectionError::DLLCompatibility(format!("wrong arch expected 332 got: {}",p_old_file_header.Machine)));
        }

        debug!("DLL checks are complete");

        let ptarget_base = VirtualAllocEx(self.process_handle, ptr::null_mut() , 
        p_old_opt_header.SizeOfImage as usize, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
    
        if ptarget_base as usize == 0
        {
            //println!( "OOPS! We ran into some problems... #493 ({:?})",GetLastError());
            return Err(InjectionError::AllocationError(GetLastError()));
        };
    
        debug!( "Allocated {} bytes in target process at {:?}" ,p_old_opt_header.SizeOfImage ,ptarget_base);

        WriteProcessMemory(self.process_handle, ptarget_base, ptr as *const c_void, (*p_old_nt_header).OptionalHeader.SizeOfHeaders as usize, ptr::null_mut());
        
        
        let p_sect_header = p_old_nt_header.add(1) as PIMAGE_SECTION_HEADER;
        debug!("P Sect Header Name : {:?}",String::from_utf8( Vec::from((*p_sect_header).Name)));


        //Copying sections of the dll to the target process
        for i in 0..(*p_old_nt_header).FileHeader.NumberOfSections
        {
            let section = *p_sect_header.add(i as usize);
            let address = (ptarget_base as LPBYTE).add(section.VirtualAddress as usize) as PVOID;
            let name = String::from_utf8(Vec::from(section.Name)).unwrap();
            let mut bytes_written:usize = 0;
            WriteProcessMemory(self.process_handle, address,
                (ptr as LPBYTE).add(section.PointerToRawData as usize) as PVOID, section.SizeOfRawData as usize, &mut bytes_written as _);
            debug!("Writing {} bytes of section {} at address {:?}",bytes_written,name,address);
        }
        //EXTRA DATA WRITING

        let bytes_written:usize = 0;
        let extra_data_addy = VirtualAllocEx(self.process_handle, ptr::null_mut(), mem::size_of_val(&extra_data), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
        WriteProcessMemory(self.process_handle, extra_data_addy, &extra_data as *const A as LPVOID, mem::size_of_val(&extra_data), bytes_written  as *mut usize);

        let mut loader_data = LoaderData{
            image_base: ptarget_base,
            nt_headers: (ptarget_base as LPBYTE).add(pimage_dos_header.e_lfanew as usize) as PIMAGE_NT_HEADERS32,
            base_reloc:((ptarget_base as LPBYTE).add((*p_old_nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_BASERELOC as usize].VirtualAddress as usize) as PIMAGE_BASE_RELOCATION),
            import_directory: ((ptarget_base as LPBYTE).add((*p_old_nt_header).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT as usize].VirtualAddress as usize) as PIMAGE_IMPORT_DESCRIPTOR),
            
            fn_get_proc_address: 0 as LPVOID,
            fn_load_library_a: 0 as LPVOID,
            extra_data_ptr: extra_data_addy as LPVOID,
        };
        write_loader_data(&mut loader_data as *mut LoaderData);
        //println!("loader_data: {:?}",loader_data);
        let mut bytes_written:usize = 0;
        let loader_memory = VirtualAllocEx(self.process_handle, ptr::null_mut(), 4096, MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE);
        //let loader_memory = write_loader_data(loader_data);
        WriteProcessMemory(self.process_handle, loader_memory, &loader_data as *const LoaderData as LPVOID, mem::size_of_val(&loader_data),bytes_written as *mut usize);

        debug!("wrote {} bytes at {:p} containing loaderdata ",bytes_written,loader_memory);

        let library_loader_addy = (loader_memory as *const LoaderData).add(1) as *mut c_void;
        //WRITING LIBRARY LOADER FUNC
        WriteProcessMemory(self.process_handle, library_loader_addy ,
            (library_loader as *const extern "C" fn()) as *const c_void, (stub as *const c_void as usize)-(library_loader as *const c_void as usize), &mut bytes_written as _);
        

        let remote_thread_start:LPTHREAD_START_ROUTINE = Some(transmute(library_loader_addy));
        debug!("Wrote LibraryLoader with mem size: {} at address {:p}",bytes_written,library_loader_addy);
        
        let h_thread = CreateRemoteThread(self.process_handle, ptr::null_mut(), 0, remote_thread_start,
        loader_memory, 0, ptr::null_mut());
        //let h_thread = call_lib(process, loader_memory);
        debug!("Created h_thread {:?}",h_thread);
        
        WaitForSingleObject(h_thread, 99999999);
        let mut return_code = -1;
        if GetExitCodeProcess(h_thread,return_code as *mut u32) == 0{
            GetLastError();
            debug!("Thread ended but errored on getting return code {}",GetLastError());
        }else{
            debug!("Thread ended with code {} ",return_code);
        }
       
        VirtualFreeEx(self.process_handle, loader_memory, 0, MEM_RELEASE);
        Ok(())
        }
    }
}
// const generics are not complete and only available in nightly
#![allow(incomplete_features)]
#![feature(const_generics)]

use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::ptr::null_mut;
use thiserror::Error;
use winapi::ctypes::c_char;
use winapi::shared::minwindef::{DWORD, FALSE, HMODULE, LPVOID, MAX_PATH};
use winapi::shared::ntdef::{HANDLE, LPSTR, NULL};
use winapi::um::errhandlingapi;
use winapi::um::memoryapi;
use winapi::um::processthreadsapi;
use winapi::um::psapi;
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

#[derive(Debug, Error)]
pub enum ProcessError {
    #[error("Couldn't enumerate processes: {0}")]
    ProcessEnumeration(u32),
    #[error("Couldn't enumerate modules for handle {0:x}: {1}")]
    ModuleEnumeration(u32, u32),
    #[error("Failed to get module name for handle {0:x}: {1}")]
    ModuleName(u32, u32),
    #[error("Failed to get module information for '{0}': {1}")]
    ModuleInformation(String, u32),
    #[error("Process '{0}' not found")]
    NotFound(String),
}

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Couldn't read memory at {0:x}: {1} (read: {2})")]
    Read(u64, u32, usize),
    #[error("Incorrect read size (expected: {0}, actual: {1})")]
    IncorrectSize(usize, usize),
    #[error("Unable to find signature")]
    NotFound,
}

#[derive(Default, Debug)]
pub struct ProcessModule {
    pub name: String,
    pub base: u64,
    pub size: usize,
}

#[derive(Debug)]
pub struct Process {
    pub name: String,
    pub handle: HANDLE,
    pub modules: Vec<ProcessModule>,
}

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum SignatureType {
    // The address is the address of signature location plus the provided offset.
    Absolute { offset: i64 },
    // The address is the address of the signature location plus a u32 value read at offset
    Relative32 { offset: i64 },
}

impl Default for SignatureType {
    fn default() -> Self {
        SignatureType::Absolute { offset: 0 }
    }
}
#[derive(Default, Debug)]
pub struct Signature<'a> {
    pub bytes: &'a [&'a str],
    pub sigtype: SignatureType,
}

impl Process {
    pub fn new(exe_name: &str) -> Result<Self, ProcessError> {
        let mut processes: Vec<DWORD> = vec![0; 1024];
        let mut needed = 0;

        unsafe {
            if psapi::EnumProcesses(
                processes.as_mut_ptr(),
                processes.len() as DWORD,
                &mut needed,
            ) == FALSE
            {
                return Err(ProcessError::ProcessEnumeration(
                    errhandlingapi::GetLastError(),
                ));
            }

            for i in 0..(needed as usize / mem::size_of::<DWORD>()) {
                let handle = processthreadsapi::OpenProcess(
                    PROCESS_VM_READ | PROCESS_QUERY_INFORMATION,
                    FALSE,
                    processes[i],
                );
                if handle == NULL {
                    continue;
                }

                let mut name_buf: Vec<i8> = vec![0; MAX_PATH];
                if psapi::GetModuleBaseNameA(
                    handle,
                    0 as HMODULE,
                    name_buf.as_mut_ptr(),
                    name_buf.len() as DWORD,
                ) == 0
                {
                    continue;
                }

                let name_str = CStr::from_ptr(name_buf.as_ptr() as *const c_char)
                    .to_string_lossy()
                    .to_string();
                if name_str == exe_name {
                    let modules = Process::get_process_modules(handle)?;
                    return Ok(Process {
                        name: name_str,
                        handle,
                        modules,
                    });
                }
            }
        }

        Err(ProcessError::NotFound(exe_name.to_string()))
    }

    fn get_process_modules(hnd: HANDLE) -> Result<Vec<ProcessModule>, ProcessError> {
        let mut result: Vec<ProcessModule> = vec![];
        unsafe {
            let mut modules: [HMODULE; 1024] = [null_mut(); 1024];
            let mut needed: DWORD = 0;
            if psapi::EnumProcessModulesEx(
                hnd,
                modules.as_mut_ptr(),
                (mem::size_of::<HMODULE>() * modules.len()) as DWORD,
                &mut needed,
                psapi::LIST_MODULES_64BIT,
            ) == FALSE
            {
                return Err(ProcessError::ModuleEnumeration(
                    hnd as u32,
                    errhandlingapi::GetLastError(),
                ));
            }

            let mut buf = [0; MAX_PATH];
            for i in 0..(needed as usize / mem::size_of::<HMODULE>()) {
                if psapi::GetModuleBaseNameA(
                    hnd,
                    modules[i],
                    buf.as_mut_ptr() as LPSTR,
                    buf.len() as u32,
                ) == 0
                {
                    return Err(ProcessError::ModuleName(
                        hnd as u32,
                        errhandlingapi::GetLastError(),
                    ));
                }

                let name = CStr::from_ptr(buf.as_ptr() as *const _)
                    .to_string_lossy()
                    .to_owned()
                    .to_string();

                let mut module_info = psapi::MODULEINFO::default();
                if psapi::GetModuleInformation(
                    hnd,
                    modules[i],
                    &mut module_info,
                    mem::size_of::<psapi::MODULEINFO>() as DWORD,
                ) == FALSE
                {
                    return Err(ProcessError::ModuleInformation(
                        name.to_string(),
                        errhandlingapi::GetLastError(),
                    ));
                }

                result.push(ProcessModule {
                    name: name.clone(),
                    base: module_info.lpBaseOfDll as u64,
                    size: module_info.SizeOfImage as usize,
                });
            }
        }
        Ok(result)
    }

    pub fn read(
        &self,
        addr: u64,
        buf: *mut u8,
        sz: usize,
        read: &mut usize,
    ) -> Result<(), MemoryError> {
        unsafe {
            if memoryapi::ReadProcessMemory(self.handle, addr as LPVOID, buf as LPVOID, sz, read)
                == FALSE
            {
                return Err(MemoryError::Read(
                    addr as u64,
                    errhandlingapi::GetLastError(),
                    *read,
                ));
            }
        }
        Ok(())
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct UnknownField<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> fmt::Debug for UnknownField<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            for byte in self.data.iter() {
                write!(f, " {:02x}", byte)?;
            }
        }
        Ok(())
    }
}

impl<const N: usize> PartialEq for UnknownField<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            for pair in self.data.iter().zip(other.data.iter()) {
                if pair.0 != pair.1 {
                    return false;
                }
            }
        }
        true
    }
}

impl<const N: usize> Default for UnknownField<N> {
    fn default() -> UnknownField<N> {
        UnknownField { data: [0; N] }
    }
}

impl<const N: usize> Eq for UnknownField<N> {}

use std::ops::Deref;
impl<'a, T> Deref for RemoteStruct<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.t }
    }
}

#[repr(C, packed)]
pub struct RemoteStruct<'a, T> {
    t: T,
    module: usize, // if we need to use other modules someday
    address: u64,
    process: &'a Process,
}

impl<'a, T: std::default::Default> RemoteStruct<'a, T> {
    pub fn new(process: &'a Process, address: u64) -> Self {
        println!(
            "Creating new remote struct for address {:#x}",
            address + process.modules[0].base
        );
        RemoteStruct {
            t: T::default(),
            address,
            module: 0,
            process,
        }
    }

    pub fn read(&mut self) -> Result<(), MemoryError> {
        let t_size = mem::size_of::<T>();
        unsafe {
            let mut read = 0;
            let read_addr = self.process.modules[self.module].base + self.address;
            match memoryapi::ReadProcessMemory(
                self.process.handle,
                read_addr as LPVOID,
                &mut self.t as *mut _ as LPVOID,
                t_size,
                &mut read,
            ) {
                0 => Err(MemoryError::Read(
                    read_addr,
                    errhandlingapi::GetLastError(),
                    read,
                )),
                _ => {
                    if read != t_size {
                        Err(MemoryError::IncorrectSize(t_size, read))
                    } else {
                        Ok(())
                    }
                }
            }
        }
    }
}

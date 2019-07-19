use std::mem::size_of;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use num_traits::AsPrimitive;

#[link(name = "KeInterface")]
extern {
    fn Initialize() -> bool;
    fn Destroy();

    fn ReadVirtualMemory(process_id: u32, address: u64, size: usize, result: *mut u64) -> bool;
    fn WriteVirtualMemory(process_id: u32, address: u64, value: u64, size: usize) -> bool;
    fn GetBaseAddress(process_id: u32, module_name: *const u16, is_32bit: bool, result: *mut u64, size: *mut u32) -> bool;
    fn ProtectMemory(process_id: u32, base_address: u64, size: usize, new_protection: u32, result: *mut u32) -> bool;
    fn GetPID(process_name: *const u16, result: *mut u32) -> bool;
}

#[derive(Clone)]
pub struct KeInterface;

impl KeInterface {
    pub fn new() -> KeInterface {
        let init = unsafe {
            Initialize()
        };

        if !init {
            panic!("Error initializing connection to driver!");
        }

        KeInterface {}
    }

    pub fn read<T: 'static + Copy>(&self, process_id: u32, address: u64) -> T where u64: AsPrimitive<T> {
        let mut result: u64 = 0;

        let success = unsafe {
            ReadVirtualMemory(process_id, address, size_of::<T>(), &mut result)
        };

        if !success {
            panic!("Error reading value from memory!");
        }

        result.as_()
    }

    pub fn write<T: AsPrimitive<u64>>(&self, process_id: u32, address: u64, value: T) {
        let success = unsafe {
            WriteVirtualMemory(process_id, address, value.as_(), size_of::<T>())
        };

        if !success {
            panic!("Error writing value to memory!");
        }
    }

    pub fn base<T: 'static + Copy>(&self, process_id: u32, module_name: &str, is_32bit: bool) -> (T, u32) where u64: AsPrimitive<T> {
        let mut result: u64 = 0;
        let module: Vec<u16> = OsStr::new(module_name)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect();
        
        let mut size: u32 = 0;

        let success = unsafe {
            GetBaseAddress(process_id, module.as_ptr(), is_32bit, &mut result, &mut size)
        };

        if !success {
            panic!("Error getting base address of process!");
        }

        (result.as_(), size)
    }

    pub fn protect(&self, process_id: u32, base_address: u64, size: usize, new_protection: u32) -> u32 {
        let mut result: u32 = 0;

        let success = unsafe {
            ProtectMemory(process_id, base_address, size, new_protection, &mut result)
        };

        if !success {
            panic!("Error protecting region of memory!");
        }

        result
    }

    pub fn pid(&self, process_name: &str) -> u32 {
        let mut result: u32 = 0;
        let pname: Vec<u16> = OsStr::new(process_name)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect();
        
        let success = unsafe {
            GetPID(pname.as_ptr(), &mut result)
        };

        if !success {
            panic!("Error getting process ID!");
        }

        result
    }
}

impl Drop for KeInterface {
    fn drop(&mut self) {
        unsafe {
            Destroy();
        }
    }
}

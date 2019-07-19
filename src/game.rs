use std::mem;
use num_traits::AsPrimitive;

use crate::ke_interface::KeInterface;

#[derive(Clone)]
pub struct Game {
    mem: KeInterface,
    pub pid: u32,
    pub base: u64,
    base_size: u32,
    old_protection: u32,
}

impl Game {
    pub fn new() -> Self {
        let mem = KeInterface::new();
        let pid = mem.pid("MCC-Win64-Shipping.exe");
        let (base, base_size) = mem.base(pid, "haloreach.dll", false);

        // Make executable region read-writable
        let old_protection = mem.protect(pid, base, base_size as usize, 0x40);

        Game {
            mem,
            pid,
            base,
            base_size,
            old_protection,
        }
    }

    pub fn read<T: 'static + Copy>(&self, address: u64) -> T where u64: AsPrimitive<T> {
        self.mem.read::<T>(self.pid, address)
    }

    pub fn read_float(&self, address: u64) -> f32  {
        unsafe {
            mem::transmute(self.read::<u32>(address))
        }
    }

    pub fn read_bytes(&self, address: u64, size: usize) -> Vec<u8> {
        let mut bytes = Vec::new();

        for i in 0..size {
            bytes.push(self.read(address + i as u64));
        }

        bytes
    }

    pub fn write<T: AsPrimitive<u64>>(&self, address: u64, value: T) {
        self.mem.write(self.pid, address, value);
    }

    pub fn write_float(&self, address: u64, value: f32) {
        let uval: u32 = unsafe {
            mem::transmute(value)
        };

        self.write::<u32>(address, uval);
    }

    pub fn write_bytes(&self, address: u64, bytes: Vec<u8>) {
        for (i, byte) in bytes.iter().enumerate() {
            self.write::<u8>(address + i as u64, *byte);
        }
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        // Freezes the game, so don't worry about it
        // println!("Writing old protection: 0x{:x}", self.old_protection);
        // self.mem.protect(self.pid, self.base, self.base_size as usize, self.old_protection);
    }
}
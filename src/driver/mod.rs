use core::ptr::{read, write};

pub mod uart;
pub mod gpio;

/// Read something from mmio address
pub fn mmio_read(addr: usize) -> u32 {
    unsafe{
        let val = read(addr as *const u32);
        return val
    }
}

/// Write something to mmio address
pub fn mmio_write(addr: usize, val: u32) {
    unsafe{
        write(addr as *mut u32, val);
    }
}
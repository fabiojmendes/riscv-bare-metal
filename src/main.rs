#![no_std]
#![no_main]
#![feature(global_asm)]

use core::ptr;

mod panic;

global_asm!(include_str!("start.s"));

#[no_mangle]
pub extern "C" fn k_main() {
    const UART0: *mut u32 = 0x10013000 as *mut u32;
    let out_str = b"RISC-V Bare Metal\n";
    for &byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, byte as u32);
        }
    }
}

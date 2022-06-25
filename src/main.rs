#![no_std]
#![no_main]
use core::{arch::global_asm, ptr};

mod panic;

global_asm!(include_str!("start.s"));

#[no_mangle]
pub extern "C" fn k_main() {
    let out_str = b"RISC-V Bare Metal\n";
    for &byte in out_str {
        put_c(byte);
    }

    let mut v = [b'a'; 32];
    test1(&mut v);
}

fn put_c(char: u8) {
    // UART0 Address
    // [SIFIVE_E_DEV_UART0] = { 0x10013000, 0x1000 }
    // https://github.com/qemu/qemu/blob/a3607def89f9cd68c1b994e1030527df33aa91d0/hw/riscv/sifive_e.c#L61
    const UART0: *mut usize = 0x10013000 as *mut usize;
    unsafe {
        // Bytes should be written as 32 bits
        // otherwise qemu will crash
        ptr::write_volatile(UART0, char as usize);
    }
}

fn test1(v: &mut [u8]) {
    for (i, x) in v.iter_mut().enumerate() {
        *x = *x + i as u8;
    }
    test2(&v[..26])
}

fn test2(v: &[u8]) {
    for &c in v {
        put_c(c);
    }
    put_c(b'\n');
}

mod test {}

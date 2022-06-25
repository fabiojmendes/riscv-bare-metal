use core::ptr;

// UART0 Address
// [SIFIVE_E_DEV_UART0] = { 0x10013000, 0x1000 }
// https://github.com/qemu/qemu/blob/a3607def89f9cd68c1b994e1030527df33aa91d0/hw/riscv/sifive_e.c#L61
const UART0: *mut usize = 0x10013000 as *mut usize;

fn put_c(char: u8) {
    unsafe {
        // Bytes should be written as 32 bits
        // otherwise qemu will crash
        ptr::write_volatile(UART0, char as usize);
    }
}

pub fn print(text: impl AsRef<str>) {
    for c in text.as_ref().bytes() {
        put_c(c);
    }
}

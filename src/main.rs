#![no_std]
#![no_main]

mod boot;
mod panic;
mod printer;

#[no_mangle]
pub extern "C" fn k_main() {
    let out_str = "RISC-V Bare Metal\n";
    printer::print(out_str);

    let mut v = [b'a'; 32];
    test1(&mut v);
}

fn test1(v: &mut [u8]) {
    for (i, x) in v.iter_mut().enumerate() {
        *x += i as u8;
    }
    test2(&v[..26])
}

fn test2(v: &[u8]) {
    let s = unsafe { core::str::from_utf8_unchecked(v) };
    printer::print(s);
}

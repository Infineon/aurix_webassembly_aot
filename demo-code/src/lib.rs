#![no_std]
#![no_main]
mod led;
mod utils;
use core::panic::PanicInfo;

unsafe extern "C" {
    pub fn __write__(address: u32, value: u32);
    pub fn __read__(address: u32) -> u32;
    pub fn __write_str__(address: u32, length: u32);
}
// #[unsafe(no_mangle)]
// pub extern "C" fn _start() {
//     let mut big_test: u32 = 0;

//     let big_test: *mut u32 = &mut big_test;
//     unsafe {
//         __write__(big_test, 0xdeadbeef);
//         let _read_value = __read__(big_test);
//     }
// }

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn test(address: u32) -> u32 {
    let value = 0xdeadbeef;
    ext_println!(
        "Executing test function in env wrapper with address: {}",
        address
    );
    unsafe {
        __write__(address, value);
        let read_value = __read__(address);
        if read_value == value {
            0x1 // success
        } else {
            0x0 // failure
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn mul_f32_f64(a: f32, b: f64) -> f64 {
    (a as f64) * b
}

#[unsafe(no_mangle)]
pub extern "C" fn div_f64_f64(a: f64, b: f64) -> f64 {
    a / b
}

#[unsafe(no_mangle)]
pub extern "C" fn div_i64_i64(a: i64, b: i64) -> i64 {
    a / b
}

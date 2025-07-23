#![no_std]
#![no_main]
mod led;
use core::panic::PanicInfo;

unsafe extern "C" {
    pub fn __write__(address: u32, value: u32);
    pub fn __read__(address: u32) -> u32;
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

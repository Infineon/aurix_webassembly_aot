const BASE_ADDR_P00: u32 = 0xF003A000;

use super::*;
fn configure_pin() {
    // Configure P00.5 and P00.6
    let addr_iocr4 = BASE_ADDR_P00 + 0x14;
    unsafe {
        __write__(addr_iocr4, (128u32 << 8) | (128u32 << 16));
    }
}

fn read_button() -> u32 {
    // Read P00.7
    let addr_in = BASE_ADDR_P00 + 0x24;
    let val = unsafe { __read__(addr_in) };
    (val >> 7) & 0x1
}

fn write_led1(value: u32) {
    let addr_out = BASE_ADDR_P00;
    let old_reg = unsafe { __read__(addr_out) };
    let value_reg = ((value & 0x1u32) << 5) | (old_reg & !(0x1u32 << 5));
    unsafe {
        __write__(addr_out, value_reg);
    }
}

fn write_led2(value: u32) {
    let addr_out = BASE_ADDR_P00;
    let old_reg = unsafe { __read__(addr_out) };
    let value_reg = ((value & 0x1) << 6) | (old_reg & !(0x1 << 6));
    unsafe {
        __write__(addr_out, value_reg);
    }
}

#[no_mangle]
pub extern "C" fn _led1() -> ! {
    configure_pin();

    loop {
        let val = read_button();
        write_led1(val);
        write_led2(!val);
    }
}

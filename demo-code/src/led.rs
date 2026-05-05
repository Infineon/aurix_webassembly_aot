const BASE_ADDR_P00: u32 = 0xF003A000;
use super::*;
use utils::Timer;
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

const AUTOSAR: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_AUTOSAR);

#[no_mangle]
pub extern "C" fn _led1() -> ! {
    configure_pin();
    let mut timer = Timer::new();

    timer.read_timer();
    let result = AUTOSAR.checksum(b"123456789123456789");
    ext_println!(
        "Time to check CRC32 20 bytes: {} result={:x}",
        timer.get_delta(),
        result
    );
    loop {
        let val = read_button();
        if val == 0 {
            write_led1(1);
            write_led2(0);
        } else {
            ext_println!("Button not pressed val={}", timer.get_delta());
            write_led1(0);
            write_led2(1);
            timer.wait(25_000_000); // wait for 1 second (assuming 100MHz timer)
            write_led1(1);
            write_led2(0);
            timer.wait(25_000_000); // wait for 1 second (assuming 100MHz timer)
        }
    }
}

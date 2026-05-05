use tc37xpd::{stm0::Stm0, RegisterValue, STM0};
#[macro_export]
macro_rules! ext_println {
    () => {
        compile_error!(
            "ext_println! expects at least a format string literal, e.g. ext_println!(\"hello\")"
        );
    };
    ($fmt:literal) => {
        {      unsafe {
                    let ptr = $fmt.as_ptr() as u32;
                    let len = $fmt.len() as u32;
                    __write_str__(ptr, len);
                }

        };
    };
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        {   use heapless::String;
            use core::fmt::Write;
            // Format and print the message
            let mut s = String::<100>::new(); // Create a new heapless String with a capacity of 100 bytes
            // s.clear(); // Clear the string to ensure it's empty before writing
            // let s: &str = format_args!($fmt $(, $args)*).as_str().unwrap_or("Formatting error");
            if core::write!(&mut s, $fmt $(, $args)*).is_ok() {
                unsafe {
                    let ptr = s.as_ptr() as u32;
                    let len = s.len() as u32;
                    __write_str__(ptr, len);
                }
            } else {
                 // formatting failed (typically capacity overflow)
                let msg = "ext_println formatting error";
                unsafe {
                    $crate::__write_str__(msg.as_ptr() as u32, msg.len() as u32);
                }
            }
        };
    };
    ($($arg:tt)+) => {
        compile_error!(
            "ext_println! expects either no arguments or a format string literal followed by values, e.g. ext_println!(\"hello\") or ext_println!(\"x = {}\", x)"
        );
    };
}

pub struct Timer {
    timer: Stm0,
    last_value: u64,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            timer: STM0,
            last_value: 0,
        }
    }
    pub fn read_timer(&mut self) {
        unsafe {
            let val1 = self.timer.tim0().read().get_raw();
            let val2 = self.timer.cap().read().get_raw();

            self.last_value = (val2 as u64) << 32 | (val1 as u64);
        }
    }
    pub fn get_delta(&mut self) -> u64 {
        let previous_value = self.last_value;
        self.read_timer();
        self.last_value - previous_value
    }

    pub fn wait(&mut self, cycles: u64) {
        self.read_timer();
        let target = self.last_value + cycles;
        while self.last_value < target {
            self.read_timer();
        }
    }
}

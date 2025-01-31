//! # DESCRIPTION:
//!
//!	The module provides the TSIM support for TC162 Tricore.
//!
//!
//! FILE HISTORY:
//!
//!    V1  01.2023
//!
//**************************************************************************************************
// The source code is under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
//**************************************************************************************************
#![no_std]
#![no_main]

use crate::vio_write::write;
use core::fmt::{self, Write};
use tc162_rt::exit;
mod vio_syscalls;
mod vio_virtio;
pub mod vio_write;
pub mod _errno;


#[no_mangle]
pub extern "C" fn _exit() -> ! {
    tc162_rt::debug();
    loop{

    }

}

#[defmt::global_logger]
pub struct Logger;
use core::sync::atomic::{AtomicU32, Ordering};
static TAKEN: AtomicU32 = AtomicU32::new(0);
static mut ENCODER: defmt::Encoder = defmt::Encoder::new();
unsafe impl defmt::Logger for Logger {
    fn acquire() {
        if TAKEN
            .compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            panic!("defmt logger taken reentrantly")
        };
        unsafe { ENCODER.start_frame(hstdout_write_all) }
    }

    unsafe fn flush() {}

    unsafe fn release() {
        ENCODER.end_frame(hstdout_write_all);
        if TAKEN
            .compare_exchange(1, 0, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            panic!("defmt logger released reentrantly")
        };
    }

    unsafe fn write(bytes: &[u8]) {
        ENCODER.write(bytes, hstdout_write_all);
    }
}

#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    //defmt::println!("{:?}",info);
    defmt::error!("{}", defmt::Display2Format(info));
        loop {
            tc162_rt::exit(1)
        }
}

/// A byte stream to the host (e.g., host's stdout or stderr).
#[derive(Clone, Copy)]
pub struct HostStream {
    fd: u32,
}

impl HostStream {
    /// Attempts to write an entire `buffer` into this sink
    pub fn write_all(&mut self, buffer: &[u8]) -> Result<(), ()> {
        write(self.fd, buffer.as_ptr(), buffer.len() as u32);
        return Ok(());
    }
}

impl fmt::Write for HostStream {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes()).map_err(|_| fmt::Error)
    }
}

pub fn hstdout_str(s: &str) {
    let _result = unsafe {
        if HSTDOUT.is_none() {
            HSTDOUT = Some(HostStream { fd: 1 as u32 });
        }

        HSTDOUT.as_mut().unwrap().write_str(s).map_err(drop)
    };
}

pub fn hstdout_write_all(s: &[u8]) {
    let _result = unsafe {
        if HSTDOUT.is_none() {
            HSTDOUT = Some(HostStream { fd: 1 as u32 });
        }

        HSTDOUT.as_mut().unwrap().write_all(s).map_err(drop)
    };
}

pub fn hstdout_fmt(args: fmt::Arguments) {
    let _result = unsafe {
        if HSTDOUT.is_none() {
            HSTDOUT = Some(HostStream { fd: 1 as u32 });
        }

        HSTDOUT.as_mut().unwrap().write_fmt(args).map_err(drop)
    };
}

pub fn exit_prog(status: u32) -> ! {
    exit(status);
}

#[used]
static mut HSTDOUT: Option<HostStream> = None;
#[macro_export]
macro_rules! println {
    () => {
        $crate::hstdout_str("\n");
    };
    ($s:expr) => {
        $crate::hstdout_str(concat!($s,"\n"));
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::hstdout_fmt(format_args!(concat!($s,"\n"), $($tt)*));
    };
}

#[macro_export]
macro_rules! assert {
    ($cond:expr) => {
        if !$cond {
            println!("assertion failed: {}", stringify!($cond));
            panic!();
        }
    };
    ($cond:expr, $($arg:tt)*) => {
        if !$cond {
            println!("assertion failed: {}", format_args!($($arg)*));
            panic!();
        }
    };
}

#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr $(, $rest:expr)* $(,)?) => {
        if $left != $right {
            println!("Assertion failed: ({} == {})", stringify!($left), stringify!($right));
            panic!();
        }
        $(
            if $left != $rest {
                println!("Assertion failed: ({} == {})", stringify!($left), stringify!($rest));
                panic!();
            }
        )*
    }
}

#[macro_export]
macro_rules! dbg {
    ($val:expr) => {{
        println!("{} = {:?}", stringify!($val), $val);
        $val
    }};
}


// Note: without this, the watchdog will reset the CPU
#[export_name = "Crt0PreInit"]
fn pre_init_fn() {
    use tc37x_hal::wdtcon::{*};
    let cpu_core_id: u32;
    unsafe {
        core::arch::asm!("mfcr {0}, 0xFE1C", out(reg32) cpu_core_id);
    }
    if cpu_core_id == 0 {
        disable_safety_watchdog();
    }
    disable_cpu_watchdog();
}

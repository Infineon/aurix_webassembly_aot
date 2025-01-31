#![no_std]
#![no_main]
#[allow(unused_imports)]
pub use defmt_rtt as _;
use tc162_rt;
use tc37x_hal::wdtcon::{disable_cpu_watchdog,disable_safety_watchdog};
#[allow(unused_variables)]
#[panic_handler]
fn panic(panic: &core::panic::PanicInfo<'_>) -> ! {
    defmt::error!("Panic! {}", defmt::Display2Format(panic));
    tc162_rt::exit(1)
    
}


// Note: without this, the watchdog will reset the CPU
#[export_name = "Crt0PreInit"]
fn pre_init_fn() {
    let cpu_core_id: u32;
    unsafe {
        core::arch::asm!("mfcr {0}, 0xFE1C", out(reg32) cpu_core_id);
    }
    if cpu_core_id == 0 {
        disable_safety_watchdog();
    }
    disable_cpu_watchdog();
}

pub fn exit_prog(status: u32) -> ! {
    tc162_rt::exit(status)
}
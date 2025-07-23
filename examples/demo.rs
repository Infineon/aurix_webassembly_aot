#![no_std]
#![no_main]
#![feature(stdsimd)]

extern crate alloc;

use alloc::vec;
use aot_wasm::isa_model::{Immediate, ValueSize};
use defmt::println;
#[cfg(feature = "board")]
use probe_semihosting::exit_prog;
use tc162_rt::entry;
#[cfg(feature = "tsim")]
use tsim_semihosting::exit_prog;

use aot_wasm::parse_and_translate::{GlobalSpace, LinearMemory, WasmRuntime};
use core::ptr;
use embedded_alloc::LlffHeap as Heap;

use tc162_rt as _;

#[global_allocator]
static HEAP: Heap = Heap::empty();

// When linker option to configure the page size will be available we will be able to reduce
// the size of the linear memory. For now, we use a page size of 64KiB.
const MAX_MEMORY_SIZE: usize = (1 << 16) + 7;

#[link_section = ".CPU0.ramcode"]
static mut INSTRUCTIONS: [u32; 4096] = [0; 4096];
#[link_section = ".CPU0.data"]
static mut LINEAR_MEMORY: LinearMemory<MAX_MEMORY_SIZE> = LinearMemory::new();
#[link_section = ".CPU0.data"]
static mut GLOBAL_SPACE: GlobalSpace<256> = GlobalSpace::new();
#[link_section = ".CPU0.data"]
static mut TABLE: [u32; 256] = [0; 256];

fn exception_handler(status: u32) -> ! {
    let b = status.to_le_bytes();
    defmt::println!("Exception class {=u8}, TIN {=u8}", b[1], b[0]);
    exit_prog(status)
}

entry!(main);

fn main() {
    let mut runtime;
    unsafe {
        use core::mem::MaybeUninit;

        use defmt as _;
        use tc162_rt::set_exception_handler;

        const HEAP_SIZE: usize = 10 * 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

        HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE);
        defmt::println!("End init");
        set_exception_handler(exception_handler);

        runtime = WasmRuntime::new(
            &mut *ptr::addr_of_mut!(INSTRUCTIONS),
            &mut *ptr::addr_of_mut!(LINEAR_MEMORY),
            &mut *ptr::addr_of_mut!(GLOBAL_SPACE),
            &mut *ptr::addr_of_mut!(TABLE),
        );
    }
    runtime
        .parse_and_translate(include_bytes!(concat!(
            "../demo-code/target/wasm32v1-none/debug/",
            "demo_code.wasm"
        )))
        .unwrap();

    // let result = runtime.call_exported_function("_led1", vec![], Some(ValueSize::Word));
    let mut value: u32 = 0;
    let ptr: *mut u32 = &mut value;

    let result = runtime.call_exported_function(
        "test",
        vec![aot_wasm::isa_model::Immediate::Word(ptr as u32)],
        Some(ValueSize::Word),
    );
    assert_eq!(result, Some(Immediate::Word(1)));
    assert_eq!(unsafe { ptr.read_volatile() }, 0xdead_beef);
    println!("Successfully called test function");

    // This will never exit
    #[cfg(feature = "board")]
    {
    println!("Start LED demo");
    println!("Press button to toggle LED1 & LED2");
    println!("This demo will never exit, you can stop it with Ctrl+C");
    let _result = runtime.call_exported_function("_led1", vec![], Some(ValueSize::Word));
    }

    exit_prog(0);
}

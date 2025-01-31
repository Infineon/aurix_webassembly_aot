extern crate alloc;
use embedded_alloc::LlffHeap as Heap;
use aot_wasm::parse_and_translate::WasmRuntime;
// use aot_wasm::handle_misaligned_load_store;
use tc162_rt as _;
#[cfg(feature="tsim")]
use tsim_semihosting::exit_prog;
#[cfg(feature="board")]
use probe_semihosting::exit_prog;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[allow(unused_imports)]
use alloc::vec;

const MAX_MEMORY_SIZE: u32 = 1 << 16;

#[link_section = ".CPU0.ramcode"]
static mut INSTRUCTIONS: [u32; 4096] = [0; 4096];
#[link_section = ".CPU0.data"]
static mut LINEAR_MEMORY: [u8; MAX_MEMORY_SIZE as usize] = [0; MAX_MEMORY_SIZE as usize];
#[link_section = ".CPU0.data"]
static mut GLOBAL_SPACE: [u8; 256] = [0; 256];
#[link_section = ".CPU0.data"]
static mut TABLE: [u32; 256] = [0; 256];


fn exception_handler(status: u32) -> ! {
    let b = status.to_le_bytes();
    defmt::println!("Exception class {=u8}, TIN {=u8}", b[1], b[0]);
    exit_prog(status)
}

pub fn init() -> WasmRuntime<'static> {
    unsafe {
        use core::mem::MaybeUninit;

        use tc162_rt::set_exception_handler;
        use defmt as _;

        const HEAP_SIZE: usize = 10 * 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

        HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE);
        defmt::println!("End init");
        set_exception_handler(exception_handler);


        let runtime = WasmRuntime::new(
            &mut INSTRUCTIONS,
            &mut LINEAR_MEMORY,
            &mut GLOBAL_SPACE,
            &mut TABLE,
        );
        runtime
    }
}
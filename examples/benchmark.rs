#![no_std]
#![no_main]
#![feature(stdsimd)]
extern crate alloc;

mod utilities;

use alloc::vec;
use aot_wasm::isa_model::{Immediate, ValueSize};
use defmt::println;
#[cfg(feature = "board")]
use probe_semihosting::exit_prog;
use tc162_rt::entry;
#[cfg(feature = "tsim")]
use tsim_semihosting::exit_prog;
use utilities::*;

entry!(main);


fn main() {
    let mut timer = Timer::new();
    let mut runtime = init();

    macro_rules! run_benchmark {
    ($wasm_file:expr, $expected_result:expr) => {{
        runtime
            .parse_and_translate(include_bytes!(concat!("../benchmark-wasm/", $wasm_file)))
            .unwrap();
        runtime.call_exported_function("init", vec![], None);
        timer.read_timer();
        runtime.call_exported_function("kernel", vec![], None);
        let delta = timer.get_delta();
        let result = runtime.call_exported_function("result", vec![], Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord($expected_result)));
        println!("{=str} Delta: {=u64} Code Size:{=i32}", stringify!($wasm_file), delta,runtime.get_function_size("kernel"));
    }};
}
    
    run_benchmark!("correlation.wasm",782);
    run_benchmark!("floyd-warshall.wasm", 6594);
    run_benchmark!("jacobi-1d.wasm", 2);
    run_benchmark!("nussinov.wasm", 16254);
    exit_prog(0);
}

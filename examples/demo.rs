#![no_std]
#![no_main]
#![feature(stdsimd)]
extern crate alloc;

mod utilities;

use alloc::vec;
use aot_wasm::isa_model::{Immediate, ValueSize};
#[cfg(feature = "board")]
use probe_semihosting::exit_prog;
use tc162_rt::entry;
#[cfg(feature = "tsim")]
use tsim_semihosting::exit_prog;
use utilities::*;

entry!(main);


fn main() {
    let mut runtime = init();

    runtime.parse_and_translate(include_bytes!(concat!("../demo-code/", "demo.wasm")))
    .unwrap();

    let result = runtime.call_exported_function("test", vec![], Some(ValueSize::Word));
    assert_eq!(result, Some(Immediate::Word(0)));
    exit_prog(0);
}

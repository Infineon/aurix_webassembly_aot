
#![no_std]
#![no_main]

extern crate alloc;
mod test_utilities;
#[cfg(test)]
#[defmt_test::tests]
mod tests {
    #[allow(unused_imports)]
    use aot_wasm::isa_model::{Immediate,ValueSize};
    use aot_wasm::parse_and_translate::WasmRuntime;
    use defmt as _;

    #[allow(unused_imports)]
    use alloc::vec;

    #[init]
    fn init() -> WasmRuntime<'static> {
            use crate::test_utilities;
            let mut runtime = test_utilities::init();

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.8.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 136
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_div_one", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 137
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_one", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    
}


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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.91.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 2443
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(1065353216 as u32),Immediate::Word(1084227584 as u32)];
        let result = runtime.call_exported_function("f32.golden_ratio", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1070537661 as u32)));
    }
    

    // Command line number: 2444
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(4617315517961601024 as u64)];
        let result = runtime.call_exported_function("f64.golden_ratio", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4609965796441453736 as u64)));
    }
    
}

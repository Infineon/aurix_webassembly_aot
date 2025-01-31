
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.88.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 2389
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4719355144821538816 as u64),Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(4725141118604279808 as u64),Immediate::DoubleWord(4720637518976909312 as u64),Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(13938223582048944128 as u64)];
        let result = runtime.call_exported_function("dot_product_example", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4611686018427387904 as u64)));
    }
    

    // Command line number: 2393
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4719355144821538816 as u64),Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(4725141118604279808 as u64),Immediate::DoubleWord(4720637518976909312 as u64),Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(13938223582048944128 as u64)];
        let result = runtime.call_exported_function("with_binary_sum_collapse", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4611686018427387904 as u64)));
    }
    
}

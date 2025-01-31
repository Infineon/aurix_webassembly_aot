
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.92.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 2462
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("f32.silver_means", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2463
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("f32.silver_means", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1070537661 as u32)));
    }
    

    // Command line number: 2464
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1073741824 as u32)];
        let result = runtime.call_exported_function("f32.silver_means", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1075479162 as u32)));
    }
    

    // Command line number: 2465
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1077936128 as u32)];
        let result = runtime.call_exported_function("f32.silver_means", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1079206061 as u32)));
    }
    

    // Command line number: 2466
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1082130432 as u32)];
        let result = runtime.call_exported_function("f32.silver_means", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1082625502 as u32)));
    }
    

    // Command line number: 2467
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1084227584 as u32)];
        let result = runtime.call_exported_function("f32.silver_means", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1084631458 as u32)));
    }
    

    // Command line number: 2468
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("f64.silver_means", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 2469
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("f64.silver_means", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4609965796441453736 as u64)));
    }
    

    // Command line number: 2470
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4611686018427387904 as u64)];
        let result = runtime.call_exported_function("f64.silver_means", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4612618744449965542 as u64)));
    }
    

    // Command line number: 2471
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4613937818241073152 as u64)];
        let result = runtime.call_exported_function("f64.silver_means", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4614619608365706490 as u64)));
    }
    

    // Command line number: 2472
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4616189618054758400 as u64)];
        let result = runtime.call_exported_function("f64.silver_means", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4616455406968633940 as u64)));
    }
    

    // Command line number: 2473
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4617315517961601024 as u64)];
        let result = runtime.call_exported_function("f64.silver_means", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4617532346471836922 as u64)));
    }
    
}

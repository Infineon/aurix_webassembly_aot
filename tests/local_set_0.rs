
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "local_set.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 107
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-local-i32", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 108
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-local-i64", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 109
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-local-f32", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 110
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-local-f64", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 112
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("type-param-i32", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 113
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(3 as u64)];
        let result = runtime.call_exported_function("type-param-i64", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 114
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1082969293 as u32)];
        let result = runtime.call_exported_function("type-param-f32", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 115
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4617878467915022336 as u64)];
        let result = runtime.call_exported_function("type-param-f64", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 117
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-block-value", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 118
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-loop-value", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 120
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-br-value", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 121
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-br_if-value", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 122
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-br_if-value-cond", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 123
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-br_table-value", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 125
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-return-value", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 127
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-if-then", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 128
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-if-else", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 131
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::Word(1074580685 as u32),Immediate::DoubleWord(4614613358185178726 as u64),Immediate::Word(4 as u32),Immediate::Word(5 as u32)];
        let result = runtime.call_exported_function("type-mixed", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 137
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::Word(1073741824 as u32),Immediate::DoubleWord(4614613358185178726 as u64),Immediate::Word(4 as u32),Immediate::Word(5 as u32)];
        let result = runtime.call_exported_function("write", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(56 as u64)));
    }
    
}


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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.29.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 505
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(16777216 as u32)];
        let result = runtime.call_exported_function("i32.no_fold_f32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(16777216 as u32)));
    }
    

    // Command line number: 506
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(16777217 as u32)];
        let result = runtime.call_exported_function("i32.no_fold_f32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(16777216 as u32)));
    }
    

    // Command line number: 507
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4026531856 as u32)];
        let result = runtime.call_exported_function("i32.no_fold_f32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4026531856 as u32)));
    }
    

    // Command line number: 509
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(16777216 as u32)];
        let result = runtime.call_exported_function("i32.no_fold_f32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(16777216 as u32)));
    }
    

    // Command line number: 510
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(16777217 as u32)];
        let result = runtime.call_exported_function("i32.no_fold_f32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(16777216 as u32)));
    }
    

    // Command line number: 511
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4026531856 as u32)];
        let result = runtime.call_exported_function("i32.no_fold_f32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4026531840 as u32)));
    }
    

    // Command line number: 513
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9007199254740992 as u64)];
        let result = runtime.call_exported_function("i64.no_fold_f64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9007199254740992 as u64)));
    }
    

    // Command line number: 514
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9007199254740993 as u64)];
        let result = runtime.call_exported_function("i64.no_fold_f64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9007199254740992 as u64)));
    }
    

    // Command line number: 515
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(17293822569102705664 as u64)];
        let result = runtime.call_exported_function("i64.no_fold_f64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(17293822569102705664 as u64)));
    }
    

    // Command line number: 517
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9007199254740992 as u64)];
        let result = runtime.call_exported_function("i64.no_fold_f64_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9007199254740992 as u64)));
    }
    

    // Command line number: 518
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9007199254740993 as u64)];
        let result = runtime.call_exported_function("i64.no_fold_f64_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9007199254740992 as u64)));
    }
    

    // Command line number: 519
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(17293822569102705664 as u64)];
        let result = runtime.call_exported_function("i64.no_fold_f64_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(17293822569102704640 as u64)));
    }
    
}

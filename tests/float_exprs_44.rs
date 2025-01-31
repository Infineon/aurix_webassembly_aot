
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.44.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 973
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_lt_if", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2143289344 as u32)));
    }
    

    // Command line number: 974
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_lt_if", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 975
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_lt_if", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 976
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_lt_if", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 977
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_le_if", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2143289344 as u32)));
    }
    

    // Command line number: 978
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_le_if", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 979
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_le_if", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 980
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_le_if", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 981
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_gt_if", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2143289344 as u32)));
    }
    

    // Command line number: 982
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_gt_if", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 983
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_gt_if", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 984
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_gt_if", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 985
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_ge_if", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2143289344 as u32)));
    }
    

    // Command line number: 986
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_ge_if", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 987
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_ge_if", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 988
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_ge_if", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 989
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_lt_if", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9221120237041090560 as u64)));
    }
    

    // Command line number: 990
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_lt_if", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 991
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_lt_if", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 992
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_lt_if", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 993
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_le_if", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9221120237041090560 as u64)));
    }
    

    // Command line number: 994
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_le_if", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 995
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_le_if", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 996
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_le_if", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 997
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_gt_if", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9221120237041090560 as u64)));
    }
    

    // Command line number: 998
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_gt_if", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 999
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_gt_if", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1000
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_gt_if", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1001
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_ge_if", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9221120237041090560 as u64)));
    }
    

    // Command line number: 1002
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_ge_if", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1003
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_ge_if", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1004
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_ge_if", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    
}

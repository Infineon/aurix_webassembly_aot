
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.46.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 1099
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_lt_if_to_abs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2141192192 as u32)));
    }
    

    // Command line number: 1100
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_lt_if_to_abs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4290772992 as u32)));
    }
    

    // Command line number: 1101
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_lt_if_to_abs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1102
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_lt_if_to_abs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1103
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_le_if_to_abs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2141192192 as u32)));
    }
    

    // Command line number: 1104
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_le_if_to_abs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4290772992 as u32)));
    }
    

    // Command line number: 1105
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_le_if_to_abs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1106
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_le_if_to_abs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1107
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_gt_if_to_abs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4288675840 as u32)));
    }
    

    // Command line number: 1108
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_gt_if_to_abs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2143289344 as u32)));
    }
    

    // Command line number: 1109
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_gt_if_to_abs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1110
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_gt_if_to_abs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1111
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_ge_if_to_abs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4288675840 as u32)));
    }
    

    // Command line number: 1112
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_ge_if_to_abs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2143289344 as u32)));
    }
    

    // Command line number: 1113
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_ge_if_to_abs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1114
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_ge_if_to_abs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1115
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_lt_if_to_abs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9219994337134247936 as u64)));
    }
    

    // Command line number: 1116
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_lt_if_to_abs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18444492273895866368 as u64)));
    }
    

    // Command line number: 1117
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_lt_if_to_abs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1118
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_lt_if_to_abs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1119
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_le_if_to_abs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9219994337134247936 as u64)));
    }
    

    // Command line number: 1120
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_le_if_to_abs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18444492273895866368 as u64)));
    }
    

    // Command line number: 1121
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_le_if_to_abs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1122
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_le_if_to_abs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1123
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_gt_if_to_abs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18443366373989023744 as u64)));
    }
    

    // Command line number: 1124
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_gt_if_to_abs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9221120237041090560 as u64)));
    }
    

    // Command line number: 1125
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_gt_if_to_abs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1126
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_gt_if_to_abs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1127
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_ge_if_to_abs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18443366373989023744 as u64)));
    }
    

    // Command line number: 1128
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_ge_if_to_abs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9221120237041090560 as u64)));
    }
    

    // Command line number: 1129
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_ge_if_to_abs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1130
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_ge_if_to_abs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    
}

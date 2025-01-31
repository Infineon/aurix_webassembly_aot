
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.39.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 723
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1069547520 as u32)];
        let result = runtime.call_exported_function("f32.i32.no_fold_trunc_s_convert_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 724
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3217031168 as u32)];
        let result = runtime.call_exported_function("f32.i32.no_fold_trunc_s_convert_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 725
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1069547520 as u32)];
        let result = runtime.call_exported_function("f32.i32.no_fold_trunc_u_convert_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 726
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("f32.i32.no_fold_trunc_u_convert_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 727
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1069547520 as u32)];
        let result = runtime.call_exported_function("f32.i32.no_fold_trunc_s_convert_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 728
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3217031168 as u32)];
        let result = runtime.call_exported_function("f32.i32.no_fold_trunc_s_convert_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1333788672 as u32)));
    }
    

    // Command line number: 729
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1069547520 as u32)];
        let result = runtime.call_exported_function("f32.i32.no_fold_trunc_u_convert_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 730
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("f32.i32.no_fold_trunc_u_convert_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 732
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4609434218613702656 as u64)];
        let result = runtime.call_exported_function("f64.i32.no_fold_trunc_s_convert_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 733
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13832806255468478464 as u64)];
        let result = runtime.call_exported_function("f64.i32.no_fold_trunc_s_convert_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13830554455654793216 as u64)));
    }
    

    // Command line number: 734
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4609434218613702656 as u64)];
        let result = runtime.call_exported_function("f64.i32.no_fold_trunc_u_convert_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 735
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("f64.i32.no_fold_trunc_u_convert_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 736
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4609434218613702656 as u64)];
        let result = runtime.call_exported_function("f64.i32.no_fold_trunc_s_convert_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 737
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13832806255468478464 as u64)];
        let result = runtime.call_exported_function("f64.i32.no_fold_trunc_s_convert_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4751297606873776128 as u64)));
    }
    

    // Command line number: 738
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4609434218613702656 as u64)];
        let result = runtime.call_exported_function("f64.i32.no_fold_trunc_u_convert_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 739
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("f64.i32.no_fold_trunc_u_convert_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 741
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1069547520 as u32)];
        let result = runtime.call_exported_function("f32.i64.no_fold_trunc_s_convert_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 742
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3217031168 as u32)];
        let result = runtime.call_exported_function("f32.i64.no_fold_trunc_s_convert_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 743
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1069547520 as u32)];
        let result = runtime.call_exported_function("f32.i64.no_fold_trunc_u_convert_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 744
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("f32.i64.no_fold_trunc_u_convert_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 745
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1069547520 as u32)];
        let result = runtime.call_exported_function("f32.i64.no_fold_trunc_s_convert_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 746
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3217031168 as u32)];
        let result = runtime.call_exported_function("f32.i64.no_fold_trunc_s_convert_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1602224128 as u32)));
    }
    

    // Command line number: 747
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1069547520 as u32)];
        let result = runtime.call_exported_function("f32.i64.no_fold_trunc_u_convert_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 748
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("f32.i64.no_fold_trunc_u_convert_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 750
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4609434218613702656 as u64)];
        let result = runtime.call_exported_function("f64.i64.no_fold_trunc_s_convert_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 751
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13832806255468478464 as u64)];
        let result = runtime.call_exported_function("f64.i64.no_fold_trunc_s_convert_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13830554455654793216 as u64)));
    }
    

    // Command line number: 752
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4609434218613702656 as u64)];
        let result = runtime.call_exported_function("f64.i64.no_fold_trunc_u_convert_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 753
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("f64.i64.no_fold_trunc_u_convert_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 754
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4609434218613702656 as u64)];
        let result = runtime.call_exported_function("f64.i64.no_fold_trunc_s_convert_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 755
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13832806255468478464 as u64)];
        let result = runtime.call_exported_function("f64.i64.no_fold_trunc_s_convert_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4895412794951729152 as u64)));
    }
    

    // Command line number: 756
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4609434218613702656 as u64)];
        let result = runtime.call_exported_function("f64.i64.no_fold_trunc_u_convert_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 757
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("f64.i64.no_fold_trunc_u_convert_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    
}

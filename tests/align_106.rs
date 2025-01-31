
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "align.106.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 802
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("f32_align_switch", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1092616192 as u32)));
    }
    

    // Command line number: 803
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("f32_align_switch", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1092616192 as u32)));
    }
    

    // Command line number: 804
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("f32_align_switch", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1092616192 as u32)));
    }
    

    // Command line number: 805
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("f32_align_switch", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1092616192 as u32)));
    }
    

    // Command line number: 807
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("f64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4621819117588971520 as u64)));
    }
    

    // Command line number: 808
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("f64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4621819117588971520 as u64)));
    }
    

    // Command line number: 809
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("f64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4621819117588971520 as u64)));
    }
    

    // Command line number: 810
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("f64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4621819117588971520 as u64)));
    }
    

    // Command line number: 811
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32)];
        let result = runtime.call_exported_function("f64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4621819117588971520 as u64)));
    }
    

    // Command line number: 813
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("i32_align_switch", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(10 as u32)));
    }
    

    // Command line number: 814
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("i32_align_switch", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(10 as u32)));
    }
    

    // Command line number: 815
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("i32_align_switch", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(10 as u32)));
    }
    

    // Command line number: 816
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("i32_align_switch", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(10 as u32)));
    }
    

    // Command line number: 817
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("i32_align_switch", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(10 as u32)));
    }
    

    // Command line number: 818
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("i32_align_switch", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(10 as u32)));
    }
    

    // Command line number: 819
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("i32_align_switch", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(10 as u32)));
    }
    

    // Command line number: 820
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("i32_align_switch", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(10 as u32)));
    }
    

    // Command line number: 821
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("i32_align_switch", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(10 as u32)));
    }
    

    // Command line number: 822
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("i32_align_switch", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(10 as u32)));
    }
    

    // Command line number: 823
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("i32_align_switch", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(10 as u32)));
    }
    

    // Command line number: 824
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("i32_align_switch", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(10 as u32)));
    }
    

    // Command line number: 825
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("i32_align_switch", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(10 as u32)));
    }
    

    // Command line number: 826
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32),Immediate::Word(4 as u32)];
        let result = runtime.call_exported_function("i32_align_switch", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(10 as u32)));
    }
    

    // Command line number: 828
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 829
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 830
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 831
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 832
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 833
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 834
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 835
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 836
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 837
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 838
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 839
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 840
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 841
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32),Immediate::Word(4 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 842
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 843
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 844
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 845
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32),Immediate::Word(4 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 846
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(6 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 847
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(6 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 848
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(6 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 849
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(6 as u32),Immediate::Word(4 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    

    // Command line number: 850
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(6 as u32),Immediate::Word(8 as u32)];
        let result = runtime.call_exported_function("i64_align_switch", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10 as u64)));
    }
    
}

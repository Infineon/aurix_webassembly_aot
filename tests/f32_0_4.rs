
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "f32.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 899
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 900
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 901
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 902
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 903
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 904
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 905
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 906
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 907
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 908
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 909
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 910
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 916
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 917
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 918
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 919
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(29954011 as u32)));
    }
    

    // Command line number: 920
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2177437659 as u32)));
    }
    

    // Command line number: 921
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2177437659 as u32)));
    }
    

    // Command line number: 922
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(29954011 as u32)));
    }
    

    // Command line number: 923
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1082130431 as u32)));
    }
    

    // Command line number: 924
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3229614079 as u32)));
    }
    

    // Command line number: 925
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3229614079 as u32)));
    }
    

    // Command line number: 926
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1082130431 as u32)));
    }
    

    // Command line number: 927
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 928
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 929
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 930
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 931
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 932
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 933
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 934
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 935
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 936
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 937
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 938
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 939
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 940
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 941
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 942
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 943
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 944
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 945
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 946
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 951
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1048576000 as u32)));
    }
    

    // Command line number: 952
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3196059648 as u32)));
    }
    

    // Command line number: 953
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3196059648 as u32)));
    }
    

    // Command line number: 954
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1048576000 as u32)));
    }
    

    // Command line number: 955
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 956
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 957
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 958
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 959
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1078530011 as u32)));
    }
    

    // Command line number: 960
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3226013659 as u32)));
    }
    

    // Command line number: 961
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3226013659 as u32)));
    }
    

    // Command line number: 962
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1078530011 as u32)));
    }
    

    // Command line number: 963
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2130706431 as u32)));
    }
    

    // Command line number: 964
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4278190079 as u32)));
    }
    

    // Command line number: 965
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4278190079 as u32)));
    }
    

    // Command line number: 966
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2130706431 as u32)));
    }
    

    // Command line number: 967
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 968
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 969
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 970
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 971
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 972
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 973
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 974
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 975
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 976
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 977
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 978
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 979
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 980
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 981
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 982
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 987
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 988
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 989
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 990
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 991
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 992
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 993
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 994
    #[test]
    fn test_82(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 995
    #[test]
    fn test_83(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 996
    #[test]
    fn test_84(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 997
    #[test]
    fn test_85(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 998
    #[test]
    fn test_86(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 999
    #[test]
    fn test_87(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 1000
    #[test]
    fn test_88(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3234402267 as u32)));
    }
    

    // Command line number: 1001
    #[test]
    fn test_89(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3234402267 as u32)));
    }
    

    // Command line number: 1002
    #[test]
    fn test_90(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 1003
    #[test]
    fn test_91(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 1004
    #[test]
    fn test_92(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    

    // Command line number: 1005
    #[test]
    fn test_93(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    

    // Command line number: 1006
    #[test]
    fn test_94(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 1007
    #[test]
    fn test_95(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1008
    #[test]
    fn test_96(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1009
    #[test]
    fn test_97(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1010
    #[test]
    fn test_98(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1011
    #[test]
    fn test_99(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1012
    #[test]
    fn test_100(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1013
    #[test]
    fn test_101(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1014
    #[test]
    fn test_102(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1015
    #[test]
    fn test_103(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1016
    #[test]
    fn test_104(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1017
    #[test]
    fn test_105(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1018
    #[test]
    fn test_106(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1019
    #[test]
    fn test_107(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1020
    #[test]
    fn test_108(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1021
    #[test]
    fn test_109(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1022
    #[test]
    fn test_110(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1027
    #[test]
    fn test_111(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(29954011 as u32)));
    }
    

    // Command line number: 1028
    #[test]
    fn test_112(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2177437659 as u32)));
    }
    

    // Command line number: 1029
    #[test]
    fn test_113(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2177437659 as u32)));
    }
    

    // Command line number: 1030
    #[test]
    fn test_114(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(29954011 as u32)));
    }
    

    // Command line number: 1031
    #[test]
    fn test_115(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1078530011 as u32)));
    }
    

    // Command line number: 1032
    #[test]
    fn test_116(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3226013659 as u32)));
    }
    

    // Command line number: 1033
    #[test]
    fn test_117(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3226013659 as u32)));
    }
    

    // Command line number: 1034
    #[test]
    fn test_118(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1078530011 as u32)));
    }
    

    // Command line number: 1035
    #[test]
    fn test_119(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 1036
    #[test]
    fn test_120(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3234402267 as u32)));
    }
    

    // Command line number: 1037
    #[test]
    fn test_121(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3234402267 as u32)));
    }
    

    // Command line number: 1038
    #[test]
    fn test_122(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 1039
    #[test]
    fn test_123(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1109256679 as u32)));
    }
    

    // Command line number: 1040
    #[test]
    fn test_124(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3256740327 as u32)));
    }
    

    // Command line number: 1041
    #[test]
    fn test_125(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3256740327 as u32)));
    }
    

    // Command line number: 1042
    #[test]
    fn test_126(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1109256679 as u32)));
    }
    

    // Command line number: 1043
    #[test]
    fn test_127(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1044
    #[test]
    fn test_128(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1045
    #[test]
    fn test_129(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1046
    #[test]
    fn test_130(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1047
    #[test]
    fn test_131(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1048
    #[test]
    fn test_132(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1049
    #[test]
    fn test_133(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1050
    #[test]
    fn test_134(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1051
    #[test]
    fn test_135(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1052
    #[test]
    fn test_136(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1053
    #[test]
    fn test_137(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1054
    #[test]
    fn test_138(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1055
    #[test]
    fn test_139(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1056
    #[test]
    fn test_140(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1057
    #[test]
    fn test_141(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1058
    #[test]
    fn test_142(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1059
    #[test]
    fn test_143(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1060
    #[test]
    fn test_144(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1061
    #[test]
    fn test_145(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1062
    #[test]
    fn test_146(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1067
    #[test]
    fn test_147(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1082130431 as u32)));
    }
    

    // Command line number: 1068
    #[test]
    fn test_148(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3229614079 as u32)));
    }
    

    // Command line number: 1069
    #[test]
    fn test_149(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3229614079 as u32)));
    }
    

    // Command line number: 1070
    #[test]
    fn test_150(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1082130431 as u32)));
    }
    

    // Command line number: 1071
    #[test]
    fn test_151(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2130706431 as u32)));
    }
    

    // Command line number: 1072
    #[test]
    fn test_152(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4278190079 as u32)));
    }
    

    // Command line number: 1073
    #[test]
    fn test_153(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4278190079 as u32)));
    }
    

    // Command line number: 1074
    #[test]
    fn test_154(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2130706431 as u32)));
    }
    

    // Command line number: 1075
    #[test]
    fn test_155(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 1076
    #[test]
    fn test_156(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    

    // Command line number: 1077
    #[test]
    fn test_157(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    

    // Command line number: 1078
    #[test]
    fn test_158(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 1079
    #[test]
    fn test_159(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1080
    #[test]
    fn test_160(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1081
    #[test]
    fn test_161(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1082
    #[test]
    fn test_162(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1083
    #[test]
    fn test_163(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1084
    #[test]
    fn test_164(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1085
    #[test]
    fn test_165(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1086
    #[test]
    fn test_166(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1087
    #[test]
    fn test_167(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1088
    #[test]
    fn test_168(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1089
    #[test]
    fn test_169(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1090
    #[test]
    fn test_170(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1091
    #[test]
    fn test_171(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1092
    #[test]
    fn test_172(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1093
    #[test]
    fn test_173(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1094
    #[test]
    fn test_174(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1095
    #[test]
    fn test_175(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1096
    #[test]
    fn test_176(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1097
    #[test]
    fn test_177(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1098
    #[test]
    fn test_178(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1099
    #[test]
    fn test_179(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1100
    #[test]
    fn test_180(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1101
    #[test]
    fn test_181(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1102
    #[test]
    fn test_182(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1107
    #[test]
    fn test_183(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1108
    #[test]
    fn test_184(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1109
    #[test]
    fn test_185(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1110
    #[test]
    fn test_186(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1111
    #[test]
    fn test_187(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1112
    #[test]
    fn test_188(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1113
    #[test]
    fn test_189(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1114
    #[test]
    fn test_190(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1115
    #[test]
    fn test_191(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1116
    #[test]
    fn test_192(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1117
    #[test]
    fn test_193(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1118
    #[test]
    fn test_194(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1119
    #[test]
    fn test_195(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1120
    #[test]
    fn test_196(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1121
    #[test]
    fn test_197(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1122
    #[test]
    fn test_198(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1123
    #[test]
    fn test_199(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    
}

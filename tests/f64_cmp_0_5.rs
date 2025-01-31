
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "f64_cmp.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 1013
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1014
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1015
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1016
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1017
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1018
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1019
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1020
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1021
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1022
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1023
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1024
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1025
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1026
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1027
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1028
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1029
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1030
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1031
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1032
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1033
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1034
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1035
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1036
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1037
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1038
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1039
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1040
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1041
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1042
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1043
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1044
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1045
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1046
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1047
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1048
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1049
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1050
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1051
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1052
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1053
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1054
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1055
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1056
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1057
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1058
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1059
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1060
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1061
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1062
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1063
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1064
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1065
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1066
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1067
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1068
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1069
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1070
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1071
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1072
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1073
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1074
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1075
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1076
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1077
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1078
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1079
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1080
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1081
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1082
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1083
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1084
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1085
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1086
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1087
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1088
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1089
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1090
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1091
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1092
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1093
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1094
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1095
    #[test]
    fn test_82(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1096
    #[test]
    fn test_83(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1097
    #[test]
    fn test_84(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1098
    #[test]
    fn test_85(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1099
    #[test]
    fn test_86(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1100
    #[test]
    fn test_87(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1101
    #[test]
    fn test_88(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1102
    #[test]
    fn test_89(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1103
    #[test]
    fn test_90(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1104
    #[test]
    fn test_91(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1105
    #[test]
    fn test_92(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1106
    #[test]
    fn test_93(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1107
    #[test]
    fn test_94(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1108
    #[test]
    fn test_95(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1109
    #[test]
    fn test_96(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1110
    #[test]
    fn test_97(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1111
    #[test]
    fn test_98(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1112
    #[test]
    fn test_99(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1113
    #[test]
    fn test_100(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1114
    #[test]
    fn test_101(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1115
    #[test]
    fn test_102(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1116
    #[test]
    fn test_103(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1117
    #[test]
    fn test_104(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1118
    #[test]
    fn test_105(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1119
    #[test]
    fn test_106(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1120
    #[test]
    fn test_107(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1121
    #[test]
    fn test_108(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1122
    #[test]
    fn test_109(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1123
    #[test]
    fn test_110(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1124
    #[test]
    fn test_111(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1125
    #[test]
    fn test_112(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1126
    #[test]
    fn test_113(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1127
    #[test]
    fn test_114(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1128
    #[test]
    fn test_115(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1129
    #[test]
    fn test_116(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1130
    #[test]
    fn test_117(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1131
    #[test]
    fn test_118(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1132
    #[test]
    fn test_119(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1133
    #[test]
    fn test_120(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1134
    #[test]
    fn test_121(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1135
    #[test]
    fn test_122(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1136
    #[test]
    fn test_123(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1137
    #[test]
    fn test_124(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1138
    #[test]
    fn test_125(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1139
    #[test]
    fn test_126(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1140
    #[test]
    fn test_127(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1141
    #[test]
    fn test_128(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1142
    #[test]
    fn test_129(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1143
    #[test]
    fn test_130(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1144
    #[test]
    fn test_131(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1145
    #[test]
    fn test_132(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1146
    #[test]
    fn test_133(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1147
    #[test]
    fn test_134(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1148
    #[test]
    fn test_135(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1149
    #[test]
    fn test_136(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1150
    #[test]
    fn test_137(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1151
    #[test]
    fn test_138(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1152
    #[test]
    fn test_139(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1153
    #[test]
    fn test_140(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1154
    #[test]
    fn test_141(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1155
    #[test]
    fn test_142(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1156
    #[test]
    fn test_143(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1157
    #[test]
    fn test_144(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1158
    #[test]
    fn test_145(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1159
    #[test]
    fn test_146(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1160
    #[test]
    fn test_147(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1161
    #[test]
    fn test_148(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1162
    #[test]
    fn test_149(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1163
    #[test]
    fn test_150(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1164
    #[test]
    fn test_151(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1165
    #[test]
    fn test_152(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1166
    #[test]
    fn test_153(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1167
    #[test]
    fn test_154(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1168
    #[test]
    fn test_155(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1169
    #[test]
    fn test_156(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1170
    #[test]
    fn test_157(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1171
    #[test]
    fn test_158(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1172
    #[test]
    fn test_159(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1173
    #[test]
    fn test_160(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1174
    #[test]
    fn test_161(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1175
    #[test]
    fn test_162(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1176
    #[test]
    fn test_163(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1177
    #[test]
    fn test_164(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1178
    #[test]
    fn test_165(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1179
    #[test]
    fn test_166(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1180
    #[test]
    fn test_167(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1181
    #[test]
    fn test_168(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1182
    #[test]
    fn test_169(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1183
    #[test]
    fn test_170(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1184
    #[test]
    fn test_171(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1185
    #[test]
    fn test_172(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1186
    #[test]
    fn test_173(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1187
    #[test]
    fn test_174(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1188
    #[test]
    fn test_175(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1189
    #[test]
    fn test_176(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1190
    #[test]
    fn test_177(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1191
    #[test]
    fn test_178(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1192
    #[test]
    fn test_179(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1193
    #[test]
    fn test_180(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1194
    #[test]
    fn test_181(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1195
    #[test]
    fn test_182(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1196
    #[test]
    fn test_183(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1197
    #[test]
    fn test_184(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1198
    #[test]
    fn test_185(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1199
    #[test]
    fn test_186(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1200
    #[test]
    fn test_187(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1201
    #[test]
    fn test_188(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1202
    #[test]
    fn test_189(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1203
    #[test]
    fn test_190(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1204
    #[test]
    fn test_191(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1205
    #[test]
    fn test_192(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1206
    #[test]
    fn test_193(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1207
    #[test]
    fn test_194(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1208
    #[test]
    fn test_195(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1209
    #[test]
    fn test_196(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1210
    #[test]
    fn test_197(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1211
    #[test]
    fn test_198(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1212
    #[test]
    fn test_199(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    
}

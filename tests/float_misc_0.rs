
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_misc.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 50
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1066388847 as u32),Immediate::Word(789036054 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1066388847 as u32)));
    }
    

    // Command line number: 51
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607738418748954166 as u64),Immediate::DoubleWord(4458835772027226175 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607738418749510135 as u64)));
    }
    

    // Command line number: 55
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(864026624 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 56
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(864026625 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353217 as u32)));
    }
    

    // Command line number: 57
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(4368491638549381120 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 58
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(4368491638549381121 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017409 as u64)));
    }
    

    // Command line number: 62
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(4503599627370495 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4503599627370496 as u64)));
    }
    

    // Command line number: 67
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1325400064 as u32),Immediate::Word(1149241344 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1325400068 as u32)));
    }
    

    // Command line number: 68
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4890909195324358656 as u64),Immediate::DoubleWord(4652219514585350144 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4890909195324358657 as u64)));
    }
    

    // Command line number: 72
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9290926031265333248 as u64),Immediate::DoubleWord(5910 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9290926031265333247 as u64)));
    }
    

    // Command line number: 75
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4845873199050653696 as u64),Immediate::DoubleWord(4607182463836013682 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4845873199050653697 as u64)));
    }
    

    // Command line number: 78
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4845873199050653697 as u64),Immediate::DoubleWord(4607182281361063936 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4845873199050653697 as u64)));
    }
    

    // Command line number: 81
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1258291200 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1258291200 as u32)));
    }
    

    // Command line number: 82
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1258291201 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1258291202 as u32)));
    }
    

    // Command line number: 83
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4841369599423283200 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4841369599423283200 as u64)));
    }
    

    // Command line number: 84
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4841369599423283201 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4841369599423283202 as u64)));
    }
    

    // Command line number: 87
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4070355885 as u32),Immediate::Word(238773414 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4070355885 as u32)));
    }
    

    // Command line number: 88
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1630930834 as u32),Immediate::Word(3650472296 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1630930534 as u32)));
    }
    

    // Command line number: 89
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(376639884 as u32),Immediate::Word(24880479 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(376639884 as u32)));
    }
    

    // Command line number: 90
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1393899754 as u32),Immediate::Word(3680827526 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3680827377 as u32)));
    }
    

    // Command line number: 91
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(947682203 as u32),Immediate::Word(1958603311 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1958603311 as u32)));
    }
    

    // Command line number: 92
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2751474943031650218 as u64),Immediate::DoubleWord(14953834855654151696 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14953834855654151696 as u64)));
    }
    

    // Command line number: 93
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14174076479053295281 as u64),Immediate::DoubleWord(3779173703388472492 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14174076479053295281 as u64)));
    }
    

    // Command line number: 94
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(3940735352412940081 as u64),Immediate::DoubleWord(13776826739676942972 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13776826739676942972 as u64)));
    }
    

    // Command line number: 95
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6845567588107709194 as u64),Immediate::DoubleWord(4904758653169279867 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(6845567588107709194 as u64)));
    }
    

    // Command line number: 96
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6057047893039655554 as u64),Immediate::DoubleWord(6381964069811498464 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(6381964069811498464 as u64)));
    }
    

    // Command line number: 99
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1670249659 as u32),Immediate::Word(3384781876 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1670249659 as u32)));
    }
    

    // Command line number: 100
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(692047414 as u32),Immediate::Word(2564611463 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(692047414 as u32)));
    }
    

    // Command line number: 101
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2862180574 as u32),Immediate::Word(2122049802 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2122049802 as u32)));
    }
    

    // Command line number: 102
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2848402951 as u32),Immediate::Word(2325576998 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2848402951 as u32)));
    }
    

    // Command line number: 103
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(982116028 as u32),Immediate::Word(2317187467 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(982116028 as u32)));
    }
    

    // Command line number: 104
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5417704807092288842 as u64),Immediate::DoubleWord(11458115339210975423 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5417704807092288842 as u64)));
    }
    

    // Command line number: 105
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5306888566943064716 as u64),Immediate::DoubleWord(13560253914302152139 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5306888566943064716 as u64)));
    }
    

    // Command line number: 106
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2791030278657170439 as u64),Immediate::DoubleWord(9423751710011603955 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2791030278657170439 as u64)));
    }
    

    // Command line number: 107
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14493512931660601393 as u64),Immediate::DoubleWord(14459238760182946131 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14493541008051035082 as u64)));
    }
    

    // Command line number: 108
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13847269089137787654 as u64),Immediate::DoubleWord(5913400236268010570 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5913400236268010570 as u64)));
    }
    

    // Command line number: 111
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2231979500 as u32),Immediate::Word(711174153 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(711174153 as u32)));
    }
    

    // Command line number: 112
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3084976721 as u32),Immediate::Word(843077785 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3084970566 as u32)));
    }
    

    // Command line number: 113
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3522127374 as u32),Immediate::Word(157500525 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3522127374 as u32)));
    }
    

    // Command line number: 114
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(78702389 as u32),Immediate::Word(3468399689 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3468399689 as u32)));
    }
    

    // Command line number: 115
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2208228371 as u32),Immediate::Word(1870536627 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1870536627 as u32)));
    }
    

    // Command line number: 116
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1111252809067936271 as u64),Immediate::DoubleWord(18353849863141451174 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18353849863141451174 as u64)));
    }
    

    // Command line number: 117
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(16156503866526998353 as u64),Immediate::DoubleWord(16051628718393451642 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(16156503867159194550 as u64)));
    }
    

    // Command line number: 118
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9478779231447112314 as u64),Immediate::DoubleWord(3336259491488022866 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3336259491488022866 as u64)));
    }
    

    // Command line number: 119
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(11541137186853127278 as u64),Immediate::DoubleWord(735421354967021004 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(11541137186853127278 as u64)));
    }
    

    // Command line number: 120
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18181384676953296798 as u64),Immediate::DoubleWord(8211214354507491487 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18181384676953296798 as u64)));
    }
    

    // Command line number: 123
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(17455847474217352892 as u64),Immediate::DoubleWord(8328504330151758329 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(8328504328029232689 as u64)));
    }
    

    // Command line number: 124
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(8138029755241725240 as u64),Immediate::DoubleWord(7967841770626914860 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(8138029755241745167 as u64)));
    }
    

    // Command line number: 125
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(16644741111433920314 as u64),Immediate::DoubleWord(7647005019700459394 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(7647005019700459387 as u64)));
    }
    

    // Command line number: 126
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6696851869641768443 as u64),Immediate::DoubleWord(6935821972358342665 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(6935821972358342665 as u64)));
    }
    

    // Command line number: 127
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(127567346952002978 as u64),Immediate::DoubleWord(273820416703444795 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(273820416704834831 as u64)));
    }
    

    // Command line number: 130
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(17221080860858566255 as u64),Immediate::DoubleWord(18119355194379769652 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18119355194379769652 as u64)));
    }
    

    // Command line number: 131
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5479469458864399218 as u64),Immediate::DoubleWord(15063363222154738502 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(15063363222154738502 as u64)));
    }
    

    // Command line number: 132
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(15191393426281101490 as u64),Immediate::DoubleWord(2705322087145917275 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(15191393426281101490 as u64)));
    }
    

    // Command line number: 133
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(12404517211283155052 as u64),Immediate::DoubleWord(15933096090325362723 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(15933096090325362723 as u64)));
    }
    

    // Command line number: 134
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(12941874353187635000 as u64),Immediate::DoubleWord(149235811938438489 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(12941874353187635000 as u64)));
    }
    

    // Command line number: 137
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2039859408 as u32),Immediate::Word(2137384617 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2137389410 as u32)));
    }
    

    // Command line number: 138
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2135652809 as u32),Immediate::Word(4113932278 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2135652798 as u32)));
    }
    

    // Command line number: 139
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2055837582 as u32),Immediate::Word(4281461529 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4281443917 as u32)));
    }
    

    // Command line number: 140
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4220448050 as u32),Immediate::Word(4286022000 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286095178 as u32)));
    }
    

    // Command line number: 141
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4280642669 as u32),Immediate::Word(4269109313 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4284663933 as u32)));
    }
    

    // Command line number: 142
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9214390167692528523 as u64),Immediate::DoubleWord(9025765862177526868 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9214390167692529673 as u64)));
    }
    

    // Command line number: 143
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218174330906182895 as u64),Immediate::DoubleWord(9066655639269665468 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218174330906813073 as u64)));
    }
    

    // Command line number: 144
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18440065048487386960 as u64),Immediate::DoubleWord(9166530475417861020 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18440062016830479601 as u64)));
    }
    

    // Command line number: 145
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18341378616977760125 as u64),Immediate::DoubleWord(9218376305633483958 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218376303911020838 as u64)));
    }
    

    // Command line number: 146
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9214484525044672441 as u64),Immediate::DoubleWord(18297568705700624757 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9214484525042704878 as u64)));
    }
    

    // Command line number: 154
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5760669582171681 as u64),Immediate::DoubleWord(9233067416817195210 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9227994927942701939 as u64)));
    }
    

    // Command line number: 155
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9301019945277718 as u64),Immediate::DoubleWord(9236877899492518055 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9231781722239704866 as u64)));
    }
    

    // Command line number: 156
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9225260466475370890 as u64),Immediate::DoubleWord(9229416573822848190 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9231305003443443272 as u64)));
    }
    

    // Command line number: 157
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(16303139584665809 as u64),Immediate::DoubleWord(9240639734747285230 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9227230270086150260 as u64)));
    }
    

    // Command line number: 158
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9232116546764732939 as u64),Immediate::DoubleWord(5103597037406761 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9227012949727326178 as u64)));
    }
    

    // Command line number: 162
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095038 as u32),Immediate::Word(1937768448 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 163
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405310 as u64),Immediate::DoubleWord(8980177656976769024 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 166
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1073741824 as u32),Immediate::Word(1073741824 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1082130432 as u32)));
    }
    

    // Command line number: 167
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4611686018427387904 as u64),Immediate::DoubleWord(4611686018427387904 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4616189618054758400 as u64)));
    }
    

    // Command line number: 170
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(1929379839 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 171
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(1929379840 as u32)];
        let result = runtime.call_exported_function("f32.add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 172
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(8975674057349398527 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 173
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(8975674057349398528 as u64)];
        let result = runtime.call_exported_function("f64.add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 177
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1199570944 as u32),Immediate::Word(754974720 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1199570944 as u32)));
    }
    

    // Command line number: 178
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4679240012837945344 as u64),Immediate::DoubleWord(4440549232587309056 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4679240012837945343 as u64)));
    }
    

    // Command line number: 182
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(855638016 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 183
    #[test]
    fn test_82(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(855638017 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353215 as u32)));
    }
    

    // Command line number: 184
    #[test]
    fn test_83(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(4363988038922010624 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 185
    #[test]
    fn test_84(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(4363988038922010625 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017407 as u64)));
    }
    

    // Command line number: 188
    #[test]
    fn test_85(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(183964211 as u32),Immediate::Word(4211807167 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2064323519 as u32)));
    }
    

    // Command line number: 189
    #[test]
    fn test_86(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4213323727 as u32),Immediate::Word(3575761746 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4213323727 as u32)));
    }
    

    // Command line number: 190
    #[test]
    fn test_87(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1069169566 as u32),Immediate::Word(2530290313 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1069169566 as u32)));
    }
    

    // Command line number: 191
    #[test]
    fn test_88(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(88711906 as u32),Immediate::Word(431212897 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2578696545 as u32)));
    }
    

    // Command line number: 192
    #[test]
    fn test_89(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(919562194 as u32),Immediate::Word(2319656354 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(919562194 as u32)));
    }
    

    // Command line number: 193
    #[test]
    fn test_90(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(12332465020357998661 as u64),Immediate::DoubleWord(9766989582560416510 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(12332465020357998661 as u64)));
    }
    

    // Command line number: 194
    #[test]
    fn test_91(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(16318450733361321773 as u64),Immediate::DoubleWord(13120762196173477233 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(16318450733361321773 as u64)));
    }
    

    // Command line number: 195
    #[test]
    fn test_92(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2534186520144737456 as u64),Immediate::DoubleWord(10065159679028096147 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2534186520144737456 as u64)));
    }
    

    // Command line number: 196
    #[test]
    fn test_93(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(81253721542311597 as u64),Immediate::DoubleWord(14524181566355681001 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5300809529500905193 as u64)));
    }
    

    // Command line number: 197
    #[test]
    fn test_94(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13189545483472981053 as u64),Immediate::DoubleWord(11407195172005604952 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13189545483472981053 as u64)));
    }
    

    // Command line number: 200
    #[test]
    fn test_95(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4122014001 as u32),Immediate::Word(4158487026 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2010278623 as u32)));
    }
    

    // Command line number: 201
    #[test]
    fn test_96(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4130431355 as u32),Immediate::Word(1119299749 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4130431355 as u32)));
    }
    

    // Command line number: 202
    #[test]
    fn test_97(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2517909066 as u32),Immediate::Word(864524238 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3012007886 as u32)));
    }
    

    // Command line number: 203
    #[test]
    fn test_98(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2573484334 as u32),Immediate::Word(3114628459 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(967144811 as u32)));
    }
    

    // Command line number: 204
    #[test]
    fn test_99(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4044667730 as u32),Immediate::Word(576942556 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4044667730 as u32)));
    }
    

    // Command line number: 205
    #[test]
    fn test_100(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14742371419310964805 as u64),Immediate::DoubleWord(17888404506408184249 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(8665032469553408441 as u64)));
    }
    

    // Command line number: 206
    #[test]
    fn test_101(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2953146626865245854 as u64),Immediate::DoubleWord(2577681730975527916 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2953146626865245854 as u64)));
    }
    

    // Command line number: 207
    #[test]
    fn test_102(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(12932364735331397407 as u64),Immediate::DoubleWord(2320782934320318207 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(12932364735331397407 as u64)));
    }
    

    // Command line number: 208
    #[test]
    fn test_103(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9365060414983047910 as u64),Immediate::DoubleWord(12353777816259046974 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3130405779404271166 as u64)));
    }
    

    // Command line number: 209
    #[test]
    fn test_104(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14336992446210099104 as u64),Immediate::DoubleWord(5379485476305549444 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14602857513160325252 as u64)));
    }
    

    // Command line number: 212
    #[test]
    fn test_105(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1869943590 as u32),Immediate::Word(943887556 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1869943590 as u32)));
    }
    

    // Command line number: 213
    #[test]
    fn test_106(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3372906046 as u32),Immediate::Word(2919590399 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3372906046 as u32)));
    }
    

    // Command line number: 214
    #[test]
    fn test_107(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2912495853 as u32),Immediate::Word(2745492671 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2912495843 as u32)));
    }
    

    // Command line number: 215
    #[test]
    fn test_108(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3096029999 as u32),Immediate::Word(957859028 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3109853804 as u32)));
    }
    

    // Command line number: 216
    #[test]
    fn test_109(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2939165019 as u32),Immediate::Word(2346559691 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2939165019 as u32)));
    }
    

    // Command line number: 217
    #[test]
    fn test_110(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13126685627069188368 as u64),Immediate::DoubleWord(17084005755352353256 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(7860633718497577448 as u64)));
    }
    

    // Command line number: 218
    #[test]
    fn test_111(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(17250804263238954355 as u64),Immediate::DoubleWord(17897322818375888829 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(8673950781521113021 as u64)));
    }
    

    // Command line number: 219
    #[test]
    fn test_112(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1024624387795162319 as u64),Immediate::DoubleWord(5422801516870904507 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14646173553725680315 as u64)));
    }
    

    // Command line number: 220
    #[test]
    fn test_113(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(11250563508203208480 as u64),Immediate::DoubleWord(2160870138446053709 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(11384242175310282886 as u64)));
    }
    

    // Command line number: 221
    #[test]
    fn test_114(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(11315355526970152728 as u64),Immediate::DoubleWord(13767999508795332779 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4544627471940556971 as u64)));
    }
    

    // Command line number: 224
    #[test]
    fn test_115(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4259315722107263852 as u64),Immediate::DoubleWord(4089445689175118070 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4259315722107229795 as u64)));
    }
    

    // Command line number: 225
    #[test]
    fn test_116(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(188121565440651108 as u64),Immediate::DoubleWord(62416789530785743 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(188121565409457039 as u64)));
    }
    

    // Command line number: 226
    #[test]
    fn test_117(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(3467770562990504390 as u64),Immediate::DoubleWord(3710644584044210353 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(12934016620898986161 as u64)));
    }
    

    // Command line number: 227
    #[test]
    fn test_118(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13655868372788825570 as u64),Immediate::DoubleWord(13522732182992332383 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13655868372781906121 as u64)));
    }
    

    // Command line number: 228
    #[test]
    fn test_119(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4496158230847685281 as u64),Immediate::DoubleWord(4595312328816348364 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13818684364224065885 as u64)));
    }
    

    // Command line number: 231
    #[test]
    fn test_120(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(10165048318361601487 as u64),Immediate::DoubleWord(10384658282813060399 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1161286245958284573 as u64)));
    }
    

    // Command line number: 232
    #[test]
    fn test_121(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13982858477006823824 as u64),Immediate::DoubleWord(6264462250080870922 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(15487834286935646730 as u64)));
    }
    

    // Command line number: 233
    #[test]
    fn test_122(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5124509539958121241 as u64),Immediate::DoubleWord(1953255029772502995 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5124509539958121241 as u64)));
    }
    

    // Command line number: 234
    #[test]
    fn test_123(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2408125051325635052 as u64),Immediate::DoubleWord(10707017964771367822 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2408125051325635052 as u64)));
    }
    

    // Command line number: 235
    #[test]
    fn test_124(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4352135458168599028 as u64),Immediate::DoubleWord(1826599214642193119 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4352135458168599028 as u64)));
    }
    

    // Command line number: 239
    #[test]
    fn test_125(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1102651427 as u32),Immediate::Word(1078530011 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1101004328 as u32)));
    }
    

    // Command line number: 240
    #[test]
    fn test_126(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4627206743602008890 as u64),Immediate::DoubleWord(4614256656552045848 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4626322463883005335 as u64)));
    }
    

    // Command line number: 243
    #[test]
    fn test_127(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1245125372 as u32),Immediate::Word(1245125368 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 244
    #[test]
    fn test_128(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1240736760 as u32),Immediate::Word(1240736728 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1082130432 as u32)));
    }
    

    // Command line number: 245
    #[test]
    fn test_129(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1240736760 as u32),Immediate::Word(1240736712 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086324736 as u32)));
    }
    

    // Command line number: 246
    #[test]
    fn test_130(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1220759616 as u32),Immediate::Word(1220759584 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 247
    #[test]
    fn test_131(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1220759616 as u32),Immediate::Word(1220759552 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1073741824 as u32)));
    }
    

    // Command line number: 248
    #[test]
    fn test_132(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4838362400168542206 as u64),Immediate::DoubleWord(4838362400168542204 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 249
    #[test]
    fn test_133(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4835858800541171708 as u64),Immediate::DoubleWord(4835858800541171692 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4616189618054758400 as u64)));
    }
    

    // Command line number: 250
    #[test]
    fn test_134(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4835858800541171708 as u64),Immediate::DoubleWord(4835858800541171684 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4618441417868443648 as u64)));
    }
    

    // Command line number: 251
    #[test]
    fn test_135(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4825251601286430752 as u64),Immediate::DoubleWord(4825251601286430736 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 252
    #[test]
    fn test_136(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4825251601286430752 as u64),Immediate::DoubleWord(4825251601286430720 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4611686018427387904 as u64)));
    }
    

    // Command line number: 256
    #[test]
    fn test_137(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(4503599627370495 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 259
    #[test]
    fn test_138(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353217 as u32),Immediate::Word(1065353215 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(876609536 as u32)));
    }
    

    // Command line number: 260
    #[test]
    fn test_139(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353217 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(872415232 as u32)));
    }
    

    // Command line number: 261
    #[test]
    fn test_140(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(1065353215 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(864026624 as u32)));
    }
    

    // Command line number: 262
    #[test]
    fn test_141(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017409 as u64),Immediate::DoubleWord(4607182418800017407 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4375247037990436864 as u64)));
    }
    

    // Command line number: 263
    #[test]
    fn test_142(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017409 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4372995238176751616 as u64)));
    }
    

    // Command line number: 264
    #[test]
    fn test_143(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(4607182418800017407 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4368491638549381120 as u64)));
    }
    

    // Command line number: 268
    #[test]
    fn test_144(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(1929379839 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 269
    #[test]
    fn test_145(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(1929379840 as u32)];
        let result = runtime.call_exported_function("f32.sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095038 as u32)));
    }
    

    // Command line number: 270
    #[test]
    fn test_146(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(8975674057349398527 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 271
    #[test]
    fn test_147(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(8975674057349398528 as u64)];
        let result = runtime.call_exported_function("f64.sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405310 as u64)));
    }
    

    // Command line number: 274
    #[test]
    fn test_148(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1482907561 as u32),Immediate::Word(1482907561 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1900671689 as u32)));
    }
    

    // Command line number: 275
    #[test]
    fn test_149(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1621981420 as u32),Immediate::Word(1621981420 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 276
    #[test]
    fn test_150(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1761892689 as u32),Immediate::Word(1761892689 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 277
    #[test]
    fn test_151(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4831355200913801216 as u64),Immediate::DoubleWord(4831355200913801216 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5055640609639927018 as u64)));
    }
    

    // Command line number: 278
    #[test]
    fn test_152(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4906019910204099648 as u64),Immediate::DoubleWord(4906019910204099648 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5205425776111082661 as u64)));
    }
    

    // Command line number: 279
    #[test]
    fn test_153(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4981134201117475473 as u64),Immediate::DoubleWord(4981134201117475473 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5355091182177117339 as u64)));
    }
    

    // Command line number: 284
    #[test]
    fn test_154(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1323067183 as u32),Immediate::Word(1351920719 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1610612737 as u32)));
    }
    

    // Command line number: 285
    #[test]
    fn test_155(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4745541551160033280 as u64),Immediate::DoubleWord(4761032175258435584 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4899916394579099649 as u64)));
    }
    

    // Command line number: 289
    #[test]
    fn test_156(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1117401907 as u32),Immediate::Word(1146388480 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1199570688 as u32)));
    }
    

    // Command line number: 290
    #[test]
    fn test_157(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4635125847112967782 as u64),Immediate::DoubleWord(4650687894887858176 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4679239875398991871 as u64)));
    }
    

    // Command line number: 293
    #[test]
    fn test_158(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3725225879 as u32),Immediate::Word(767181884 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3427365876 as u32)));
    }
    

    // Command line number: 294
    #[test]
    fn test_159(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4071298289 as u32),Immediate::Word(2363594761 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1075233538 as u32)));
    }
    

    // Command line number: 295
    #[test]
    fn test_160(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3990403914 as u32),Immediate::Word(2406636213 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1036856791 as u32)));
    }
    

    // Command line number: 296
    #[test]
    fn test_161(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3876262739 as u32),Immediate::Word(2590162616 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1106664791 as u32)));
    }
    

    // Command line number: 297
    #[test]
    fn test_162(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1513973689 as u32),Immediate::Word(1110852822 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1560730283 as u32)));
    }
    

    // Command line number: 298
    #[test]
    fn test_163(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(15745248579468343268 as u64),Immediate::DoubleWord(8867411559652116184 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 299
    #[test]
    fn test_164(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9393595877903301723 as u64),Immediate::DoubleWord(12847704090045979814 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 300
    #[test]
    fn test_165(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(7339743909496900438 as u64),Immediate::DoubleWord(14312273837817528234 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(17044856006279008467 as u64)));
    }
    

    // Command line number: 301
    #[test]
    fn test_166(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(12422633289194844785 as u64),Immediate::DoubleWord(7233496047441461849 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(15049217706304651865 as u64)));
    }
    

    // Command line number: 302
    #[test]
    fn test_167(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(8188526721198436258 as u64),Immediate::DoubleWord(745378815681991665 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4327221634240578200 as u64)));
    }
    

    // Command line number: 305
    #[test]
    fn test_168(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2468326353 as u32),Immediate::Word(3984723636 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1092762840 as u32)));
    }
    

    // Command line number: 306
    #[test]
    fn test_169(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1680245441 as u32),Immediate::Word(1228341789 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1844318640 as u32)));
    }
    

    // Command line number: 307
    #[test]
    fn test_170(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2945969408 as u32),Immediate::Word(3840981390 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1427037263 as u32)));
    }
    

    // Command line number: 308
    #[test]
    fn test_171(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1631446143 as u32),Immediate::Word(3340593865 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3907618991 as u32)));
    }
    

    // Command line number: 309
    #[test]
    fn test_172(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1321841569 as u32),Immediate::Word(3771977928 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4029080461 as u32)));
    }
    

    // Command line number: 310
    #[test]
    fn test_173(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(10956034376207055635 as u64),Immediate::DoubleWord(3195464683520095288 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9544600684599159351 as u64)));
    }
    

    // Command line number: 311
    #[test]
    fn test_174(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(3366372084365197849 as u64),Immediate::DoubleWord(10971437597834234015 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9730951521726442211 as u64)));
    }
    

    // Command line number: 312
    #[test]
    fn test_175(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(15316253934569525376 as u64),Immediate::DoubleWord(17593766689751523793 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 313
    #[test]
    fn test_176(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(8873430008942894499 as u64),Immediate::DoubleWord(2630310872370005737 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(6896620497836925834 as u64)));
    }
    

    // Command line number: 314
    #[test]
    fn test_177(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6138390736122864607 as u64),Immediate::DoubleWord(14591737303103877464 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(16122953269734234204 as u64)));
    }
    

    // Command line number: 317
    #[test]
    fn test_178(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3603447621 as u32),Immediate::Word(4012809012 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 318
    #[test]
    fn test_179(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3192278970 as u32),Immediate::Word(242309673 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2369451750 as u32)));
    }
    

    // Command line number: 319
    #[test]
    fn test_180(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3060150317 as u32),Immediate::Word(1953896717 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3948759250 as u32)));
    }
    

    // Command line number: 320
    #[test]
    fn test_181(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3685735869 as u32),Immediate::Word(265571119 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2886850375 as u32)));
    }
    

    // Command line number: 321
    #[test]
    fn test_182(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3355295625 as u32),Immediate::Word(69023757 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2359023110 as u32)));
    }
    

    // Command line number: 322
    #[test]
    fn test_183(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(11598370094301102699 as u64),Immediate::DoubleWord(15783697741936198553 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4328598597203697163 as u64)));
    }
    

    // Command line number: 323
    #[test]
    fn test_184(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6760137556672200005 as u64),Immediate::DoubleWord(10390971429121674926 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(12543987340819631453 as u64)));
    }
    

    // Command line number: 324
    #[test]
    fn test_185(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14608361260346079328 as u64),Immediate::DoubleWord(6227029768418337647 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(16228420234814429284 as u64)));
    }
    

    // Command line number: 325
    #[test]
    fn test_186(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(8069908676175581646 as u64),Immediate::DoubleWord(15517844864458283108 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 326
    #[test]
    fn test_187(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4973816491418843402 as u64),Immediate::DoubleWord(4922606816281832528 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5289311430451755328 as u64)));
    }
    

    // Command line number: 329
    #[test]
    fn test_188(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(3075846375446710711 as u64),Immediate::DoubleWord(2019524319895558702 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(488220554609224151 as u64)));
    }
    

    // Command line number: 330
    #[test]
    fn test_189(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13792389967993824121 as u64),Immediate::DoubleWord(7865224186729676301 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(17051039770050770421 as u64)));
    }
    

    // Command line number: 331
    #[test]
    fn test_190(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(15483491341737357200 as u64),Immediate::DoubleWord(10898188329492193507 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3327855557076626973 as u64)));
    }
    

    // Command line number: 332
    #[test]
    fn test_191(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(8100886621587996114 as u64),Immediate::DoubleWord(4881220253517731398 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(8375006889399652877 as u64)));
    }
    

    // Command line number: 333
    #[test]
    fn test_192(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(12589892576446671178 as u64),Immediate::DoubleWord(8646729507413822745 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(16629483498509877989 as u64)));
    }
    

    // Command line number: 336
    #[test]
    fn test_193(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1913383041739811140 as u64),Immediate::DoubleWord(2696786534754768330 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3776602816350777 as u64)));
    }
    

    // Command line number: 337
    #[test]
    fn test_194(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13576886274997223712 as u64),Immediate::DoubleWord(9480836654241272360 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4406081533924885 as u64)));
    }
    

    // Command line number: 338
    #[test]
    fn test_195(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(11636123185307544831 as u64),Immediate::DoubleWord(11418839304735837198 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2919536087652621 as u64)));
    }
    

    // Command line number: 339
    #[test]
    fn test_196(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(10357270161024523488 as u64),Immediate::DoubleWord(3460198093814186274 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223701543089011575 as u64)));
    }
    

    // Command line number: 340
    #[test]
    fn test_197(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(11506848031881565957 as u64),Immediate::DoubleWord(11547514064126512393 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2474372522531115 as u64)));
    }
    

    // Command line number: 343
    #[test]
    fn test_198(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(8446730755803745435 as u64),Immediate::DoubleWord(640105622936255012 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4479981512316931443 as u64)));
    }
    

    // Command line number: 344
    #[test]
    fn test_199(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4578331547514318947 as u64),Immediate::DoubleWord(3937894457345907544 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3909603143436010489 as u64)));
    }
    
}

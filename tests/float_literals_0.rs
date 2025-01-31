
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_literals.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 105
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.nan", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2143289344 as u32)));
    }
    

    // Command line number: 106
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.positive_nan", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2143289344 as u32)));
    }
    

    // Command line number: 107
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.negative_nan", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4290772992 as u32)));
    }
    

    // Command line number: 108
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.plain_nan", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2143289344 as u32)));
    }
    

    // Command line number: 109
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.informally_known_as_plain_snan", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2141192192 as u32)));
    }
    

    // Command line number: 110
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.all_ones_nan", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 111
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.misc_nan", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139169605 as u32)));
    }
    

    // Command line number: 112
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.misc_positive_nan", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2142257232 as u32)));
    }
    

    // Command line number: 113
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.misc_negative_nan", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4289379550 as u32)));
    }
    

    // Command line number: 114
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.infinity", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 115
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.positive_infinity", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 116
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.negative_infinity", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 117
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.zero", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 118
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.positive_zero", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 119
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.negative_zero", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 120
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.misc", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 121
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.min_positive", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 122
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.min_normal", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 123
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.max_subnormal", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388607 as u32)));
    }
    

    // Command line number: 124
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.max_finite", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 125
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.trailing_dot", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1149239296 as u32)));
    }
    

    // Command line number: 126
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_dec.zero", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 127
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_dec.positive_zero", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 128
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_dec.negative_zero", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 129
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_dec.misc", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 130
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_dec.min_positive", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 131
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_dec.min_normal", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 132
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_dec.max_subnormal", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388607 as u32)));
    }
    

    // Command line number: 133
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_dec.max_finite", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 134
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_dec.trailing_dot", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1343554297 as u32)));
    }
    

    // Command line number: 135
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_dec.root_beer_float", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353217 as u32)));
    }
    

    // Command line number: 137
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.nan", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9221120237041090560 as u64)));
    }
    

    // Command line number: 138
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.positive_nan", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9221120237041090560 as u64)));
    }
    

    // Command line number: 139
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.negative_nan", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18444492273895866368 as u64)));
    }
    

    // Command line number: 140
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.plain_nan", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9221120237041090560 as u64)));
    }
    

    // Command line number: 141
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.informally_known_as_plain_snan", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9219994337134247936 as u64)));
    }
    

    // Command line number: 142
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.all_ones_nan", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551615 as u64)));
    }
    

    // Command line number: 143
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.misc_nan", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218888453225749180 as u64)));
    }
    

    // Command line number: 144
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.misc_positive_nan", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9219717281780008969 as u64)));
    }
    

    // Command line number: 145
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.misc_negative_nan", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442992325002076997 as u64)));
    }
    

    // Command line number: 146
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.infinity", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 147
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.positive_infinity", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 148
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.negative_infinity", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 149
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.zero", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 150
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.positive_zero", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 151
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.negative_zero", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 152
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.misc", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4618760256179416344 as u64)));
    }
    

    // Command line number: 153
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.min_positive", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 154
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.min_normal", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4503599627370496 as u64)));
    }
    

    // Command line number: 155
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.max_subnormal", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4503599627370495 as u64)));
    }
    

    // Command line number: 156
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.max_finite", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 157
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64.trailing_dot", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5057542381537067008 as u64)));
    }
    

    // Command line number: 158
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_dec.zero", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 159
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_dec.positive_zero", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 160
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_dec.negative_zero", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 161
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_dec.misc", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4618760256179416344 as u64)));
    }
    

    // Command line number: 162
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_dec.min_positive", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 163
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_dec.min_normal", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4503599627370496 as u64)));
    }
    

    // Command line number: 164
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_dec.max_subnormal", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4503599627370495 as u64)));
    }
    

    // Command line number: 165
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_dec.max_finite", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 166
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_dec.trailing_dot", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(6103021453049119613 as u64)));
    }
    

    // Command line number: 167
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_dec.root_beer_float", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182419335945764 as u64)));
    }
    

    // Command line number: 169
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32-dec-sep1", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1232348160 as u32)));
    }
    

    // Command line number: 170
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32-dec-sep2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1148846080 as u32)));
    }
    

    // Command line number: 171
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32-dec-sep3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1148897552 as u32)));
    }
    

    // Command line number: 172
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32-dec-sep4", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1482758550 as u32)));
    }
    

    // Command line number: 173
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32-dec-sep5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1847438964 as u32)));
    }
    

    // Command line number: 174
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32-hex-sep1", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1294004234 as u32)));
    }
    

    // Command line number: 175
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32-hex-sep2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1205143424 as u32)));
    }
    

    // Command line number: 176
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32-hex-sep3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1193345009 as u32)));
    }
    

    // Command line number: 177
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32-hex-sep4", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1240465408 as u32)));
    }
    

    // Command line number: 178
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32-hex-sep5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1437319208 as u32)));
    }
    

    // Command line number: 180
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64-dec-sep1", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4696837146684686336 as u64)));
    }
    

    // Command line number: 181
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64-dec-sep2", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4652007308841189376 as u64)));
    }
    

    // Command line number: 182
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64-dec-sep3", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4652034942576659200 as u64)));
    }
    

    // Command line number: 183
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64-dec-sep4", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2796837019126844485 as u64)));
    }
    

    // Command line number: 184
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64-dec-sep5", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5027061507362119324 as u64)));
    }
    

    // Command line number: 185
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64-hex-sep1", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4838519794133185330 as u64)));
    }
    

    // Command line number: 186
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64-hex-sep2", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4682231715257647104 as u64)));
    }
    

    // Command line number: 187
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64-hex-sep3", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4675897489574114112 as u64)));
    }
    

    // Command line number: 188
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64-hex-sep4", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4701195061021376512 as u64)));
    }
    

    // Command line number: 189
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64-hex-sep5", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4806880140420149248 as u64)));
    }
    
}

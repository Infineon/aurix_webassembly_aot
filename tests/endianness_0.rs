
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "endianness.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 133
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("i32_load16_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 134
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294963054 as u32)];
        let result = runtime.call_exported_function("i32_load16_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294963054 as u32)));
    }
    

    // Command line number: 135
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(42 as u32)];
        let result = runtime.call_exported_function("i32_load16_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(42 as u32)));
    }
    

    // Command line number: 136
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(12816 as u32)];
        let result = runtime.call_exported_function("i32_load16_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(12816 as u32)));
    }
    

    // Command line number: 138
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("i32_load16_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(65535 as u32)));
    }
    

    // Command line number: 139
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294963054 as u32)];
        let result = runtime.call_exported_function("i32_load16_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(61294 as u32)));
    }
    

    // Command line number: 140
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(42 as u32)];
        let result = runtime.call_exported_function("i32_load16_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(42 as u32)));
    }
    

    // Command line number: 141
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(51966 as u32)];
        let result = runtime.call_exported_function("i32_load16_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(51966 as u32)));
    }
    

    // Command line number: 143
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("i32_load", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 144
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4252543054 as u32)];
        let result = runtime.call_exported_function("i32_load", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4252543054 as u32)));
    }
    

    // Command line number: 145
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(42424242 as u32)];
        let result = runtime.call_exported_function("i32_load", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(42424242 as u32)));
    }
    

    // Command line number: 146
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2880249322 as u32)];
        let result = runtime.call_exported_function("i32_load", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2880249322 as u32)));
    }
    

    // Command line number: 148
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709551615 as u64)];
        let result = runtime.call_exported_function("i64_load16_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551615 as u64)));
    }
    

    // Command line number: 149
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709547374 as u64)];
        let result = runtime.call_exported_function("i64_load16_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709547374 as u64)));
    }
    

    // Command line number: 150
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(42 as u64)];
        let result = runtime.call_exported_function("i64_load16_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(42 as u64)));
    }
    

    // Command line number: 151
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(12816 as u64)];
        let result = runtime.call_exported_function("i64_load16_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(12816 as u64)));
    }
    

    // Command line number: 153
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709551615 as u64)];
        let result = runtime.call_exported_function("i64_load16_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(65535 as u64)));
    }
    

    // Command line number: 154
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709547374 as u64)];
        let result = runtime.call_exported_function("i64_load16_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(61294 as u64)));
    }
    

    // Command line number: 155
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(42 as u64)];
        let result = runtime.call_exported_function("i64_load16_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(42 as u64)));
    }
    

    // Command line number: 156
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(51966 as u64)];
        let result = runtime.call_exported_function("i64_load16_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(51966 as u64)));
    }
    

    // Command line number: 158
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709551615 as u64)];
        let result = runtime.call_exported_function("i64_load32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551615 as u64)));
    }
    

    // Command line number: 159
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073667127374 as u64)];
        let result = runtime.call_exported_function("i64_load32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073667127374 as u64)));
    }
    

    // Command line number: 160
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(42424242 as u64)];
        let result = runtime.call_exported_function("i64_load32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(42424242 as u64)));
    }
    

    // Command line number: 161
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(305419896 as u64)];
        let result = runtime.call_exported_function("i64_load32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(305419896 as u64)));
    }
    

    // Command line number: 163
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709551615 as u64)];
        let result = runtime.call_exported_function("i64_load32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4294967295 as u64)));
    }
    

    // Command line number: 164
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073667127374 as u64)];
        let result = runtime.call_exported_function("i64_load32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4252543054 as u64)));
    }
    

    // Command line number: 165
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(42424242 as u64)];
        let result = runtime.call_exported_function("i64_load32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(42424242 as u64)));
    }
    

    // Command line number: 166
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2880249322 as u64)];
        let result = runtime.call_exported_function("i64_load32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2880249322 as u64)));
    }
    

    // Command line number: 168
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709551615 as u64)];
        let result = runtime.call_exported_function("i64_load", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551615 as u64)));
    }
    

    // Command line number: 169
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073667127374 as u64)];
        let result = runtime.call_exported_function("i64_load", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073667127374 as u64)));
    }
    

    // Command line number: 170
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2880249322 as u64)];
        let result = runtime.call_exported_function("i64_load", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2880249322 as u64)));
    }
    

    // Command line number: 171
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(12370766947463011818 as u64)];
        let result = runtime.call_exported_function("i64_load", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(12370766947463011818 as u64)));
    }
    

    // Command line number: 173
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("f32_load", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 174
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1011494326 as u32)];
        let result = runtime.call_exported_function("f32_load", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1011494326 as u32)));
    }
    

    // Command line number: 175
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1166316389 as u32)];
        let result = runtime.call_exported_function("f32_load", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1166316389 as u32)));
    }
    

    // Command line number: 176
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("f32_load", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 178
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("f64_load", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13830554455654793216 as u64)));
    }
    

    // Command line number: 179
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4653144502447687399 as u64)];
        let result = runtime.call_exported_function("f64_load", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4653144502447687399 as u64)));
    }
    

    // Command line number: 180
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4691032041816096430 as u64)];
        let result = runtime.call_exported_function("f64_load", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4691032041816096430 as u64)));
    }
    

    // Command line number: 181
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("f64_load", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 184
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("i32_store16", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(65535 as u32)));
    }
    

    // Command line number: 185
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294963054 as u32)];
        let result = runtime.call_exported_function("i32_store16", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(61294 as u32)));
    }
    

    // Command line number: 186
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(42 as u32)];
        let result = runtime.call_exported_function("i32_store16", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(42 as u32)));
    }
    

    // Command line number: 187
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(51966 as u32)];
        let result = runtime.call_exported_function("i32_store16", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(51966 as u32)));
    }
    

    // Command line number: 189
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("i32_store", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 190
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294963054 as u32)];
        let result = runtime.call_exported_function("i32_store", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294963054 as u32)));
    }
    

    // Command line number: 191
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(42424242 as u32)];
        let result = runtime.call_exported_function("i32_store", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(42424242 as u32)));
    }
    

    // Command line number: 192
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3735931646 as u32)];
        let result = runtime.call_exported_function("i32_store", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3735931646 as u32)));
    }
    

    // Command line number: 194
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709551615 as u64)];
        let result = runtime.call_exported_function("i64_store16", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(65535 as u64)));
    }
    

    // Command line number: 195
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709547374 as u64)];
        let result = runtime.call_exported_function("i64_store16", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(61294 as u64)));
    }
    

    // Command line number: 196
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(42 as u64)];
        let result = runtime.call_exported_function("i64_store16", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(42 as u64)));
    }
    

    // Command line number: 197
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(51966 as u64)];
        let result = runtime.call_exported_function("i64_store16", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(51966 as u64)));
    }
    

    // Command line number: 199
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709551615 as u64)];
        let result = runtime.call_exported_function("i64_store32", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4294967295 as u64)));
    }
    

    // Command line number: 200
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709547374 as u64)];
        let result = runtime.call_exported_function("i64_store32", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4294963054 as u64)));
    }
    

    // Command line number: 201
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(42424242 as u64)];
        let result = runtime.call_exported_function("i64_store32", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(42424242 as u64)));
    }
    

    // Command line number: 202
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(3735931646 as u64)];
        let result = runtime.call_exported_function("i64_store32", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3735931646 as u64)));
    }
    

    // Command line number: 204
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709551615 as u64)];
        let result = runtime.call_exported_function("i64_store", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551615 as u64)));
    }
    

    // Command line number: 205
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073667127374 as u64)];
        let result = runtime.call_exported_function("i64_store", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073667127374 as u64)));
    }
    

    // Command line number: 206
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2880249322 as u64)];
        let result = runtime.call_exported_function("i64_store", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2880249322 as u64)));
    }
    

    // Command line number: 207
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(12370766947463011818 as u64)];
        let result = runtime.call_exported_function("i64_store", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(12370766947463011818 as u64)));
    }
    

    // Command line number: 209
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("f32_store", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 210
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1011494326 as u32)];
        let result = runtime.call_exported_function("f32_store", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1011494326 as u32)));
    }
    

    // Command line number: 211
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1166316389 as u32)];
        let result = runtime.call_exported_function("f32_store", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1166316389 as u32)));
    }
    

    // Command line number: 212
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("f32_store", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 214
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("f64_store", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13830554455654793216 as u64)));
    }
    

    // Command line number: 215
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4653144502447687399 as u64)];
        let result = runtime.call_exported_function("f64_store", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4653144502447687399 as u64)));
    }
    

    // Command line number: 216
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4691032041816096430 as u64)];
        let result = runtime.call_exported_function("f64_store", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4691032041816096430 as u64)));
    }
    

    // Command line number: 217
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("f64_store", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    
}

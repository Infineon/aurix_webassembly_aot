
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "left-to-right.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 181
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 181
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 182
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 182
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 183
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 183
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 184
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_div_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 184
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_div_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 185
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_div_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 185
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_div_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 186
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_rem_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 186
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_rem_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 187
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_rem_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 187
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_rem_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 188
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_and", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 188
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_and", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 189
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_or", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 189
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_or", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 190
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_xor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 190
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_xor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 191
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_shl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 191
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_shl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 192
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_shr_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 192
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_shr_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 193
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_shr_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 193
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_shr_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 194
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_eq", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 194
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_eq", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 195
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_ne", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 195
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_ne", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 196
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_lt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 196
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_lt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 197
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_le_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 197
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_le_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 198
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_lt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 198
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_lt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 199
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_le_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 199
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_le_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 200
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_gt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 200
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_gt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 201
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_ge_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 201
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_ge_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 202
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_gt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 202
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_gt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 203
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_ge_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 203
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_ge_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 204
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_store", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 204
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_store", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 205
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_store8", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 205
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_store8", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 206
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_store16", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 206
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_store16", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 207
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_store32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 208
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_call", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 208
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_call", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 209
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_call_indirect", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(66052 as u32)));
    }
    

    // Command line number: 210
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_call_indirect", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(66052 as u32)));
    }
    

    // Command line number: 211
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32_select", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(66053 as u32)));
    }
    

    // Command line number: 211
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i64_select", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(66053 as u32)));
    }
    

    // Command line number: 213
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 213
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 214
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 214
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 215
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 215
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 216
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 216
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 217
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_copysign", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 217
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_copysign", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 218
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_eq", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 218
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_eq", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 219
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_ne", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 219
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_ne", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 220
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 220
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_lt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 221
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_le", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 221
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_le", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 222
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_gt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 222
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_gt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 223
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_ge", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 223
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_ge", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 224
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 224
    #[test]
    fn test_82(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 225
    #[test]
    fn test_83(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 225
    #[test]
    fn test_84(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 226
    #[test]
    fn test_85(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_store", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 226
    #[test]
    fn test_86(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_store", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 227
    #[test]
    fn test_87(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_call", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 227
    #[test]
    fn test_88(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_call", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 228
    #[test]
    fn test_89(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_call_indirect", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(66052 as u32)));
    }
    

    // Command line number: 229
    #[test]
    fn test_90(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_call_indirect", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(66052 as u32)));
    }
    

    // Command line number: 230
    #[test]
    fn test_91(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32_select", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(66053 as u32)));
    }
    

    // Command line number: 230
    #[test]
    fn test_92(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f64_select", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(66053 as u32)));
    }
    

    // Command line number: 232
    #[test]
    fn test_93(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("br_if", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    

    // Command line number: 233
    #[test]
    fn test_94(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("br_table", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(258 as u32)));
    }
    
}

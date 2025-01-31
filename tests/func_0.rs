
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "func.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 171
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-use-1", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 172
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-use-2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 173
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("type-use-3", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 175
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::DoubleWord(4607182418800017408 as u64),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("type-use-4", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 178
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-use-5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 179
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("type-use-6", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 181
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::DoubleWord(4607182418800017408 as u64),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("type-use-7", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 185
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("local-first-i32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 186
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("local-first-i64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 187
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("local-first-f32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 188
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("local-first-f64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 189
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("local-second-i32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 190
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("local-second-i64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 191
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("local-second-f32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 192
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("local-second-f64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 193
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("local-mixed", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 196
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32),Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("param-first-i32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 199
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2 as u64),Immediate::DoubleWord(3 as u64)];
        let result = runtime.call_exported_function("param-first-i64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2 as u64)));
    }
    

    // Command line number: 202
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1073741824 as u32),Immediate::Word(1077936128 as u32)];
        let result = runtime.call_exported_function("param-first-f32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1073741824 as u32)));
    }
    

    // Command line number: 205
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4611686018427387904 as u64),Immediate::DoubleWord(4613937818241073152 as u64)];
        let result = runtime.call_exported_function("param-first-f64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4611686018427387904 as u64)));
    }
    

    // Command line number: 208
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32),Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("param-second-i32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 211
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2 as u64),Immediate::DoubleWord(3 as u64)];
        let result = runtime.call_exported_function("param-second-i64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3 as u64)));
    }
    

    // Command line number: 214
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1073741824 as u32),Immediate::Word(1077936128 as u32)];
        let result = runtime.call_exported_function("param-second-f32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1077936128 as u32)));
    }
    

    // Command line number: 217
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4611686018427387904 as u64),Immediate::DoubleWord(4613937818241073152 as u64)];
        let result = runtime.call_exported_function("param-second-f64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4613937818241073152 as u64)));
    }
    

    // Command line number: 221
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2 as u32),Immediate::DoubleWord(3 as u64),Immediate::Word(4 as u32),Immediate::DoubleWord(4617878467915022336 as u64),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("param-mixed", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4617878467915022336 as u64)));
    }
    

    // Command line number: 228
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("empty", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 229
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("value-void", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 230
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("value-i32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(77 as u32)));
    }
    

    // Command line number: 231
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("value-i64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(7777 as u64)));
    }
    

    // Command line number: 232
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("value-f32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1117480550 as u32)));
    }
    

    // Command line number: 233
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("value-f64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4635172994171566817 as u64)));
    }
    

    // Command line number: 234
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("value-block-void", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 235
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("value-block-i32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(77 as u32)));
    }
    

    // Command line number: 237
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("return-empty", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 238
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("return-i32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(78 as u32)));
    }
    

    // Command line number: 239
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("return-i64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(7878 as u64)));
    }
    

    // Command line number: 240
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("return-f32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1117611622 as u32)));
    }
    

    // Command line number: 241
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("return-f64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4635244066603186258 as u64)));
    }
    

    // Command line number: 242
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("return-block-i32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(77 as u32)));
    }
    

    // Command line number: 244
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("break-empty", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 245
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("break-i32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(79 as u32)));
    }
    

    // Command line number: 246
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("break-i64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(7979 as u64)));
    }
    

    // Command line number: 247
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("break-f32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1117768909 as u32)));
    }
    

    // Command line number: 248
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("break-f64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4635315139034805699 as u64)));
    }
    

    // Command line number: 249
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("break-block-i32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(77 as u32)));
    }
    

    // Command line number: 251
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("break-br_if-empty", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 252
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("break-br_if-empty", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 253
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("break-br_if-num", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(51 as u32)));
    }
    

    // Command line number: 254
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("break-br_if-num", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(50 as u32)));
    }
    

    // Command line number: 256
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("break-br_table-empty", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 257
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("break-br_table-empty", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 258
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32)];
        let result = runtime.call_exported_function("break-br_table-empty", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 259
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("break-br_table-empty", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 260
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("break-br_table-num", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(50 as u32)));
    }
    

    // Command line number: 261
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("break-br_table-num", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(50 as u32)));
    }
    

    // Command line number: 262
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(10 as u32)];
        let result = runtime.call_exported_function("break-br_table-num", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(50 as u32)));
    }
    

    // Command line number: 263
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967196 as u32)];
        let result = runtime.call_exported_function("break-br_table-num", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(50 as u32)));
    }
    

    // Command line number: 264
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("break-br_table-nested-empty", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 265
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("break-br_table-nested-empty", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 266
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("break-br_table-nested-empty", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 267
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967294 as u32)];
        let result = runtime.call_exported_function("break-br_table-nested-empty", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 269
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("break-br_table-nested-num", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(52 as u32)));
    }
    

    // Command line number: 272
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("break-br_table-nested-num", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(50 as u32)));
    }
    

    // Command line number: 275
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("break-br_table-nested-num", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(52 as u32)));
    }
    

    // Command line number: 278
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967293 as u32)];
        let result = runtime.call_exported_function("break-br_table-nested-num", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(52 as u32)));
    }
    

    // Command line number: 281
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("init-local-i32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 282
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("init-local-i64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 283
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("init-local-f32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 284
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("init-local-f64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    
}

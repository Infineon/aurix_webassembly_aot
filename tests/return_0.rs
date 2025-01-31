
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "return.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 224
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-i32", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 225
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-i64", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 226
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-f32", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 227
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-f64", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 229
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-i32-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 230
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-i64-value", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2 as u64)));
    }
    

    // Command line number: 231
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-f32-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1077936128 as u32)));
    }
    

    // Command line number: 232
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-f64-value", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4616189618054758400 as u64)));
    }
    

    // Command line number: 234
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("nullary", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 235
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("unary", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4613937818241073152 as u64)));
    }
    

    // Command line number: 237
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-func-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 238
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-func-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 239
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-func-last", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 240
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-func-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 242
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-block-first", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 243
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-block-mid", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 244
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-block-last", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 245
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-block-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 247
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-loop-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 248
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-loop-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4 as u32)));
    }
    

    // Command line number: 249
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-loop-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(5 as u32)));
    }
    

    // Command line number: 251
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 253
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_if-cond", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 254
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_if-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 255
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_if-value-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 257
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_table-index", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9 as u64)));
    }
    

    // Command line number: 258
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_table-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(10 as u32)));
    }
    

    // Command line number: 259
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_table-value-index", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(11 as u32)));
    }
    

    // Command line number: 261
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-return-value", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(7 as u64)));
    }
    

    // Command line number: 263
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-if-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 264
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-if-then", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 265
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-if-then", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 266
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-if-else", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4 as u32)));
    }
    

    // Command line number: 267
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-if-else", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 269
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-select-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(5 as u32)));
    }
    

    // Command line number: 270
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-select-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(5 as u32)));
    }
    

    // Command line number: 271
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-select-second", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 272
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-select-second", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 273
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-select-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(7 as u32)));
    }
    

    // Command line number: 275
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(12 as u32)));
    }
    

    // Command line number: 276
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(13 as u32)));
    }
    

    // Command line number: 277
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(14 as u32)));
    }
    

    // Command line number: 279
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-func", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(20 as u32)));
    }
    

    // Command line number: 280
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(21 as u32)));
    }
    

    // Command line number: 281
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(22 as u32)));
    }
    

    // Command line number: 282
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(23 as u32)));
    }
    

    // Command line number: 284
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-local.set-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(17 as u32)));
    }
    

    // Command line number: 285
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-local.tee-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 286
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-global.set-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 288
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-load-address", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1071225242 as u32)));
    }
    

    // Command line number: 289
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-loadN-address", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(30 as u64)));
    }
    

    // Command line number: 291
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-store-address", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(30 as u32)));
    }
    

    // Command line number: 292
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-store-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(31 as u32)));
    }
    

    // Command line number: 293
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-storeN-address", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(32 as u32)));
    }
    

    // Command line number: 294
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-storeN-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 296
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-unary-operand", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1079613850 as u32)));
    }
    

    // Command line number: 298
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-binary-left", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 299
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-binary-right", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(45 as u64)));
    }
    

    // Command line number: 301
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-test-operand", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(44 as u32)));
    }
    

    // Command line number: 303
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-compare-left", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(43 as u32)));
    }
    

    // Command line number: 304
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-compare-right", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(42 as u32)));
    }
    

    // Command line number: 306
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-convert-operand", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(41 as u32)));
    }
    

    // Command line number: 308
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-memory.grow-size", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(40 as u32)));
    }
    
}

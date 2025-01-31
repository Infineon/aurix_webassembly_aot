
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "memory_grow.4.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 259
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 261
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_if-cond", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 262
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_if-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 263
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_if-value-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 265
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_table-index", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 266
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_table-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 267
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_table-value-index", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 269
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-return-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 271
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-if-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 272
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-if-then", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 273
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-if-else", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 275
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-select-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 276
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-select-second", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 277
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-select-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 279
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 280
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 281
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 283
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 284
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 285
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 288
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-local.set-value", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 289
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-local.tee-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 290
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-global.set-value", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 292
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-load-address", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 293
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-loadN-address", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 294
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-store-address", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 295
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-store-value", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 296
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-storeN-address", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 297
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-storeN-value", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 299
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-unary-operand", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(31 as u32)));
    }
    

    // Command line number: 301
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-binary-left", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(11 as u32)));
    }
    

    // Command line number: 302
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-binary-right", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 304
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-test-operand", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 306
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-compare-left", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 307
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-compare-right", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 309
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-memory.grow-size", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    
}

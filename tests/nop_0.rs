
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "nop.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 306
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-func-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 307
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-func-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 308
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-func-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 309
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-func-everywhere", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4 as u32)));
    }
    

    // Command line number: 311
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-drop-first", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 312
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-drop-last", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 313
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-drop-everywhere", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 315
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("as-select-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 316
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("as-select-mid1", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 317
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("as-select-mid2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 318
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("as-select-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 319
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("as-select-everywhere", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 321
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-block-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 322
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-block-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 323
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-block-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 324
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-block-everywhere", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4 as u32)));
    }
    

    // Command line number: 326
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-loop-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 327
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-loop-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 328
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-loop-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 329
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-loop-everywhere", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4 as u32)));
    }
    

    // Command line number: 331
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-if-condition", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 332
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("as-if-condition", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 333
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-if-then", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 334
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32)];
        let result = runtime.call_exported_function("as-if-then", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 335
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-if-else", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 336
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("as-if-else", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 338
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32)];
        let result = runtime.call_exported_function("as-br-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(5 as u32)));
    }
    

    // Command line number: 339
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-br-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 340
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(7 as u32)];
        let result = runtime.call_exported_function("as-br-everywhere", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(7 as u32)));
    }
    

    // Command line number: 342
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32)];
        let result = runtime.call_exported_function("as-br_if-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4 as u32)));
    }
    

    // Command line number: 343
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32)];
        let result = runtime.call_exported_function("as-br_if-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(5 as u32)));
    }
    

    // Command line number: 344
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-br_if-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 345
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(7 as u32)];
        let result = runtime.call_exported_function("as-br_if-everywhere", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(7 as u32)));
    }
    

    // Command line number: 347
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32)];
        let result = runtime.call_exported_function("as-br_table-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4 as u32)));
    }
    

    // Command line number: 348
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32)];
        let result = runtime.call_exported_function("as-br_table-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(5 as u32)));
    }
    

    // Command line number: 349
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-br_table-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 350
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(7 as u32)];
        let result = runtime.call_exported_function("as-br_table-everywhere", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(7 as u32)));
    }
    

    // Command line number: 352
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32)];
        let result = runtime.call_exported_function("as-return-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(5 as u32)));
    }
    

    // Command line number: 353
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-return-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 354
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(7 as u32)];
        let result = runtime.call_exported_function("as-return-everywhere", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(7 as u32)));
    }
    

    // Command line number: 356
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32),Immediate::Word(1 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("as-call-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 357
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32),Immediate::Word(1 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("as-call-mid1", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 358
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(3 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-call-mid2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 359
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(10 as u32),Immediate::Word(9 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("as-call-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(20 as u32)));
    }
    

    // Command line number: 360
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32),Immediate::Word(1 as u32),Immediate::Word(5 as u32)];
        let result = runtime.call_exported_function("as-call-everywhere", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967294 as u32)));
    }
    

    // Command line number: 362
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(30 as u32)];
        let result = runtime.call_exported_function("as-unary-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 363
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(30 as u32)];
        let result = runtime.call_exported_function("as-unary-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 364
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(12 as u32)];
        let result = runtime.call_exported_function("as-unary-everywhere", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 366
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("as-binary-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 367
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("as-binary-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 368
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("as-binary-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 369
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("as-binary-everywhere", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 371
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-test-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 372
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-test-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 373
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-test-everywhere", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 375
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("as-compare-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 376
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("as-compare-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 377
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("as-compare-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 378
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("as-compare-everywhere", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 380
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-memory.grow-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 381
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("as-memory.grow-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 382
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(12 as u32)];
        let result = runtime.call_exported_function("as-memory.grow-everywhere", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 384
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 385
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-mid1", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 386
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-mid2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 387
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 388
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-everywhere", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 390
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-local.set-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 391
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-local.set-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 392
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-local.set-everywhere", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 394
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-local.tee-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 395
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-local.tee-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 396
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-local.tee-everywhere", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 398
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-global.set-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 399
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-global.set-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 400
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-global.set-everywhere", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 402
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(100 as u32)];
        let result = runtime.call_exported_function("as-load-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 403
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(100 as u32)];
        let result = runtime.call_exported_function("as-load-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 404
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(100 as u32)];
        let result = runtime.call_exported_function("as-load-everywhere", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 406
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-store-first", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 407
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("as-store-mid", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 408
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("as-store-last", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 409
    #[test]
    fn test_82(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(4 as u32)];
        let result = runtime.call_exported_function("as-store-everywhere", args, None);
        assert_eq!(result, None);
    }
    
}

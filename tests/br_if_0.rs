
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "br_if.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 372
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-i32", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 373
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-i64", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 374
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-f32", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 375
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-f64", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 377
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-i32-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 378
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-i64-value", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2 as u64)));
    }
    

    // Command line number: 379
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-f32-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1077936128 as u32)));
    }
    

    // Command line number: 380
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-f64-value", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4616189618054758400 as u64)));
    }
    

    // Command line number: 382
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-block-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 383
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-block-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 384
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-block-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 385
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-block-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 386
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-block-last", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 387
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-block-last", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 389
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-block-first-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(11 as u32)));
    }
    

    // Command line number: 390
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-block-first-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(10 as u32)));
    }
    

    // Command line number: 391
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-block-mid-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(21 as u32)));
    }
    

    // Command line number: 392
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-block-mid-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(20 as u32)));
    }
    

    // Command line number: 393
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-block-last-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(11 as u32)));
    }
    

    // Command line number: 394
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-block-last-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(11 as u32)));
    }
    

    // Command line number: 396
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-loop-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 397
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-loop-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 398
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-loop-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 399
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-loop-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4 as u32)));
    }
    

    // Command line number: 400
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-loop-last", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 401
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-loop-last", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 403
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 405
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_if-cond", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 406
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_if-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 407
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-br_if-value-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 408
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-br_if-value-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 410
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_table-index", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 411
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_table-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 412
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_table-value-index", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 414
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-return-value", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 416
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-if-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 417
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-if-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 418
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-if-then", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 419
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-if-then", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 420
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-if-then", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 421
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-if-then", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 422
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-if-else", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 423
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-if-else", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 424
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-if-else", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 425
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-if-else", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 427
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-select-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 428
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-select-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 429
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-select-second", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 430
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-select-second", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 431
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-select-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 433
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(12 as u32)));
    }
    

    // Command line number: 434
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(13 as u32)));
    }
    

    // Command line number: 435
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(14 as u32)));
    }
    

    // Command line number: 437
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-func", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4 as u32)));
    }
    

    // Command line number: 438
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4 as u32)));
    }
    

    // Command line number: 439
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4 as u32)));
    }
    

    // Command line number: 440
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4 as u32)));
    }
    

    // Command line number: 442
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-local.set-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 443
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-local.set-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(17 as u32)));
    }
    

    // Command line number: 445
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-local.tee-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 446
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-local.tee-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 448
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("as-global.set-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 449
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("as-global.set-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 451
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-load-address", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 452
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-loadN-address", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(30 as u32)));
    }
    

    // Command line number: 454
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-store-address", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(30 as u32)));
    }
    

    // Command line number: 455
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-store-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(31 as u32)));
    }
    

    // Command line number: 456
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-storeN-address", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(32 as u32)));
    }
    

    // Command line number: 457
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-storeN-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 459
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-unary-operand", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 460
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-binary-left", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 461
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-binary-right", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 462
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-test-operand", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 463
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-compare-left", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 464
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-compare-right", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 465
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-memory.grow-size", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 467
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("nested-block-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(21 as u32)));
    }
    

    // Command line number: 468
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nested-block-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 469
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("nested-br-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(5 as u32)));
    }
    

    // Command line number: 470
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nested-br-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 471
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(5 as u32)));
    }
    

    // Command line number: 472
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 473
    #[test]
    fn test_82(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(5 as u32)));
    }
    

    // Command line number: 474
    #[test]
    fn test_83(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 475
    #[test]
    fn test_84(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(5 as u32)));
    }
    

    // Command line number: 476
    #[test]
    fn test_85(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 477
    #[test]
    fn test_86(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value-index", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(5 as u32)));
    }
    

    // Command line number: 478
    #[test]
    fn test_87(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value-index", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    
}

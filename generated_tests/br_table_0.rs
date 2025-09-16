
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

            let wasm_code = include_bytes!(concat!("../wasm_json_from_wast/", "br_table.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 525
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-i32", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 526
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-i64", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 527
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-f32", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 528
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-f64", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 530
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-i32-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 531
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-i64-value", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2 as u64)));
    }
    

    // Command line number: 532
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-f32-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1077936128 as u32)));
    }
    

    // Command line number: 533
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-f64-value", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4616189618054758400 as u64)));
    }
    

    // Command line number: 535
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("empty", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(22 as u32)));
    }
    

    // Command line number: 536
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("empty", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(22 as u32)));
    }
    

    // Command line number: 537
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(11 as u32)];
        let result = runtime.call_exported_function("empty", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(22 as u32)));
    }
    

    // Command line number: 538
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("empty", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(22 as u32)));
    }
    

    // Command line number: 539
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967196 as u32)];
        let result = runtime.call_exported_function("empty", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(22 as u32)));
    }
    

    // Command line number: 540
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("empty", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(22 as u32)));
    }
    

    // Command line number: 542
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("empty-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 543
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("empty-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 544
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(11 as u32)];
        let result = runtime.call_exported_function("empty-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 545
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("empty-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 546
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967196 as u32)];
        let result = runtime.call_exported_function("empty-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 547
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("empty-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 549
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("singleton", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(22 as u32)));
    }
    

    // Command line number: 550
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("singleton", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(20 as u32)));
    }
    

    // Command line number: 551
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(11 as u32)];
        let result = runtime.call_exported_function("singleton", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(20 as u32)));
    }
    

    // Command line number: 552
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("singleton", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(20 as u32)));
    }
    

    // Command line number: 553
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967196 as u32)];
        let result = runtime.call_exported_function("singleton", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(20 as u32)));
    }
    

    // Command line number: 554
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("singleton", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(20 as u32)));
    }
    

    // Command line number: 556
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("singleton-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(32 as u32)));
    }
    

    // Command line number: 557
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("singleton-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 558
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(11 as u32)];
        let result = runtime.call_exported_function("singleton-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 559
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("singleton-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 560
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967196 as u32)];
        let result = runtime.call_exported_function("singleton-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 561
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("singleton-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 563
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("multiple", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(103 as u32)));
    }
    

    // Command line number: 564
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("multiple", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(102 as u32)));
    }
    

    // Command line number: 565
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("multiple", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(101 as u32)));
    }
    

    // Command line number: 566
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("multiple", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(100 as u32)));
    }
    

    // Command line number: 567
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32)];
        let result = runtime.call_exported_function("multiple", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(104 as u32)));
    }
    

    // Command line number: 568
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32)];
        let result = runtime.call_exported_function("multiple", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(104 as u32)));
    }
    

    // Command line number: 569
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("multiple", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(104 as u32)));
    }
    

    // Command line number: 570
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(10 as u32)];
        let result = runtime.call_exported_function("multiple", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(104 as u32)));
    }
    

    // Command line number: 571
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("multiple", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(104 as u32)));
    }
    

    // Command line number: 572
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("multiple", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(104 as u32)));
    }
    

    // Command line number: 574
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("multiple-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(213 as u32)));
    }
    

    // Command line number: 575
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("multiple-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(212 as u32)));
    }
    

    // Command line number: 576
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("multiple-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(211 as u32)));
    }
    

    // Command line number: 577
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("multiple-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(210 as u32)));
    }
    

    // Command line number: 578
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32)];
        let result = runtime.call_exported_function("multiple-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(214 as u32)));
    }
    

    // Command line number: 579
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32)];
        let result = runtime.call_exported_function("multiple-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(214 as u32)));
    }
    

    // Command line number: 580
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("multiple-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(214 as u32)));
    }
    

    // Command line number: 581
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(10 as u32)];
        let result = runtime.call_exported_function("multiple-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(214 as u32)));
    }
    

    // Command line number: 582
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("multiple-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(214 as u32)));
    }
    

    // Command line number: 583
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("multiple-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(214 as u32)));
    }
    

    // Command line number: 585
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("large", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 586
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("large", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 587
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("large", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 588
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(11 as u32)];
        let result = runtime.call_exported_function("large", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 589
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(10000 as u32)];
        let result = runtime.call_exported_function("large", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 590
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(10001 as u32)];
        let result = runtime.call_exported_function("large", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 591
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1000000 as u32)];
        let result = runtime.call_exported_function("large", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 592
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1000001 as u32)];
        let result = runtime.call_exported_function("large", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 594
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-block-first", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 595
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-block-mid", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 596
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-block-last", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 597
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-block-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 599
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-loop-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 600
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-loop-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4 as u32)));
    }
    

    // Command line number: 601
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-loop-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(5 as u32)));
    }
    

    // Command line number: 603
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 605
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_if-cond", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 606
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_if-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 607
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_if-value-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 609
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_table-index", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 610
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_table-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(10 as u32)));
    }
    

    // Command line number: 611
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_table-value-index", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(11 as u32)));
    }
    

    // Command line number: 613
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-return-value", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(7 as u64)));
    }
    

    // Command line number: 615
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-if-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 616
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-if-then", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 617
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-if-then", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 618
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-if-else", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4 as u32)));
    }
    

    // Command line number: 619
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-if-else", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 621
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-select-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(5 as u32)));
    }
    

    // Command line number: 622
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-select-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(5 as u32)));
    }
    

    // Command line number: 623
    #[test]
    fn test_82(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-select-second", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 624
    #[test]
    fn test_83(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-select-second", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 625
    #[test]
    fn test_84(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-select-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(7 as u32)));
    }
    

    // Command line number: 627
    #[test]
    fn test_85(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(12 as u32)));
    }
    

    // Command line number: 628
    #[test]
    fn test_86(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(13 as u32)));
    }
    

    // Command line number: 629
    #[test]
    fn test_87(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(14 as u32)));
    }
    

    // Command line number: 631
    #[test]
    fn test_88(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(20 as u32)));
    }
    

    // Command line number: 632
    #[test]
    fn test_89(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(21 as u32)));
    }
    

    // Command line number: 633
    #[test]
    fn test_90(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(22 as u32)));
    }
    

    // Command line number: 634
    #[test]
    fn test_91(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-func", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(23 as u32)));
    }
    

    // Command line number: 636
    #[test]
    fn test_92(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-local.set-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(17 as u32)));
    }
    

    // Command line number: 637
    #[test]
    fn test_93(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-local.tee-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 638
    #[test]
    fn test_94(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-global.set-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 640
    #[test]
    fn test_95(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-load-address", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1071225242 as u32)));
    }
    

    // Command line number: 641
    #[test]
    fn test_96(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-loadN-address", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(30 as u64)));
    }
    

    // Command line number: 643
    #[test]
    fn test_97(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-store-address", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(30 as u32)));
    }
    

    // Command line number: 644
    #[test]
    fn test_98(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-store-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(31 as u32)));
    }
    

    // Command line number: 645
    #[test]
    fn test_99(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-storeN-address", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(32 as u32)));
    }
    

    // Command line number: 646
    #[test]
    fn test_100(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-storeN-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 648
    #[test]
    fn test_101(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-unary-operand", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1079613850 as u32)));
    }
    

    // Command line number: 650
    #[test]
    fn test_102(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-binary-left", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 651
    #[test]
    fn test_103(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-binary-right", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(45 as u64)));
    }
    

    // Command line number: 653
    #[test]
    fn test_104(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-test-operand", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(44 as u32)));
    }
    

    // Command line number: 655
    #[test]
    fn test_105(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-compare-left", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(43 as u32)));
    }
    

    // Command line number: 656
    #[test]
    fn test_106(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-compare-right", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(42 as u32)));
    }
    

    // Command line number: 658
    #[test]
    fn test_107(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-convert-operand", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(41 as u32)));
    }
    

    // Command line number: 660
    #[test]
    fn test_108(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-memory.grow-size", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(40 as u32)));
    }
    

    // Command line number: 662
    #[test]
    fn test_109(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("nested-block-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(19 as u32)));
    }
    

    // Command line number: 663
    #[test]
    fn test_110(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nested-block-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(17 as u32)));
    }
    

    // Command line number: 664
    #[test]
    fn test_111(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("nested-block-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(16 as u32)));
    }
    

    // Command line number: 665
    #[test]
    fn test_112(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(10 as u32)];
        let result = runtime.call_exported_function("nested-block-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(16 as u32)));
    }
    

    // Command line number: 666
    #[test]
    fn test_113(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("nested-block-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(16 as u32)));
    }
    

    // Command line number: 667
    #[test]
    fn test_114(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(100000 as u32)];
        let result = runtime.call_exported_function("nested-block-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(16 as u32)));
    }
    

    // Command line number: 669
    #[test]
    fn test_115(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("nested-br-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 670
    #[test]
    fn test_116(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nested-br-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 671
    #[test]
    fn test_117(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("nested-br-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(17 as u32)));
    }
    

    // Command line number: 672
    #[test]
    fn test_118(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(11 as u32)];
        let result = runtime.call_exported_function("nested-br-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(17 as u32)));
    }
    

    // Command line number: 673
    #[test]
    fn test_119(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967292 as u32)];
        let result = runtime.call_exported_function("nested-br-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(17 as u32)));
    }
    

    // Command line number: 674
    #[test]
    fn test_120(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(10213210 as u32)];
        let result = runtime.call_exported_function("nested-br-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(17 as u32)));
    }
    

    // Command line number: 676
    #[test]
    fn test_121(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(17 as u32)));
    }
    

    // Command line number: 677
    #[test]
    fn test_122(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 678
    #[test]
    fn test_123(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 679
    #[test]
    fn test_124(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(9 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 680
    #[test]
    fn test_125(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967287 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 681
    #[test]
    fn test_126(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(999999 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 683
    #[test]
    fn test_127(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 684
    #[test]
    fn test_128(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 685
    #[test]
    fn test_129(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 686
    #[test]
    fn test_130(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 687
    #[test]
    fn test_131(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4293967296 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 688
    #[test]
    fn test_132(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(9423975 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 690
    #[test]
    fn test_133(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(17 as u32)));
    }
    

    // Command line number: 691
    #[test]
    fn test_134(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 692
    #[test]
    fn test_135(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 693
    #[test]
    fn test_136(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(9 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 694
    #[test]
    fn test_137(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967287 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 695
    #[test]
    fn test_138(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(999999 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 697
    #[test]
    fn test_139(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value-index", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 698
    #[test]
    fn test_140(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value-index", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 699
    #[test]
    fn test_141(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value-index", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 700
    #[test]
    fn test_142(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value-index", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 701
    #[test]
    fn test_143(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4293967296 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value-index", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 702
    #[test]
    fn test_144(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(9423975 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value-index", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 704
    #[test]
    fn test_145(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nested-br_table-loop-block", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    
}

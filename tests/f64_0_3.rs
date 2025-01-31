
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "f64.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 619
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13842132293034192152 as u64)));
    }
    

    // Command line number: 620
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13842132293034192152 as u64)));
    }
    

    // Command line number: 621
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4618760256179416344 as u64)));
    }
    

    // Command line number: 622
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4618760256179416344 as u64)));
    }
    

    // Command line number: 623
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13842132293034192152 as u64)));
    }
    

    // Command line number: 624
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13842132293034192152 as u64)));
    }
    

    // Command line number: 625
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4618760256179416344 as u64)));
    }
    

    // Command line number: 626
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4618760256179416344 as u64)));
    }
    

    // Command line number: 627
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13842132293034192152 as u64)));
    }
    

    // Command line number: 628
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13842132293034192152 as u64)));
    }
    

    // Command line number: 629
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4618760256179416344 as u64)));
    }
    

    // Command line number: 630
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4618760256179416344 as u64)));
    }
    

    // Command line number: 631
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13841569343080770840 as u64)));
    }
    

    // Command line number: 632
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13842695242987613464 as u64)));
    }
    

    // Command line number: 633
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4619323206132837656 as u64)));
    }
    

    // Command line number: 634
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4618197306225995032 as u64)));
    }
    

    // Command line number: 635
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13841006393127349528 as u64)));
    }
    

    // Command line number: 636
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13843258192941034776 as u64)));
    }
    

    // Command line number: 637
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4619886156086258968 as u64)));
    }
    

    // Command line number: 638
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4617634356272573720 as u64)));
    }
    

    // Command line number: 639
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 640
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13846635892661562648 as u64)));
    }
    

    // Command line number: 641
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4623263855806786840 as u64)));
    }
    

    // Command line number: 642
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 643
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 644
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181119 as u64)));
    }
    

    // Command line number: 645
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 646
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181119 as u64)));
    }
    

    // Command line number: 647
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 648
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 649
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 650
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 651
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 652
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 653
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 654
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 655
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 656
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 657
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 658
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 659
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181119 as u64)));
    }
    

    // Command line number: 660
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181119 as u64)));
    }
    

    // Command line number: 661
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 662
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 663
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181119 as u64)));
    }
    

    // Command line number: 664
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181119 as u64)));
    }
    

    // Command line number: 665
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 666
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 667
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181119 as u64)));
    }
    

    // Command line number: 668
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181119 as u64)));
    }
    

    // Command line number: 669
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 670
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 671
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181119 as u64)));
    }
    

    // Command line number: 672
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181119 as u64)));
    }
    

    // Command line number: 673
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 674
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 675
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181119 as u64)));
    }
    

    // Command line number: 676
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181119 as u64)));
    }
    

    // Command line number: 677
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 678
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 679
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181119 as u64)));
    }
    

    // Command line number: 680
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181119 as u64)));
    }
    

    // Command line number: 681
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 682
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 683
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 684
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 685
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 686
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 687
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 688
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 689
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 690
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 691
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 692
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 693
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 694
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 695
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 696
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 697
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 698
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 699
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 700
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 701
    #[test]
    fn test_82(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 702
    #[test]
    fn test_83(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 703
    #[test]
    fn test_84(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 704
    #[test]
    fn test_85(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 705
    #[test]
    fn test_86(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 706
    #[test]
    fn test_87(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 707
    #[test]
    fn test_88(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 708
    #[test]
    fn test_89(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 709
    #[test]
    fn test_90(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 710
    #[test]
    fn test_91(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 711
    #[test]
    fn test_92(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 712
    #[test]
    fn test_93(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 713
    #[test]
    fn test_94(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 714
    #[test]
    fn test_95(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 715
    #[test]
    fn test_96(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 716
    #[test]
    fn test_97(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 717
    #[test]
    fn test_98(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 718
    #[test]
    fn test_99(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 719
    #[test]
    fn test_100(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 720
    #[test]
    fn test_101(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 721
    #[test]
    fn test_102(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 722
    #[test]
    fn test_103(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 723
    #[test]
    fn test_104(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 724
    #[test]
    fn test_105(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 725
    #[test]
    fn test_106(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 726
    #[test]
    fn test_107(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 727
    #[test]
    fn test_108(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 728
    #[test]
    fn test_109(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 729
    #[test]
    fn test_110(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 730
    #[test]
    fn test_111(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 731
    #[test]
    fn test_112(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 732
    #[test]
    fn test_113(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 733
    #[test]
    fn test_114(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 734
    #[test]
    fn test_115(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 735
    #[test]
    fn test_116(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 736
    #[test]
    fn test_117(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 737
    #[test]
    fn test_118(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 738
    #[test]
    fn test_119(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 739
    #[test]
    fn test_120(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 740
    #[test]
    fn test_121(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 741
    #[test]
    fn test_122(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 742
    #[test]
    fn test_123(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 743
    #[test]
    fn test_124(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 744
    #[test]
    fn test_125(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 745
    #[test]
    fn test_126(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 746
    #[test]
    fn test_127(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 747
    #[test]
    fn test_128(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 748
    #[test]
    fn test_129(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 749
    #[test]
    fn test_130(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 750
    #[test]
    fn test_131(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 751
    #[test]
    fn test_132(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 752
    #[test]
    fn test_133(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 753
    #[test]
    fn test_134(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 754
    #[test]
    fn test_135(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 755
    #[test]
    fn test_136(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 756
    #[test]
    fn test_137(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 757
    #[test]
    fn test_138(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 758
    #[test]
    fn test_139(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 759
    #[test]
    fn test_140(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 760
    #[test]
    fn test_141(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 761
    #[test]
    fn test_142(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 762
    #[test]
    fn test_143(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 763
    #[test]
    fn test_144(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 764
    #[test]
    fn test_145(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 765
    #[test]
    fn test_146(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 766
    #[test]
    fn test_147(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 767
    #[test]
    fn test_148(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 768
    #[test]
    fn test_149(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 769
    #[test]
    fn test_150(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 770
    #[test]
    fn test_151(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 771
    #[test]
    fn test_152(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 772
    #[test]
    fn test_153(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 773
    #[test]
    fn test_154(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 774
    #[test]
    fn test_155(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 775
    #[test]
    fn test_156(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 776
    #[test]
    fn test_157(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 777
    #[test]
    fn test_158(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 778
    #[test]
    fn test_159(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 779
    #[test]
    fn test_160(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 780
    #[test]
    fn test_161(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 781
    #[test]
    fn test_162(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 782
    #[test]
    fn test_163(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 783
    #[test]
    fn test_164(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 784
    #[test]
    fn test_165(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 785
    #[test]
    fn test_166(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 786
    #[test]
    fn test_167(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 787
    #[test]
    fn test_168(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 788
    #[test]
    fn test_169(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 789
    #[test]
    fn test_170(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 790
    #[test]
    fn test_171(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 791
    #[test]
    fn test_172(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 792
    #[test]
    fn test_173(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 793
    #[test]
    fn test_174(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 794
    #[test]
    fn test_175(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 795
    #[test]
    fn test_176(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 796
    #[test]
    fn test_177(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 797
    #[test]
    fn test_178(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 798
    #[test]
    fn test_179(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 799
    #[test]
    fn test_180(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 800
    #[test]
    fn test_181(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 801
    #[test]
    fn test_182(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 802
    #[test]
    fn test_183(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 803
    #[test]
    fn test_184(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 804
    #[test]
    fn test_185(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 805
    #[test]
    fn test_186(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 806
    #[test]
    fn test_187(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 807
    #[test]
    fn test_188(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 808
    #[test]
    fn test_189(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 809
    #[test]
    fn test_190(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 810
    #[test]
    fn test_191(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 811
    #[test]
    fn test_192(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 812
    #[test]
    fn test_193(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 813
    #[test]
    fn test_194(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 814
    #[test]
    fn test_195(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 815
    #[test]
    fn test_196(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 816
    #[test]
    fn test_197(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 817
    #[test]
    fn test_198(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 818
    #[test]
    fn test_199(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    
}

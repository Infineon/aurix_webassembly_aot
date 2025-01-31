
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

    
    // Command line number: 1419
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1420
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1421
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1422
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1423
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1424
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1425
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1426
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1427
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1428
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1429
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1430
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1431
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4623263855806786840 as u64)));
    }
    

    // Command line number: 1432
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13846635892661562648 as u64)));
    }
    

    // Command line number: 1433
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13846635892661562648 as u64)));
    }
    

    // Command line number: 1434
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4623263855806786840 as u64)));
    }
    

    // Command line number: 1435
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4618760256179416344 as u64)));
    }
    

    // Command line number: 1436
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13842132293034192152 as u64)));
    }
    

    // Command line number: 1437
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13842132293034192152 as u64)));
    }
    

    // Command line number: 1438
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4618760256179416344 as u64)));
    }
    

    // Command line number: 1439
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 1440
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13830554455654793216 as u64)));
    }
    

    // Command line number: 1441
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13830554455654793216 as u64)));
    }
    

    // Command line number: 1442
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 1443
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(7074237752028441 as u64)));
    }
    

    // Command line number: 1444
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9230446274606804249 as u64)));
    }
    

    // Command line number: 1445
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9230446274606804249 as u64)));
    }
    

    // Command line number: 1446
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(7074237752028441 as u64)));
    }
    

    // Command line number: 1447
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1448
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1449
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1450
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1451
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1452
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1453
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1454
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13842132293034192152 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1455
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1456
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1457
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1458
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4618760256179416344 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1459
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1460
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1461
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1462
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1463
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1464
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1465
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1466
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1467
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1468
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1469
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1470
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1471
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1472
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1473
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1474
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1475
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 1476
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181119 as u64)));
    }
    

    // Command line number: 1477
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181119 as u64)));
    }
    

    // Command line number: 1478
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405311 as u64)));
    }
    

    // Command line number: 1479
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9206588199857145986 as u64)));
    }
    

    // Command line number: 1480
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18429960236711921794 as u64)));
    }
    

    // Command line number: 1481
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18429960236711921794 as u64)));
    }
    

    // Command line number: 1482
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9206588199857145986 as u64)));
    }
    

    // Command line number: 1483
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 1484
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13830554455654793216 as u64)));
    }
    

    // Command line number: 1485
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13830554455654793216 as u64)));
    }
    

    // Command line number: 1486
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 1487
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1488
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1489
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1490
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1491
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1492
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1493
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1494
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181119 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1495
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1496
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1497
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1498
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405311 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1499
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1500
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1501
    #[test]
    fn test_82(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1502
    #[test]
    fn test_83(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1503
    #[test]
    fn test_84(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1504
    #[test]
    fn test_85(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1505
    #[test]
    fn test_86(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1506
    #[test]
    fn test_87(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1507
    #[test]
    fn test_88(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1508
    #[test]
    fn test_89(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1509
    #[test]
    fn test_90(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1510
    #[test]
    fn test_91(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1511
    #[test]
    fn test_92(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1512
    #[test]
    fn test_93(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1513
    #[test]
    fn test_94(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1514
    #[test]
    fn test_95(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1515
    #[test]
    fn test_96(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1516
    #[test]
    fn test_97(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1517
    #[test]
    fn test_98(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1518
    #[test]
    fn test_99(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1519
    #[test]
    fn test_100(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1520
    #[test]
    fn test_101(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1521
    #[test]
    fn test_102(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1522
    #[test]
    fn test_103(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1523
    #[test]
    fn test_104(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1524
    #[test]
    fn test_105(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1525
    #[test]
    fn test_106(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1526
    #[test]
    fn test_107(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1527
    #[test]
    fn test_108(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1528
    #[test]
    fn test_109(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1529
    #[test]
    fn test_110(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1530
    #[test]
    fn test_111(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1531
    #[test]
    fn test_112(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1532
    #[test]
    fn test_113(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1533
    #[test]
    fn test_114(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1534
    #[test]
    fn test_115(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1535
    #[test]
    fn test_116(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1536
    #[test]
    fn test_117(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1537
    #[test]
    fn test_118(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1538
    #[test]
    fn test_119(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1539
    #[test]
    fn test_120(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1540
    #[test]
    fn test_121(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1541
    #[test]
    fn test_122(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1542
    #[test]
    fn test_123(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1543
    #[test]
    fn test_124(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1544
    #[test]
    fn test_125(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1545
    #[test]
    fn test_126(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1546
    #[test]
    fn test_127(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1547
    #[test]
    fn test_128(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1548
    #[test]
    fn test_129(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1549
    #[test]
    fn test_130(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1550
    #[test]
    fn test_131(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1551
    #[test]
    fn test_132(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1552
    #[test]
    fn test_133(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1553
    #[test]
    fn test_134(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1554
    #[test]
    fn test_135(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1555
    #[test]
    fn test_136(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1556
    #[test]
    fn test_137(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1557
    #[test]
    fn test_138(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1558
    #[test]
    fn test_139(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1559
    #[test]
    fn test_140(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1560
    #[test]
    fn test_141(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1561
    #[test]
    fn test_142(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1562
    #[test]
    fn test_143(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1563
    #[test]
    fn test_144(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1564
    #[test]
    fn test_145(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1565
    #[test]
    fn test_146(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1566
    #[test]
    fn test_147(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1567
    #[test]
    fn test_148(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1568
    #[test]
    fn test_149(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1569
    #[test]
    fn test_150(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1570
    #[test]
    fn test_151(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1571
    #[test]
    fn test_152(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1572
    #[test]
    fn test_153(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1573
    #[test]
    fn test_154(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1574
    #[test]
    fn test_155(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1575
    #[test]
    fn test_156(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1576
    #[test]
    fn test_157(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1577
    #[test]
    fn test_158(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1578
    #[test]
    fn test_159(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1579
    #[test]
    fn test_160(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1580
    #[test]
    fn test_161(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1581
    #[test]
    fn test_162(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1582
    #[test]
    fn test_163(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1583
    #[test]
    fn test_164(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1584
    #[test]
    fn test_165(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1585
    #[test]
    fn test_166(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1586
    #[test]
    fn test_167(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1587
    #[test]
    fn test_168(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1588
    #[test]
    fn test_169(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1589
    #[test]
    fn test_170(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1590
    #[test]
    fn test_171(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1591
    #[test]
    fn test_172(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1592
    #[test]
    fn test_173(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1593
    #[test]
    fn test_174(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1594
    #[test]
    fn test_175(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1595
    #[test]
    fn test_176(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1596
    #[test]
    fn test_177(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1597
    #[test]
    fn test_178(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1598
    #[test]
    fn test_179(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1599
    #[test]
    fn test_180(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1600
    #[test]
    fn test_181(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1601
    #[test]
    fn test_182(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1602
    #[test]
    fn test_183(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1603
    #[test]
    fn test_184(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1604
    #[test]
    fn test_185(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1605
    #[test]
    fn test_186(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1606
    #[test]
    fn test_187(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1607
    #[test]
    fn test_188(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1608
    #[test]
    fn test_189(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1609
    #[test]
    fn test_190(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18444492273895866368 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1610
    #[test]
    fn test_191(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18443366373989023744 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1611
    #[test]
    fn test_192(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1612
    #[test]
    fn test_193(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1613
    #[test]
    fn test_194(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1614
    #[test]
    fn test_195(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1615
    #[test]
    fn test_196(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1616
    #[test]
    fn test_197(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1617
    #[test]
    fn test_198(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9221120237041090560 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1618
    #[test]
    fn test_199(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9219994337134247936 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    
}

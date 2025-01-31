
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "f32.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 1356
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 1357
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 1358
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 1359
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1034090883 as u32)));
    }
    

    // Command line number: 1360
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3181574531 as u32)));
    }
    

    // Command line number: 1361
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3181574531 as u32)));
    }
    

    // Command line number: 1362
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1034090883 as u32)));
    }
    

    // Command line number: 1367
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1368
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1369
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1370
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1371
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1372
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1373
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1374
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1375
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1376
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1377
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1378
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1379
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1380
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1381
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1382
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1383
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1384
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1385
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1386
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1387
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2122317824 as u32)));
    }
    

    // Command line number: 1388
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4269801472 as u32)));
    }
    

    // Command line number: 1389
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4269801472 as u32)));
    }
    

    // Command line number: 1390
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2122317824 as u32)));
    }
    

    // Command line number: 1391
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1073741824 as u32)));
    }
    

    // Command line number: 1392
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3221225472 as u32)));
    }
    

    // Command line number: 1393
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3221225472 as u32)));
    }
    

    // Command line number: 1394
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1073741824 as u32)));
    }
    

    // Command line number: 1395
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 1396
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 1397
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 1398
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 1399
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1042479491 as u32)));
    }
    

    // Command line number: 1400
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3189963139 as u32)));
    }
    

    // Command line number: 1401
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3189963139 as u32)));
    }
    

    // Command line number: 1402
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1042479491 as u32)));
    }
    

    // Command line number: 1407
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1408
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1409
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1410
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1411
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1412
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1413
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1414
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1415
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1416
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1417
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1418
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1419
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1420
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1421
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1422
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1423
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1424
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1425
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1426
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1427
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1428
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1429
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1430
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1431
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1095307227 as u32)));
    }
    

    // Command line number: 1432
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3242790875 as u32)));
    }
    

    // Command line number: 1433
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3242790875 as u32)));
    }
    

    // Command line number: 1434
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1095307227 as u32)));
    }
    

    // Command line number: 1435
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 1436
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3234402267 as u32)));
    }
    

    // Command line number: 1437
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3234402267 as u32)));
    }
    

    // Command line number: 1438
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 1439
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 1440
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 1441
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 1442
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 1443
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(13176796 as u32)));
    }
    

    // Command line number: 1444
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2160660444 as u32)));
    }
    

    // Command line number: 1445
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2160660444 as u32)));
    }
    

    // Command line number: 1446
    #[test]
    fn test_82(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(13176796 as u32)));
    }
    

    // Command line number: 1447
    #[test]
    fn test_83(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1448
    #[test]
    fn test_84(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1449
    #[test]
    fn test_85(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1450
    #[test]
    fn test_86(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1451
    #[test]
    fn test_87(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1452
    #[test]
    fn test_88(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1453
    #[test]
    fn test_89(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1454
    #[test]
    fn test_90(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1455
    #[test]
    fn test_91(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1456
    #[test]
    fn test_92(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1457
    #[test]
    fn test_93(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1458
    #[test]
    fn test_94(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1459
    #[test]
    fn test_95(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1460
    #[test]
    fn test_96(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1461
    #[test]
    fn test_97(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1462
    #[test]
    fn test_98(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1463
    #[test]
    fn test_99(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1464
    #[test]
    fn test_100(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1465
    #[test]
    fn test_101(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1466
    #[test]
    fn test_102(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1467
    #[test]
    fn test_103(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1468
    #[test]
    fn test_104(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1469
    #[test]
    fn test_105(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1470
    #[test]
    fn test_106(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1471
    #[test]
    fn test_107(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1472
    #[test]
    fn test_108(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1473
    #[test]
    fn test_109(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1474
    #[test]
    fn test_110(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1475
    #[test]
    fn test_111(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 1476
    #[test]
    fn test_112(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    

    // Command line number: 1477
    #[test]
    fn test_113(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    

    // Command line number: 1478
    #[test]
    fn test_114(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 1479
    #[test]
    fn test_115(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2116221314 as u32)));
    }
    

    // Command line number: 1480
    #[test]
    fn test_116(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4263704962 as u32)));
    }
    

    // Command line number: 1481
    #[test]
    fn test_117(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4263704962 as u32)));
    }
    

    // Command line number: 1482
    #[test]
    fn test_118(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2116221314 as u32)));
    }
    

    // Command line number: 1483
    #[test]
    fn test_119(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 1484
    #[test]
    fn test_120(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 1485
    #[test]
    fn test_121(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 1486
    #[test]
    fn test_122(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 1487
    #[test]
    fn test_123(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1488
    #[test]
    fn test_124(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1489
    #[test]
    fn test_125(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1490
    #[test]
    fn test_126(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1491
    #[test]
    fn test_127(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1492
    #[test]
    fn test_128(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1493
    #[test]
    fn test_129(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1494
    #[test]
    fn test_130(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1495
    #[test]
    fn test_131(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1496
    #[test]
    fn test_132(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1497
    #[test]
    fn test_133(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1498
    #[test]
    fn test_134(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1499
    #[test]
    fn test_135(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1500
    #[test]
    fn test_136(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1501
    #[test]
    fn test_137(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1502
    #[test]
    fn test_138(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1503
    #[test]
    fn test_139(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1504
    #[test]
    fn test_140(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1505
    #[test]
    fn test_141(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1506
    #[test]
    fn test_142(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1507
    #[test]
    fn test_143(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1508
    #[test]
    fn test_144(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1509
    #[test]
    fn test_145(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1510
    #[test]
    fn test_146(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1511
    #[test]
    fn test_147(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1512
    #[test]
    fn test_148(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1513
    #[test]
    fn test_149(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1514
    #[test]
    fn test_150(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1515
    #[test]
    fn test_151(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1516
    #[test]
    fn test_152(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1517
    #[test]
    fn test_153(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1518
    #[test]
    fn test_154(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1519
    #[test]
    fn test_155(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1520
    #[test]
    fn test_156(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1521
    #[test]
    fn test_157(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1522
    #[test]
    fn test_158(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1523
    #[test]
    fn test_159(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1524
    #[test]
    fn test_160(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1525
    #[test]
    fn test_161(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1526
    #[test]
    fn test_162(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1527
    #[test]
    fn test_163(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1528
    #[test]
    fn test_164(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1529
    #[test]
    fn test_165(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1530
    #[test]
    fn test_166(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1531
    #[test]
    fn test_167(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1532
    #[test]
    fn test_168(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1533
    #[test]
    fn test_169(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1534
    #[test]
    fn test_170(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1535
    #[test]
    fn test_171(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1536
    #[test]
    fn test_172(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1537
    #[test]
    fn test_173(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1538
    #[test]
    fn test_174(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1539
    #[test]
    fn test_175(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1540
    #[test]
    fn test_176(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1541
    #[test]
    fn test_177(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1542
    #[test]
    fn test_178(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1543
    #[test]
    fn test_179(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1544
    #[test]
    fn test_180(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1545
    #[test]
    fn test_181(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1546
    #[test]
    fn test_182(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1547
    #[test]
    fn test_183(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1548
    #[test]
    fn test_184(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1549
    #[test]
    fn test_185(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1550
    #[test]
    fn test_186(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1551
    #[test]
    fn test_187(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1552
    #[test]
    fn test_188(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1553
    #[test]
    fn test_189(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1554
    #[test]
    fn test_190(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1555
    #[test]
    fn test_191(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1556
    #[test]
    fn test_192(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1557
    #[test]
    fn test_193(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1558
    #[test]
    fn test_194(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1559
    #[test]
    fn test_195(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1560
    #[test]
    fn test_196(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1561
    #[test]
    fn test_197(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1562
    #[test]
    fn test_198(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1563
    #[test]
    fn test_199(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    
}

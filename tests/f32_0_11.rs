
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

    
    // Command line number: 2364
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2365
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2366
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2367
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2368
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2369
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2370
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2371
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2372
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2373
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2374
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2375
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2376
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2377
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2378
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2379
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2380
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2381
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2382
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2383
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2384
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2385
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2386
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2387
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2388
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2389
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2390
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2391
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2392
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2393
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2394
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2395
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2396
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2397
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2398
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2399
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2400
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2401
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2402
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2403
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2404
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2405
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2406
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2407
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2408
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2409
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2410
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2411
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2412
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2413
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2414
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2415
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2416
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2417
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2418
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2419
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2420
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2421
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("sqrt", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2422
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(439682291 as u32)));
    }
    

    // Command line number: 2423
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("sqrt", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2424
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(536870912 as u32)));
    }
    

    // Command line number: 2425
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("sqrt", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2426
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1060439283 as u32)));
    }
    

    // Command line number: 2427
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("sqrt", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2428
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2429
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("sqrt", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2430
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1075866777 as u32)));
    }
    

    // Command line number: 2431
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("sqrt", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2432
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1602224127 as u32)));
    }
    

    // Command line number: 2433
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("sqrt", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2434
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2435
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("sqrt", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2436
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("sqrt", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2437
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("sqrt", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2438
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("sqrt", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2439
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2440
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2441
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 2442
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2443
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 2444
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2445
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 2446
    #[test]
    fn test_82(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2447
    #[test]
    fn test_83(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 2448
    #[test]
    fn test_84(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2449
    #[test]
    fn test_85(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3235905536 as u32)));
    }
    

    // Command line number: 2450
    #[test]
    fn test_86(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086324736 as u32)));
    }
    

    // Command line number: 2451
    #[test]
    fn test_87(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    

    // Command line number: 2452
    #[test]
    fn test_88(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2453
    #[test]
    fn test_89(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 2454
    #[test]
    fn test_90(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2455
    #[test]
    fn test_91(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("floor", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2456
    #[test]
    fn test_92(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("floor", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2457
    #[test]
    fn test_93(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("floor", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2458
    #[test]
    fn test_94(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("floor", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2459
    #[test]
    fn test_95(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2460
    #[test]
    fn test_96(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2461
    #[test]
    fn test_97(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2462
    #[test]
    fn test_98(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2463
    #[test]
    fn test_99(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2464
    #[test]
    fn test_100(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2465
    #[test]
    fn test_101(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2466
    #[test]
    fn test_102(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2467
    #[test]
    fn test_103(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 2468
    #[test]
    fn test_104(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2469
    #[test]
    fn test_105(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3233808384 as u32)));
    }
    

    // Command line number: 2470
    #[test]
    fn test_106(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1088421888 as u32)));
    }
    

    // Command line number: 2471
    #[test]
    fn test_107(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    

    // Command line number: 2472
    #[test]
    fn test_108(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2473
    #[test]
    fn test_109(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 2474
    #[test]
    fn test_110(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2475
    #[test]
    fn test_111(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("ceil", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2476
    #[test]
    fn test_112(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("ceil", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2477
    #[test]
    fn test_113(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("ceil", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2478
    #[test]
    fn test_114(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("ceil", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2479
    #[test]
    fn test_115(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("trunc", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2480
    #[test]
    fn test_116(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("trunc", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2481
    #[test]
    fn test_117(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("trunc", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2482
    #[test]
    fn test_118(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("trunc", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2483
    #[test]
    fn test_119(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("trunc", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2484
    #[test]
    fn test_120(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("trunc", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2485
    #[test]
    fn test_121(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("trunc", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2486
    #[test]
    fn test_122(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("trunc", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2487
    #[test]
    fn test_123(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("trunc", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 2488
    #[test]
    fn test_124(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("trunc", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2489
    #[test]
    fn test_125(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("trunc", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3233808384 as u32)));
    }
    

    // Command line number: 2490
    #[test]
    fn test_126(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("trunc", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086324736 as u32)));
    }
    

    // Command line number: 2491
    #[test]
    fn test_127(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("trunc", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    

    // Command line number: 2492
    #[test]
    fn test_128(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("trunc", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2493
    #[test]
    fn test_129(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("trunc", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 2494
    #[test]
    fn test_130(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("trunc", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2495
    #[test]
    fn test_131(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("trunc", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2496
    #[test]
    fn test_132(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("trunc", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2497
    #[test]
    fn test_133(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("trunc", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2498
    #[test]
    fn test_134(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("trunc", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2499
    #[test]
    fn test_135(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2500
    #[test]
    fn test_136(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2501
    #[test]
    fn test_137(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2502
    #[test]
    fn test_138(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2503
    #[test]
    fn test_139(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2504
    #[test]
    fn test_140(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2505
    #[test]
    fn test_141(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2506
    #[test]
    fn test_142(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2507
    #[test]
    fn test_143(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 2508
    #[test]
    fn test_144(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2509
    #[test]
    fn test_145(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3233808384 as u32)));
    }
    

    // Command line number: 2510
    #[test]
    fn test_146(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086324736 as u32)));
    }
    

    // Command line number: 2511
    #[test]
    fn test_147(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    

    // Command line number: 2512
    #[test]
    fn test_148(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2513
    #[test]
    fn test_149(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 2514
    #[test]
    fn test_150(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2515
    #[test]
    fn test_151(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("nearest", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2516
    #[test]
    fn test_152(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("nearest", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2517
    #[test]
    fn test_153(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("nearest", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2518
    #[test]
    fn test_154(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("nearest", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    
}

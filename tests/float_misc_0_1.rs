
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_misc.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 345
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13839181621982597923 as u64),Immediate::DoubleWord(4426983905639375259 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13658985409477228135 as u64)));
    }
    

    // Command line number: 346
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2931365221683549507 as u64),Immediate::DoubleWord(3687257801017498184 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2011503789135851990 as u64)));
    }
    

    // Command line number: 347
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14180592859625794472 as u64),Immediate::DoubleWord(3043918530412552819 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(12617399980269247921 as u64)));
    }
    

    // Command line number: 350
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(436207616 as u32),Immediate::Word(436207616 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 352
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2186111271319845836 as u64),Immediate::DoubleWord(2186111271319845836 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 353
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2186111271319845837 as u64),Immediate::DoubleWord(2186111271319845837 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 356
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1602224127 as u32),Immediate::Word(1602224127 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095038 as u32)));
    }
    

    // Command line number: 357
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1602224128 as u32),Immediate::Word(1602224128 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 358
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6913025428013711359 as u64),Immediate::DoubleWord(6913025428013711359 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405310 as u64)));
    }
    

    // Command line number: 359
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6913025428013711360 as u64),Immediate::DoubleWord(6913025428013711360 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 362
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353217 as u32),Immediate::Word(1065353217 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353218 as u32)));
    }
    

    // Command line number: 363
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353215 as u32),Immediate::Word(1065353215 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353214 as u32)));
    }
    

    // Command line number: 364
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017409 as u64),Immediate::DoubleWord(4607182418800017409 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017410 as u64)));
    }
    

    // Command line number: 365
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017407 as u64),Immediate::DoubleWord(4607182418800017407 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017406 as u64)));
    }
    

    // Command line number: 368
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353217 as u32),Immediate::Word(1065353215 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 369
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353218 as u32),Immediate::Word(1065353214 as u32)];
        let result = runtime.call_exported_function("f32.mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353217 as u32)));
    }
    

    // Command line number: 370
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017409 as u64),Immediate::DoubleWord(4607182418800017407 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 371
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017410 as u64),Immediate::DoubleWord(4607182418800017406 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017409 as u64)));
    }
    

    // Command line number: 376
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(4372995238176751616 as u64)];
        let result = runtime.call_exported_function("f64.mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 382
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1066388847 as u32),Immediate::Word(1120403456 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1010307378 as u32)));
    }
    

    // Command line number: 383
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1258294259 as u32),Immediate::Word(1262485497 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1059764896 as u32)));
    }
    

    // Command line number: 384
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1199570944 as u32),Immediate::Word(754974720 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1509949440 as u32)));
    }
    

    // Command line number: 386
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1082130432 as u32),Immediate::Word(1077936128 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1068149419 as u32)));
    }
    

    // Command line number: 387
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607738418748954166 as u64),Immediate::DoubleWord(4636737291354636288 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4577629909238726725 as u64)));
    }
    

    // Command line number: 388
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4710766852517658624 as u64),Immediate::DoubleWord(4713017006285127680 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4604182212107680295 as u64)));
    }
    

    // Command line number: 389
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4679240012837945344 as u64),Immediate::DoubleWord(4440549232587309056 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4845873199050653696 as u64)));
    }
    

    // Command line number: 390
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4611065853604003840 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2096758607839232 as u64)));
    }
    

    // Command line number: 391
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4616189618054758400 as u64),Immediate::DoubleWord(4613937818241073152 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4608683618675807573 as u64)));
    }
    

    // Command line number: 395
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1249905654 as u32),Immediate::Word(1245708284 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1068153505 as u32)));
    }
    

    // Command line number: 396
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4706263254500900864 as u64),Immediate::DoubleWord(4704009808640999424 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4608685812450356035 as u64)));
    }
    

    // Command line number: 399
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(666187309 as u32),Immediate::Word(2138899896 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 400
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(338737113 as u32),Immediate::Word(1637454009 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 401
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3351999420 as u32),Immediate::Word(401966635 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4015268357 as u32)));
    }
    

    // Command line number: 402
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2582720800 as u32),Immediate::Word(2197197239 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1450863298 as u32)));
    }
    

    // Command line number: 403
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3564182439 as u32),Immediate::Word(397999726 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4230402947 as u32)));
    }
    

    // Command line number: 404
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5269602717919885505 as u64),Immediate::DoubleWord(5615431061534361830 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4261119458085367075 as u64)));
    }
    

    // Command line number: 405
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1791703022234099881 as u64),Immediate::DoubleWord(230587289790163684 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(6167818452243445017 as u64)));
    }
    

    // Command line number: 406
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(15176390632865983384 as u64),Immediate::DoubleWord(7489729865403831125 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(12293662509603441500 as u64)));
    }
    

    // Command line number: 407
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(16945752639368638170 as u64),Immediate::DoubleWord(5718505335991307775 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(15834399302320062011 as u64)));
    }
    

    // Command line number: 408
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(16263708435887736913 as u64),Immediate::DoubleWord(6648866667391375614 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14221856927329063502 as u64)));
    }
    

    // Command line number: 411
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3798033061 as u32),Immediate::Word(2491443361 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 412
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(686927199 as u32),Immediate::Word(1440960248 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(310908762 as u32)));
    }
    

    // Command line number: 413
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1067646869 as u32),Immediate::Word(3960423338 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2467406247 as u32)));
    }
    

    // Command line number: 415
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1824271560 as u32),Immediate::Word(1797890210 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1091436270 as u32)));
    }
    

    // Command line number: 416
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4278321332874065 as u64),Immediate::DoubleWord(15464375907975098426 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 417
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9385779280320516508 as u64),Immediate::DoubleWord(2195352144435258723 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(11796946241575527184 as u64)));
    }
    

    // Command line number: 418
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(12019785500442997560 as u64),Immediate::DoubleWord(14033717547286793792 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2592881844429368931 as u64)));
    }
    

    // Command line number: 419
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14192622858179995755 as u64),Immediate::DoubleWord(1048270709368415171 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(17751311910175316838 as u64)));
    }
    

    // Command line number: 420
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14895971708944847415 as u64),Immediate::DoubleWord(14082502105592202184 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5420603229995464966 as u64)));
    }
    

    // Command line number: 423
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1286756690 as u32),Immediate::Word(74754985 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 424
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3861505243 as u32),Immediate::Word(1298875600 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3627605330 as u32)));
    }
    

    // Command line number: 425
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1387472197 as u32),Immediate::Word(4021087697 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2726599162 as u32)));
    }
    

    // Command line number: 426
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4149014653 as u32),Immediate::Word(3294714580 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1919471347 as u32)));
    }
    

    // Command line number: 427
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3987554477 as u32),Immediate::Word(3327836421 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1724563548 as u32)));
    }
    

    // Command line number: 428
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(10882983485852580843 as u64),Immediate::DoubleWord(1499380901194126188 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13990715555071506017 as u64)));
    }
    

    // Command line number: 429
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(17353632673902705731 as u64),Immediate::DoubleWord(11343375610449883809 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 430
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(10471640091267549911 as u64),Immediate::DoubleWord(4317563730789156718 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10760831595995331201 as u64)));
    }
    

    // Command line number: 431
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(15262578458239868670 as u64),Immediate::DoubleWord(11184694400998603321 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(8684356686754640262 as u64)));
    }
    

    // Command line number: 432
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5423766533942924987 as u64),Immediate::DoubleWord(14797877595923399406 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13679565210380085385 as u64)));
    }
    

    // Command line number: 435
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(16467910284400317286 as u64),Immediate::DoubleWord(14710955777971283471 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(6363956244686200611 as u64)));
    }
    

    // Command line number: 436
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(16290965053652735591 as u64),Immediate::DoubleWord(15317706837775392625 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5580355845867636249 as u64)));
    }
    

    // Command line number: 437
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5440366292977110369 as u64),Immediate::DoubleWord(18177315027663270891 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10316437533262127859 as u64)));
    }
    

    // Command line number: 438
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5586144947709395659 as u64),Immediate::DoubleWord(6532436425888624877 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3660708720660625999 as u64)));
    }
    

    // Command line number: 439
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(17550005606375580620 as u64),Immediate::DoubleWord(6180045224129992598 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(15976575755200563719 as u64)));
    }
    

    // Command line number: 442
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4611065853588275309 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2096758603907099 as u64)));
    }
    

    // Command line number: 443
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(58005932411648970 as u64),Immediate::DoubleWord(4662638783615926141 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3222112482593593 as u64)));
    }
    

    // Command line number: 444
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13185764271695251812 as u64),Immediate::DoubleWord(17795032469661562376 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1594123367530375 as u64)));
    }
    

    // Command line number: 445
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(12062544014359965409 as u64),Immediate::DoubleWord(16674780236028867992 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1036869663772871 as u64)));
    }
    

    // Command line number: 446
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(752499975442349235 as u64),Immediate::DoubleWord(5356690663768240614 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3442089173194313 as u64)));
    }
    

    // Command line number: 447
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(83041154348962762 as u64),Immediate::DoubleWord(13934889252531101931 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223458798833500621 as u64)));
    }
    

    // Command line number: 450
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1817629909 as u32),Immediate::Word(1421132838 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1461198615 as u32)));
    }
    

    // Command line number: 451
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1824604388 as u32),Immediate::Word(4105258014 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3079375694 as u32)));
    }
    

    // Command line number: 452
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1747858929 as u32),Immediate::Word(1063784191 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1749036825 as u32)));
    }
    

    // Command line number: 453
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3378374953 as u32),Immediate::Word(3497907368 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(945802923 as u32)));
    }
    

    // Command line number: 454
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2861015275 as u32),Immediate::Word(3684669151 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(240750700 as u32)));
    }
    

    // Command line number: 455
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4885037015672297625 as u64),Immediate::DoubleWord(9954263091431620051 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(17984289216286432053 as u64)));
    }
    

    // Command line number: 456
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4022347395424227722 as u64),Immediate::DoubleWord(11486191764643320419 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(15589554694846453438 as u64)));
    }
    

    // Command line number: 457
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(7623973190248181230 as u64),Immediate::DoubleWord(7481900282945497881 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4748664889769504925 as u64)));
    }
    

    // Command line number: 458
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14650287530633645126 as u64),Immediate::DoubleWord(8091500841285620267 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(11165393291297270552 as u64)));
    }
    

    // Command line number: 459
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9910334119372230161 as u64),Immediate::DoubleWord(10921408834055405722 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3596021921286071170 as u64)));
    }
    

    // Command line number: 462
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4352764235752579571 as u64),Immediate::DoubleWord(4610853254537913145 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4348952422448611708 as u64)));
    }
    

    // Command line number: 463
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4148344307949876337 as u64),Immediate::DoubleWord(13642564096968604240 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13559333871418047670 as u64)));
    }
    

    // Command line number: 464
    #[test]
    fn test_82(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4425554483259831683 as u64),Immediate::DoubleWord(4103199251532205583 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4929315895081747238 as u64)));
    }
    

    // Command line number: 465
    #[test]
    fn test_83(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5011653323436109419 as u64),Immediate::DoubleWord(5044067306150920275 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4574739988274663188 as u64)));
    }
    

    // Command line number: 466
    #[test]
    fn test_84(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4558988817252519457 as u64),Immediate::DoubleWord(14066629987516208597 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13546063418479940250 as u64)));
    }
    

    // Command line number: 471
    #[test]
    fn test_85(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(4503599627370495 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017409 as u64)));
    }
    

    // Command line number: 472
    #[test]
    fn test_86(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370495 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017406 as u64)));
    }
    

    // Command line number: 475
    #[test]
    fn test_87(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(880803839 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 477
    #[test]
    fn test_88(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4377498837804122111 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 478
    #[test]
    fn test_89(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4377498837804122112 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 481
    #[test]
    fn test_90(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2097152 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 483
    #[test]
    fn test_91(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(1125899906842624 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 484
    #[test]
    fn test_92(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(1125899906842625 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405304 as u64)));
    }
    

    // Command line number: 488
    #[test]
    fn test_93(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2122317824 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 489
    #[test]
    fn test_94(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(9209861237972664321 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4503599627370495 as u64)));
    }
    

    // Command line number: 490
    #[test]
    fn test_95(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(9209861237972664320 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4503599627370496 as u64)));
    }
    

    // Command line number: 500
    #[test]
    fn test_96(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(1077936128 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1051372203 as u32)));
    }
    

    // Command line number: 501
    #[test]
    fn test_97(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1077936128 as u32),Immediate::Word(1091567616 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1051372203 as u32)));
    }
    

    // Command line number: 502
    #[test]
    fn test_98(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1091567616 as u32),Immediate::Word(1104674816 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1051372203 as u32)));
    }
    

    // Command line number: 503
    #[test]
    fn test_99(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(4613937818241073152 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4599676419421066581 as u64)));
    }
    

    // Command line number: 504
    #[test]
    fn test_100(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4613937818241073152 as u64),Immediate::DoubleWord(4621256167635550208 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4599676419421066581 as u64)));
    }
    

    // Command line number: 505
    #[test]
    fn test_101(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4621256167635550208 as u64),Immediate::DoubleWord(4628293042053316608 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4599676419421066581 as u64)));
    }
    

    // Command line number: 508
    #[test]
    fn test_102(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353217 as u32),Immediate::Word(1065353215 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353218 as u32)));
    }
    

    // Command line number: 509
    #[test]
    fn test_103(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353215 as u32),Immediate::Word(1065353217 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353213 as u32)));
    }
    

    // Command line number: 510
    #[test]
    fn test_104(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(1065353215 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353217 as u32)));
    }
    

    // Command line number: 511
    #[test]
    fn test_105(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(1065353217 as u32)];
        let result = runtime.call_exported_function("f32.div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353214 as u32)));
    }
    

    // Command line number: 512
    #[test]
    fn test_106(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017409 as u64),Immediate::DoubleWord(4607182418800017407 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017410 as u64)));
    }
    

    // Command line number: 513
    #[test]
    fn test_107(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017407 as u64),Immediate::DoubleWord(4607182418800017409 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017405 as u64)));
    }
    

    // Command line number: 514
    #[test]
    fn test_108(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(4607182418800017407 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017409 as u64)));
    }
    

    // Command line number: 515
    #[test]
    fn test_109(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(4607182418800017409 as u64)];
        let result = runtime.call_exported_function("f64.div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017406 as u64)));
    }
    

    // Command line number: 519
    #[test]
    fn test_110(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1126891520 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1095842342 as u32)));
    }
    

    // Command line number: 520
    #[test]
    fn test_111(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(875341566 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(970079310 as u32)));
    }
    

    // Command line number: 521
    #[test]
    fn test_112(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4640220544191430656 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4623551143926461685 as u64)));
    }
    

    // Command line number: 522
    #[test]
    fn test_113(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4505170691092029701 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4556032630010904473 as u64)));
    }
    

    // Command line number: 525
    #[test]
    fn test_114(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(3868634135368364633 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4237908228221851551 as u64)));
    }
    

    // Command line number: 526
    #[test]
    fn test_115(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6130334638238385213 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5368736472460186647 as u64)));
    }
    

    // Command line number: 527
    #[test]
    fn test_116(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6420749722515481850 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5513761933181195411 as u64)));
    }
    

    // Command line number: 528
    #[test]
    fn test_117(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4441983858611369811 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4524423892028972029 as u64)));
    }
    

    // Command line number: 529
    #[test]
    fn test_118(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2943222801950574478 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3775077603066051757 as u64)));
    }
    

    // Command line number: 533
    #[test]
    fn test_119(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017407 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017407 as u64)));
    }
    

    // Command line number: 536
    #[test]
    fn test_120(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1040498738 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1052268824 as u32)));
    }
    

    // Command line number: 537
    #[test]
    fn test_121(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1911349112 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1487806505 as u32)));
    }
    

    // Command line number: 538
    #[test]
    fn test_122(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1033984731 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1049605767 as u32)));
    }
    

    // Command line number: 539
    #[test]
    fn test_123(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(471548520 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(768040305 as u32)));
    }
    

    // Command line number: 540
    #[test]
    fn test_124(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(980180519 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1022754037 as u32)));
    }
    

    // Command line number: 541
    #[test]
    fn test_125(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(447478826231748356 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2527268346316778217 as u64)));
    }
    

    // Command line number: 542
    #[test]
    fn test_126(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(7882765141420635228 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(6244818117372907505 as u64)));
    }
    

    // Command line number: 543
    #[test]
    fn test_127(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6345006020483303032 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5476089630784211276 as u64)));
    }
    

    // Command line number: 544
    #[test]
    fn test_128(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5080974719163899092 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4843851732197030563 as u64)));
    }
    

    // Command line number: 545
    #[test]
    fn test_129(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9108179429173493573 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(6857570252025178944 as u64)));
    }
    

    // Command line number: 548
    #[test]
    fn test_130(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1293112087 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1178743848 as u32)));
    }
    

    // Command line number: 549
    #[test]
    fn test_131(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(118173697 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(591184817 as u32)));
    }
    

    // Command line number: 550
    #[test]
    fn test_132(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(941555112 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1003077404 as u32)));
    }
    

    // Command line number: 551
    #[test]
    fn test_133(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(60323222 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(562803203 as u32)));
    }
    

    // Command line number: 552
    #[test]
    fn test_134(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(40220718 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(552763653 as u32)));
    }
    

    // Command line number: 553
    #[test]
    fn test_135(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(7741365502393472048 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(6174272493021072850 as u64)));
    }
    

    // Command line number: 554
    #[test]
    fn test_136(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1175297548666390979 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2891239701856850373 as u64)));
    }
    

    // Command line number: 555
    #[test]
    fn test_137(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4474969389470739153 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4540897185465678583 as u64)));
    }
    

    // Command line number: 556
    #[test]
    fn test_138(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(3521792205891300764 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4064104284248214854 as u64)));
    }
    

    // Command line number: 557
    #[test]
    fn test_139(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(8798159938601760820 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(6702520650372831162 as u64)));
    }
    

    // Command line number: 560
    #[test]
    fn test_140(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2058533950 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1561805138 as u32)));
    }
    

    // Command line number: 561
    #[test]
    fn test_141(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1194962978 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1129973536 as u32)));
    }
    

    // Command line number: 562
    #[test]
    fn test_142(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1062209384 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1063699830 as u32)));
    }
    

    // Command line number: 563
    #[test]
    fn test_143(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(339551808 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(702287090 as u32)));
    }
    

    // Command line number: 564
    #[test]
    fn test_144(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(279876202 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(672496559 as u32)));
    }
    

    // Command line number: 565
    #[test]
    fn test_145(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(7933654283200418420 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(6270248298536475417 as u64)));
    }
    

    // Command line number: 566
    #[test]
    fn test_146(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2893298197477532149 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3750137482611732493 as u64)));
    }
    

    // Command line number: 567
    #[test]
    fn test_147(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(8954101765007123282 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(6780419764181964566 as u64)));
    }
    

    // Command line number: 568
    #[test]
    fn test_148(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4405641787023763989 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4506213966714545194 as u64)));
    }
    

    // Command line number: 569
    #[test]
    fn test_149(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2394824404681191749 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3500764341454448999 as u64)));
    }
    

    // Command line number: 572
    #[test]
    fn test_150(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9494538992465685200 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 573
    #[test]
    fn test_151(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5074882158241187075 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4841025724322244342 as u64)));
    }
    

    // Command line number: 574
    #[test]
    fn test_152(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1187437724299488295 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2897120674751402313 as u64)));
    }
    

    // Command line number: 575
    #[test]
    fn test_153(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(3368223178643061438 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3987382964717611901 as u64)));
    }
    

    // Command line number: 576
    #[test]
    fn test_154(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(7161993531047854177 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5884408968142469554 as u64)));
    }
    

    // Command line number: 579
    #[test]
    fn test_155(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353217 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 580
    #[test]
    fn test_156(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353218 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353217 as u32)));
    }
    

    // Command line number: 581
    #[test]
    fn test_157(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017409 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 582
    #[test]
    fn test_158(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017410 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017409 as u64)));
    }
    

    // Command line number: 585
    #[test]
    fn test_159(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353214 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353215 as u32)));
    }
    

    // Command line number: 586
    #[test]
    fn test_160(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353213 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353214 as u32)));
    }
    

    // Command line number: 587
    #[test]
    fn test_161(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017406 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017407 as u64)));
    }
    

    // Command line number: 588
    #[test]
    fn test_162(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017405 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017406 as u64)));
    }
    

    // Command line number: 592
    #[test]
    fn test_163(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139156962 as u32)];
        let result = runtime.call_exported_function("f32.abs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139156962 as u32)));
    }
    

    // Command line number: 593
    #[test]
    fn test_164(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286640610 as u32)];
        let result = runtime.call_exported_function("f32.abs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139156962 as u32)));
    }
    

    // Command line number: 594
    #[test]
    fn test_165(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868441285556843 as u64)];
        let result = runtime.call_exported_function("f64.abs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868441285556843 as u64)));
    }
    

    // Command line number: 595
    #[test]
    fn test_166(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240478140332651 as u64)];
        let result = runtime.call_exported_function("f64.abs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868441285556843 as u64)));
    }
    

    // Command line number: 597
    #[test]
    fn test_167(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139156962 as u32)];
        let result = runtime.call_exported_function("f32.neg", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286640610 as u32)));
    }
    

    // Command line number: 598
    #[test]
    fn test_168(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286640610 as u32)];
        let result = runtime.call_exported_function("f32.neg", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139156962 as u32)));
    }
    

    // Command line number: 599
    #[test]
    fn test_169(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868441285556843 as u64)];
        let result = runtime.call_exported_function("f64.neg", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240478140332651 as u64)));
    }
    

    // Command line number: 600
    #[test]
    fn test_170(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240478140332651 as u64)];
        let result = runtime.call_exported_function("f64.neg", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868441285556843 as u64)));
    }
    

    // Command line number: 602
    #[test]
    fn test_171(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139156962 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("f32.copysign", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139156962 as u32)));
    }
    

    // Command line number: 603
    #[test]
    fn test_172(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139156962 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("f32.copysign", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286640610 as u32)));
    }
    

    // Command line number: 604
    #[test]
    fn test_173(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286640610 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("f32.copysign", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139156962 as u32)));
    }
    

    // Command line number: 605
    #[test]
    fn test_174(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286640610 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("f32.copysign", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286640610 as u32)));
    }
    

    // Command line number: 606
    #[test]
    fn test_175(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868441285556843 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("f64.copysign", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868441285556843 as u64)));
    }
    

    // Command line number: 607
    #[test]
    fn test_176(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868441285556843 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("f64.copysign", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240478140332651 as u64)));
    }
    

    // Command line number: 608
    #[test]
    fn test_177(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240478140332651 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("f64.copysign", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868441285556843 as u64)));
    }
    

    // Command line number: 609
    #[test]
    fn test_178(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240478140332651 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("f64.copysign", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240478140332651 as u64)));
    }
    

    // Command line number: 612
    #[test]
    fn test_179(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353215 as u32)];
        let result = runtime.call_exported_function("f32.ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 613
    #[test]
    fn test_180(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353217 as u32)];
        let result = runtime.call_exported_function("f32.ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1073741824 as u32)));
    }
    

    // Command line number: 614
    #[test]
    fn test_181(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017407 as u64)];
        let result = runtime.call_exported_function("f64.ceil", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 615
    #[test]
    fn test_182(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017409 as u64)];
        let result = runtime.call_exported_function("f64.ceil", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4611686018427387904 as u64)));
    }
    

    // Command line number: 618
    #[test]
    fn test_183(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1258291199 as u32)];
        let result = runtime.call_exported_function("f32.ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1258291200 as u32)));
    }
    

    // Command line number: 619
    #[test]
    fn test_184(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3405774847 as u32)];
        let result = runtime.call_exported_function("f32.ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3405774846 as u32)));
    }
    

    // Command line number: 620
    #[test]
    fn test_185(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4841369599423283199 as u64)];
        let result = runtime.call_exported_function("f64.ceil", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4841369599423283200 as u64)));
    }
    

    // Command line number: 621
    #[test]
    fn test_186(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14064741636278059007 as u64)];
        let result = runtime.call_exported_function("f64.ceil", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14064741636278059006 as u64)));
    }
    

    // Command line number: 625
    #[test]
    fn test_187(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1266679807 as u32)];
        let result = runtime.call_exported_function("f32.ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1266679807 as u32)));
    }
    

    // Command line number: 626
    #[test]
    fn test_188(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3414163455 as u32)];
        let result = runtime.call_exported_function("f32.ceil", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3414163455 as u32)));
    }
    

    // Command line number: 627
    #[test]
    fn test_189(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4845873199050653695 as u64)];
        let result = runtime.call_exported_function("f64.ceil", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4845873199050653695 as u64)));
    }
    

    // Command line number: 628
    #[test]
    fn test_190(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14069245235905429503 as u64)];
        let result = runtime.call_exported_function("f64.ceil", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14069245235905429503 as u64)));
    }
    

    // Command line number: 631
    #[test]
    fn test_191(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836863 as u32)];
        let result = runtime.call_exported_function("f32.floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 632
    #[test]
    fn test_192(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836865 as u32)];
        let result = runtime.call_exported_function("f32.floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3221225472 as u32)));
    }
    

    // Command line number: 633
    #[test]
    fn test_193(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793215 as u64)];
        let result = runtime.call_exported_function("f64.floor", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13830554455654793216 as u64)));
    }
    

    // Command line number: 634
    #[test]
    fn test_194(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793217 as u64)];
        let result = runtime.call_exported_function("f64.floor", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13835058055282163712 as u64)));
    }
    

    // Command line number: 637
    #[test]
    fn test_195(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3405774847 as u32)];
        let result = runtime.call_exported_function("f32.floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3405774848 as u32)));
    }
    

    // Command line number: 638
    #[test]
    fn test_196(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1258291199 as u32)];
        let result = runtime.call_exported_function("f32.floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1258291198 as u32)));
    }
    

    // Command line number: 639
    #[test]
    fn test_197(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14064741636278059007 as u64)];
        let result = runtime.call_exported_function("f64.floor", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14064741636278059008 as u64)));
    }
    

    // Command line number: 640
    #[test]
    fn test_198(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4841369599423283199 as u64)];
        let result = runtime.call_exported_function("f64.floor", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4841369599423283198 as u64)));
    }
    

    // Command line number: 644
    #[test]
    fn test_199(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1202524032 as u32)];
        let result = runtime.call_exported_function("f32.floor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1202524032 as u32)));
    }
    
}

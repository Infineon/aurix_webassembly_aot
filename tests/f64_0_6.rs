
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

    
    // Command line number: 1219
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1220
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1221
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1222
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1223
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1224
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1225
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1226
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1227
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1228
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1229
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1230
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1231
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1232
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1233
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1234
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1235
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1236
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1237
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1238
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1239
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1240
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1241
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1242
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1243
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1244
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1245
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1246
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1247
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1248
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1249
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1250
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1251
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1252
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1253
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1254
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1255
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1256
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1257
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1258
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1259
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1260
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1261
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1262
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1263
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 1264
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13830554455654793216 as u64)));
    }
    

    // Command line number: 1265
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13830554455654793216 as u64)));
    }
    

    // Command line number: 1266
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 1267
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4372995238176751616 as u64)));
    }
    

    // Command line number: 1268
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13596367275031527424 as u64)));
    }
    

    // Command line number: 1269
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13596367275031527424 as u64)));
    }
    

    // Command line number: 1270
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4372995238176751616 as u64)));
    }
    

    // Command line number: 1271
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2 as u64)));
    }
    

    // Command line number: 1272
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775810 as u64)));
    }
    

    // Command line number: 1273
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775810 as u64)));
    }
    

    // Command line number: 1274
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2 as u64)));
    }
    

    // Command line number: 1275
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 1276
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775809 as u64)));
    }
    

    // Command line number: 1277
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775809 as u64)));
    }
    

    // Command line number: 1278
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 1279
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1280
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1281
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1282
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1283
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1284
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1285
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1286
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1287
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1288
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1289
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1290
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1291
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1292
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1293
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1294
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1295
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1296
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1297
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1298
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1299
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9227875636482146304 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1300
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9227875636482146304 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1301
    #[test]
    fn test_82(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1302
    #[test]
    fn test_83(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1303
    #[test]
    fn test_84(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9227875636482146304 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4841369599423283200 as u64)));
    }
    

    // Command line number: 1304
    #[test]
    fn test_85(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9227875636482146304 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14064741636278059008 as u64)));
    }
    

    // Command line number: 1305
    #[test]
    fn test_86(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14064741636278059008 as u64)));
    }
    

    // Command line number: 1306
    #[test]
    fn test_87(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4841369599423283200 as u64)));
    }
    

    // Command line number: 1307
    #[test]
    fn test_88(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9227875636482146304 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 1308
    #[test]
    fn test_89(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9227875636482146304 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13830554455654793216 as u64)));
    }
    

    // Command line number: 1309
    #[test]
    fn test_90(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13830554455654793216 as u64)));
    }
    

    // Command line number: 1310
    #[test]
    fn test_91(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 1311
    #[test]
    fn test_92(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9227875636482146304 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9007199254740992 as u64)));
    }
    

    // Command line number: 1312
    #[test]
    fn test_93(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9227875636482146304 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9232379236109516800 as u64)));
    }
    

    // Command line number: 1313
    #[test]
    fn test_94(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9232379236109516800 as u64)));
    }
    

    // Command line number: 1314
    #[test]
    fn test_95(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9007199254740992 as u64)));
    }
    

    // Command line number: 1315
    #[test]
    fn test_96(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9227875636482146304 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4503599627370496 as u64)));
    }
    

    // Command line number: 1316
    #[test]
    fn test_97(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9227875636482146304 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9227875636482146304 as u64)));
    }
    

    // Command line number: 1317
    #[test]
    fn test_98(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9227875636482146304 as u64)));
    }
    

    // Command line number: 1318
    #[test]
    fn test_99(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4503599627370496 as u64)));
    }
    

    // Command line number: 1319
    #[test]
    fn test_100(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9227875636482146304 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(716770142402832 as u64)));
    }
    

    // Command line number: 1320
    #[test]
    fn test_101(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9227875636482146304 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9224088806997178640 as u64)));
    }
    

    // Command line number: 1321
    #[test]
    fn test_102(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9224088806997178640 as u64)));
    }
    

    // Command line number: 1322
    #[test]
    fn test_103(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(716770142402832 as u64)));
    }
    

    // Command line number: 1323
    #[test]
    fn test_104(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9227875636482146304 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1324
    #[test]
    fn test_105(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9227875636482146304 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1325
    #[test]
    fn test_106(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1326
    #[test]
    fn test_107(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1327
    #[test]
    fn test_108(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9227875636482146304 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1328
    #[test]
    fn test_109(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9227875636482146304 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1329
    #[test]
    fn test_110(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1330
    #[test]
    fn test_111(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1331
    #[test]
    fn test_112(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9227875636482146304 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1332
    #[test]
    fn test_113(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9227875636482146304 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1333
    #[test]
    fn test_114(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9227875636482146304 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1334
    #[test]
    fn test_115(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9227875636482146304 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1335
    #[test]
    fn test_116(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1336
    #[test]
    fn test_117(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1337
    #[test]
    fn test_118(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1338
    #[test]
    fn test_119(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4503599627370496 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1339
    #[test]
    fn test_120(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1340
    #[test]
    fn test_121(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1341
    #[test]
    fn test_122(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1342
    #[test]
    fn test_123(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1343
    #[test]
    fn test_124(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1344
    #[test]
    fn test_125(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1345
    #[test]
    fn test_126(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1346
    #[test]
    fn test_127(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1347
    #[test]
    fn test_128(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9205357638345293824 as u64)));
    }
    

    // Command line number: 1348
    #[test]
    fn test_129(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18428729675200069632 as u64)));
    }
    

    // Command line number: 1349
    #[test]
    fn test_130(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18428729675200069632 as u64)));
    }
    

    // Command line number: 1350
    #[test]
    fn test_131(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9205357638345293824 as u64)));
    }
    

    // Command line number: 1351
    #[test]
    fn test_132(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 1352
    #[test]
    fn test_133(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13830554455654793216 as u64)));
    }
    

    // Command line number: 1353
    #[test]
    fn test_134(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13830554455654793216 as u64)));
    }
    

    // Command line number: 1354
    #[test]
    fn test_135(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 1355
    #[test]
    fn test_136(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4602678819172646912 as u64)));
    }
    

    // Command line number: 1356
    #[test]
    fn test_137(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13826050856027422720 as u64)));
    }
    

    // Command line number: 1357
    #[test]
    fn test_138(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13826050856027422720 as u64)));
    }
    

    // Command line number: 1358
    #[test]
    fn test_139(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4602678819172646912 as u64)));
    }
    

    // Command line number: 1359
    #[test]
    fn test_140(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4590398581802387587 as u64)));
    }
    

    // Command line number: 1360
    #[test]
    fn test_141(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13813770618657163395 as u64)));
    }
    

    // Command line number: 1361
    #[test]
    fn test_142(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13813770618657163395 as u64)));
    }
    

    // Command line number: 1362
    #[test]
    fn test_143(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4590398581802387587 as u64)));
    }
    

    // Command line number: 1363
    #[test]
    fn test_144(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(562949953421312 as u64)));
    }
    

    // Command line number: 1364
    #[test]
    fn test_145(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223934986808197120 as u64)));
    }
    

    // Command line number: 1365
    #[test]
    fn test_146(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223934986808197120 as u64)));
    }
    

    // Command line number: 1366
    #[test]
    fn test_147(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(562949953421312 as u64)));
    }
    

    // Command line number: 1367
    #[test]
    fn test_148(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1368
    #[test]
    fn test_149(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1369
    #[test]
    fn test_150(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1370
    #[test]
    fn test_151(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1371
    #[test]
    fn test_152(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1372
    #[test]
    fn test_153(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1373
    #[test]
    fn test_154(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1374
    #[test]
    fn test_155(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13826050856027422720 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1375
    #[test]
    fn test_156(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1376
    #[test]
    fn test_157(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1377
    #[test]
    fn test_158(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1378
    #[test]
    fn test_159(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646912 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1379
    #[test]
    fn test_160(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1380
    #[test]
    fn test_161(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1381
    #[test]
    fn test_162(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1382
    #[test]
    fn test_163(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1383
    #[test]
    fn test_164(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1384
    #[test]
    fn test_165(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1385
    #[test]
    fn test_166(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1386
    #[test]
    fn test_167(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1387
    #[test]
    fn test_168(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9209861237972664320 as u64)));
    }
    

    // Command line number: 1388
    #[test]
    fn test_169(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18433233274827440128 as u64)));
    }
    

    // Command line number: 1389
    #[test]
    fn test_170(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(9227875636482146304 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18433233274827440128 as u64)));
    }
    

    // Command line number: 1390
    #[test]
    fn test_171(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(4503599627370496 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9209861237972664320 as u64)));
    }
    

    // Command line number: 1391
    #[test]
    fn test_172(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4611686018427387904 as u64)));
    }
    

    // Command line number: 1392
    #[test]
    fn test_173(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13835058055282163712 as u64)));
    }
    

    // Command line number: 1393
    #[test]
    fn test_174(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(13826050856027422720 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13835058055282163712 as u64)));
    }
    

    // Command line number: 1394
    #[test]
    fn test_175(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4611686018427387904 as u64)));
    }
    

    // Command line number: 1395
    #[test]
    fn test_176(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 1396
    #[test]
    fn test_177(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13830554455654793216 as u64)));
    }
    

    // Command line number: 1397
    #[test]
    fn test_178(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13830554455654793216 as u64)));
    }
    

    // Command line number: 1398
    #[test]
    fn test_179(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 1399
    #[test]
    fn test_180(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4594902181429758083 as u64)));
    }
    

    // Command line number: 1400
    #[test]
    fn test_181(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13818274218284533891 as u64)));
    }
    

    // Command line number: 1401
    #[test]
    fn test_182(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(13842132293034192152 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13818274218284533891 as u64)));
    }
    

    // Command line number: 1402
    #[test]
    fn test_183(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(4618760256179416344 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4594902181429758083 as u64)));
    }
    

    // Command line number: 1403
    #[test]
    fn test_184(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1125899906842624 as u64)));
    }
    

    // Command line number: 1404
    #[test]
    fn test_185(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9224497936761618432 as u64)));
    }
    

    // Command line number: 1405
    #[test]
    fn test_186(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(18442240474082181119 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9224497936761618432 as u64)));
    }
    

    // Command line number: 1406
    #[test]
    fn test_187(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(9218868437227405311 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1125899906842624 as u64)));
    }
    

    // Command line number: 1407
    #[test]
    fn test_188(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1408
    #[test]
    fn test_189(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1409
    #[test]
    fn test_190(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1410
    #[test]
    fn test_191(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1411
    #[test]
    fn test_192(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1412
    #[test]
    fn test_193(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1413
    #[test]
    fn test_194(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1414
    #[test]
    fn test_195(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1415
    #[test]
    fn test_196(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(18444492273895866368 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1416
    #[test]
    fn test_197(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(18443366373989023744 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    

    // Command line number: 1417
    #[test]
    fn test_198(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(9221120237041090560 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1418
    #[test]
    fn test_199(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(9219994337134247936 as u64)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::DoubleWord));
        assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());
    }
    
}


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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "br_table.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 1247
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-i32", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 1248
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-i64", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 1249
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-f32", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 1250
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-f64", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 1252
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-i32-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1253
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-i64-value", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2 as u64)));
    }
    

    // Command line number: 1254
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-f32-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1077936128 as u32)));
    }
    

    // Command line number: 1255
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("type-f64-value", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4616189618054758400 as u64)));
    }
    

    // Command line number: 1257
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("empty", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(22 as u32)));
    }
    

    // Command line number: 1258
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("empty", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(22 as u32)));
    }
    

    // Command line number: 1259
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(11 as u32)];
        let result = runtime.call_exported_function("empty", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(22 as u32)));
    }
    

    // Command line number: 1260
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("empty", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(22 as u32)));
    }
    

    // Command line number: 1261
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967196 as u32)];
        let result = runtime.call_exported_function("empty", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(22 as u32)));
    }
    

    // Command line number: 1262
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("empty", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(22 as u32)));
    }
    

    // Command line number: 1264
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("empty-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 1265
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("empty-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 1266
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(11 as u32)];
        let result = runtime.call_exported_function("empty-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 1267
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("empty-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 1268
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967196 as u32)];
        let result = runtime.call_exported_function("empty-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 1269
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("empty-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 1271
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("singleton", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(22 as u32)));
    }
    

    // Command line number: 1272
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("singleton", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(20 as u32)));
    }
    

    // Command line number: 1273
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(11 as u32)];
        let result = runtime.call_exported_function("singleton", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(20 as u32)));
    }
    

    // Command line number: 1274
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("singleton", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(20 as u32)));
    }
    

    // Command line number: 1275
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967196 as u32)];
        let result = runtime.call_exported_function("singleton", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(20 as u32)));
    }
    

    // Command line number: 1276
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("singleton", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(20 as u32)));
    }
    

    // Command line number: 1278
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("singleton-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(32 as u32)));
    }
    

    // Command line number: 1279
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("singleton-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 1280
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(11 as u32)];
        let result = runtime.call_exported_function("singleton-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 1281
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("singleton-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 1282
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967196 as u32)];
        let result = runtime.call_exported_function("singleton-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 1283
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("singleton-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 1285
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("multiple", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(103 as u32)));
    }
    

    // Command line number: 1286
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("multiple", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(102 as u32)));
    }
    

    // Command line number: 1287
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("multiple", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(101 as u32)));
    }
    

    // Command line number: 1288
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("multiple", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(100 as u32)));
    }
    

    // Command line number: 1289
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32)];
        let result = runtime.call_exported_function("multiple", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(104 as u32)));
    }
    

    // Command line number: 1290
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32)];
        let result = runtime.call_exported_function("multiple", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(104 as u32)));
    }
    

    // Command line number: 1291
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("multiple", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(104 as u32)));
    }
    

    // Command line number: 1292
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(10 as u32)];
        let result = runtime.call_exported_function("multiple", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(104 as u32)));
    }
    

    // Command line number: 1293
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("multiple", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(104 as u32)));
    }
    

    // Command line number: 1294
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("multiple", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(104 as u32)));
    }
    

    // Command line number: 1296
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("multiple-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(213 as u32)));
    }
    

    // Command line number: 1297
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("multiple-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(212 as u32)));
    }
    

    // Command line number: 1298
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("multiple-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(211 as u32)));
    }
    

    // Command line number: 1299
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("multiple-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(210 as u32)));
    }
    

    // Command line number: 1300
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32)];
        let result = runtime.call_exported_function("multiple-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(214 as u32)));
    }
    

    // Command line number: 1301
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32)];
        let result = runtime.call_exported_function("multiple-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(214 as u32)));
    }
    

    // Command line number: 1302
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("multiple-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(214 as u32)));
    }
    

    // Command line number: 1303
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(10 as u32)];
        let result = runtime.call_exported_function("multiple-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(214 as u32)));
    }
    

    // Command line number: 1304
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("multiple-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(214 as u32)));
    }
    

    // Command line number: 1305
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("multiple-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(214 as u32)));
    }
    

    // Command line number: 1316
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-block-first", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 1317
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-block-mid", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 1318
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-block-last", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 1319
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-block-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 1321
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-loop-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 1322
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-loop-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4 as u32)));
    }
    

    // Command line number: 1323
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-loop-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(5 as u32)));
    }
    

    // Command line number: 1325
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 1327
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_if-cond", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 1328
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_if-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 1329
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_if-value-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 1331
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_table-index", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 1332
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_table-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(10 as u32)));
    }
    

    // Command line number: 1333
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-br_table-value-index", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(11 as u32)));
    }
    

    // Command line number: 1335
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-return-value", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(7 as u64)));
    }
    

    // Command line number: 1337
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-if-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 1338
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-if-then", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 1339
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-if-then", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 1340
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-if-else", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4 as u32)));
    }
    

    // Command line number: 1341
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-if-else", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 1343
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-select-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(5 as u32)));
    }
    

    // Command line number: 1344
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-select-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(5 as u32)));
    }
    

    // Command line number: 1345
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-select-second", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 1346
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("as-select-second", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(6 as u32)));
    }
    

    // Command line number: 1347
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-select-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(7 as u32)));
    }
    

    // Command line number: 1349
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(12 as u32)));
    }
    

    // Command line number: 1350
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(13 as u32)));
    }
    

    // Command line number: 1351
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(14 as u32)));
    }
    

    // Command line number: 1353
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-first", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(20 as u32)));
    }
    

    // Command line number: 1354
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-mid", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(21 as u32)));
    }
    

    // Command line number: 1355
    #[test]
    fn test_82(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-last", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(22 as u32)));
    }
    

    // Command line number: 1356
    #[test]
    fn test_83(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-call_indirect-func", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(23 as u32)));
    }
    

    // Command line number: 1358
    #[test]
    fn test_84(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-local.set-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(17 as u32)));
    }
    

    // Command line number: 1359
    #[test]
    fn test_85(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-local.tee-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1360
    #[test]
    fn test_86(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-global.set-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1362
    #[test]
    fn test_87(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-load-address", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1071225242 as u32)));
    }
    

    // Command line number: 1363
    #[test]
    fn test_88(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-loadN-address", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(30 as u64)));
    }
    

    // Command line number: 1365
    #[test]
    fn test_89(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-store-address", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(30 as u32)));
    }
    

    // Command line number: 1366
    #[test]
    fn test_90(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-store-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(31 as u32)));
    }
    

    // Command line number: 1367
    #[test]
    fn test_91(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-storeN-address", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(32 as u32)));
    }
    

    // Command line number: 1368
    #[test]
    fn test_92(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-storeN-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(33 as u32)));
    }
    

    // Command line number: 1370
    #[test]
    fn test_93(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-unary-operand", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1079613850 as u32)));
    }
    

    // Command line number: 1372
    #[test]
    fn test_94(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-binary-left", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 1373
    #[test]
    fn test_95(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-binary-right", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(45 as u64)));
    }
    

    // Command line number: 1375
    #[test]
    fn test_96(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-test-operand", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(44 as u32)));
    }
    

    // Command line number: 1377
    #[test]
    fn test_97(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-compare-left", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(43 as u32)));
    }
    

    // Command line number: 1378
    #[test]
    fn test_98(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-compare-right", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(42 as u32)));
    }
    

    // Command line number: 1380
    #[test]
    fn test_99(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-convert-operand", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(41 as u32)));
    }
    

    // Command line number: 1382
    #[test]
    fn test_100(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("as-memory.grow-size", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(40 as u32)));
    }
    

    // Command line number: 1384
    #[test]
    fn test_101(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("nested-block-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(19 as u32)));
    }
    

    // Command line number: 1385
    #[test]
    fn test_102(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nested-block-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(17 as u32)));
    }
    

    // Command line number: 1386
    #[test]
    fn test_103(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("nested-block-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(16 as u32)));
    }
    

    // Command line number: 1387
    #[test]
    fn test_104(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(10 as u32)];
        let result = runtime.call_exported_function("nested-block-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(16 as u32)));
    }
    

    // Command line number: 1388
    #[test]
    fn test_105(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("nested-block-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(16 as u32)));
    }
    

    // Command line number: 1389
    #[test]
    fn test_106(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(100000 as u32)];
        let result = runtime.call_exported_function("nested-block-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(16 as u32)));
    }
    

    // Command line number: 1391
    #[test]
    fn test_107(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("nested-br-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 1392
    #[test]
    fn test_108(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nested-br-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 1393
    #[test]
    fn test_109(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("nested-br-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(17 as u32)));
    }
    

    // Command line number: 1394
    #[test]
    fn test_110(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(11 as u32)];
        let result = runtime.call_exported_function("nested-br-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(17 as u32)));
    }
    

    // Command line number: 1395
    #[test]
    fn test_111(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967292 as u32)];
        let result = runtime.call_exported_function("nested-br-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(17 as u32)));
    }
    

    // Command line number: 1396
    #[test]
    fn test_112(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(10213210 as u32)];
        let result = runtime.call_exported_function("nested-br-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(17 as u32)));
    }
    

    // Command line number: 1398
    #[test]
    fn test_113(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(17 as u32)));
    }
    

    // Command line number: 1399
    #[test]
    fn test_114(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 1400
    #[test]
    fn test_115(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 1401
    #[test]
    fn test_116(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(9 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 1402
    #[test]
    fn test_117(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967287 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 1403
    #[test]
    fn test_118(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(999999 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 1405
    #[test]
    fn test_119(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 1406
    #[test]
    fn test_120(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 1407
    #[test]
    fn test_121(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 1408
    #[test]
    fn test_122(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 1409
    #[test]
    fn test_123(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4293967296 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 1410
    #[test]
    fn test_124(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(9423975 as u32)];
        let result = runtime.call_exported_function("nested-br_if-value-cond", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 1412
    #[test]
    fn test_125(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(17 as u32)));
    }
    

    // Command line number: 1413
    #[test]
    fn test_126(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 1414
    #[test]
    fn test_127(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 1415
    #[test]
    fn test_128(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(9 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 1416
    #[test]
    fn test_129(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967287 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 1417
    #[test]
    fn test_130(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(999999 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 1419
    #[test]
    fn test_131(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value-index", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 1420
    #[test]
    fn test_132(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value-index", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8 as u32)));
    }
    

    // Command line number: 1421
    #[test]
    fn test_133(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value-index", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 1422
    #[test]
    fn test_134(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value-index", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 1423
    #[test]
    fn test_135(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4293967296 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value-index", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 1424
    #[test]
    fn test_136(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(9423975 as u32)];
        let result = runtime.call_exported_function("nested-br_table-value-index", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(9 as u32)));
    }
    

    // Command line number: 1426
    #[test]
    fn test_137(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("nested-br_table-loop-block", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    
}

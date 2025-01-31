
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "i32.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 262
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147516416 as u32)];
        let result = runtime.call_exported_function("popcnt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 263
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("popcnt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(31 as u32)));
    }
    

    // Command line number: 264
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2863311530 as u32)];
        let result = runtime.call_exported_function("popcnt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(16 as u32)));
    }
    

    // Command line number: 265
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1431655765 as u32)];
        let result = runtime.call_exported_function("popcnt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(16 as u32)));
    }
    

    // Command line number: 266
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3735928559 as u32)];
        let result = runtime.call_exported_function("popcnt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(24 as u32)));
    }
    

    // Command line number: 268
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("eqz", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 269
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("eqz", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 270
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("eqz", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 271
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("eqz", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 272
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("eqz", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 274
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("eq", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 275
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("eq", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 276
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("eq", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 277
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("eq", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 278
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("eq", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 279
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("eq", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 280
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("eq", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 281
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("eq", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 282
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("eq", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 283
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("eq", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 284
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("eq", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 285
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("eq", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 286
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("eq", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 287
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("eq", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 289
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("ne", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 290
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("ne", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 291
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("ne", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 292
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("ne", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 293
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("ne", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 294
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("ne", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 295
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("ne", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 296
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("ne", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 297
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("ne", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 298
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("ne", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 299
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("ne", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 300
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("ne", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 301
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("ne", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 302
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("ne", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 304
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("lt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 305
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("lt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 306
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("lt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 307
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("lt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 308
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("lt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 309
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("lt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 310
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("lt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 311
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("lt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 312
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("lt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 313
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("lt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 314
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("lt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 315
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("lt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 316
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("lt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 317
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("lt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 319
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("lt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 320
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("lt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 321
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("lt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 322
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("lt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 323
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("lt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 324
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("lt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 325
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("lt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 326
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("lt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 327
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("lt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 328
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("lt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 329
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("lt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 330
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("lt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 331
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("lt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 332
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("lt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 334
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("le_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 335
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("le_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 336
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("le_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 337
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("le_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 338
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("le_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 339
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("le_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 340
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("le_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 341
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("le_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 342
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("le_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 343
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("le_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 344
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("le_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 345
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("le_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 346
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("le_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 347
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("le_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 349
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("le_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 350
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("le_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 351
    #[test]
    fn test_82(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("le_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 352
    #[test]
    fn test_83(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("le_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 353
    #[test]
    fn test_84(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("le_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 354
    #[test]
    fn test_85(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("le_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 355
    #[test]
    fn test_86(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("le_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 356
    #[test]
    fn test_87(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("le_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 357
    #[test]
    fn test_88(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("le_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 358
    #[test]
    fn test_89(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("le_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 359
    #[test]
    fn test_90(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("le_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 360
    #[test]
    fn test_91(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("le_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 361
    #[test]
    fn test_92(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("le_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 362
    #[test]
    fn test_93(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("le_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 364
    #[test]
    fn test_94(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("gt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 365
    #[test]
    fn test_95(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("gt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 366
    #[test]
    fn test_96(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("gt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 367
    #[test]
    fn test_97(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("gt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 368
    #[test]
    fn test_98(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("gt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 369
    #[test]
    fn test_99(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("gt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 370
    #[test]
    fn test_100(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("gt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 371
    #[test]
    fn test_101(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("gt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 372
    #[test]
    fn test_102(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("gt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 373
    #[test]
    fn test_103(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("gt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 374
    #[test]
    fn test_104(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("gt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 375
    #[test]
    fn test_105(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("gt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 376
    #[test]
    fn test_106(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("gt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 377
    #[test]
    fn test_107(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("gt_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 379
    #[test]
    fn test_108(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("gt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 380
    #[test]
    fn test_109(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("gt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 381
    #[test]
    fn test_110(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("gt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 382
    #[test]
    fn test_111(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("gt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 383
    #[test]
    fn test_112(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("gt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 384
    #[test]
    fn test_113(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("gt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 385
    #[test]
    fn test_114(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("gt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 386
    #[test]
    fn test_115(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("gt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 387
    #[test]
    fn test_116(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("gt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 388
    #[test]
    fn test_117(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("gt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 389
    #[test]
    fn test_118(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("gt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 390
    #[test]
    fn test_119(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("gt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 391
    #[test]
    fn test_120(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("gt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 392
    #[test]
    fn test_121(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("gt_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 394
    #[test]
    fn test_122(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("ge_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 395
    #[test]
    fn test_123(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("ge_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 396
    #[test]
    fn test_124(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("ge_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 397
    #[test]
    fn test_125(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("ge_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 398
    #[test]
    fn test_126(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("ge_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 399
    #[test]
    fn test_127(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("ge_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 400
    #[test]
    fn test_128(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("ge_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 401
    #[test]
    fn test_129(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("ge_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 402
    #[test]
    fn test_130(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("ge_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 403
    #[test]
    fn test_131(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("ge_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 404
    #[test]
    fn test_132(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("ge_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 405
    #[test]
    fn test_133(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("ge_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 406
    #[test]
    fn test_134(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("ge_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 407
    #[test]
    fn test_135(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("ge_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 409
    #[test]
    fn test_136(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("ge_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 410
    #[test]
    fn test_137(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("ge_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 411
    #[test]
    fn test_138(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("ge_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 412
    #[test]
    fn test_139(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("ge_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 413
    #[test]
    fn test_140(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("ge_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 414
    #[test]
    fn test_141(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("ge_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 415
    #[test]
    fn test_142(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("ge_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 416
    #[test]
    fn test_143(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("ge_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 417
    #[test]
    fn test_144(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("ge_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 418
    #[test]
    fn test_145(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("ge_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 419
    #[test]
    fn test_146(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("ge_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 420
    #[test]
    fn test_147(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("ge_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 421
    #[test]
    fn test_148(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("ge_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 422
    #[test]
    fn test_149(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("ge_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    
}

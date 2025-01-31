
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.56.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 1285
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3765723020 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_recip_recip", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3765723019 as u32)));
    }
    

    // Command line number: 1286
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(426844452 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_recip_recip", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(426844451 as u32)));
    }
    

    // Command line number: 1287
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(535132276 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_recip_recip", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(535132277 as u32)));
    }
    

    // Command line number: 1288
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3253941441 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_recip_recip", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3253941442 as u32)));
    }
    

    // Command line number: 1289
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1660734603 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_recip_recip", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1660734602 as u32)));
    }
    

    // Command line number: 1291
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_recip_recip", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1292
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_recip_recip", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1293
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_recip_recip", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1294
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_recip_recip", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1296
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14500888369201570768 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_recip_recip", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14500888369201570769 as u64)));
    }
    

    // Command line number: 1297
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14132092565459057123 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_recip_recip", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14132092565459057122 as u64)));
    }
    

    // Command line number: 1298
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5359183527603521526 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_recip_recip", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5359183527603521525 as u64)));
    }
    

    // Command line number: 1299
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1521566147669375634 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_recip_recip", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1521566147669375633 as u64)));
    }
    

    // Command line number: 1300
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(8671785631545870379 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_recip_recip", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(8671785631545870378 as u64)));
    }
    

    // Command line number: 1302
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_recip_recip", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1303
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_recip_recip", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1304
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18442240474082181120 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_recip_recip", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1305
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9218868437227405312 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_recip_recip", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    
}

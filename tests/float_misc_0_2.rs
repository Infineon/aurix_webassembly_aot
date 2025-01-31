
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

    
    // Command line number: 645
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4680825439885721600 as u64)];
        let result = runtime.call_exported_function("f64.floor", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4680825439885721600 as u64)));
    }
    

    // Command line number: 648
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3405774847 as u32)];
        let result = runtime.call_exported_function("f32.trunc", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3405774846 as u32)));
    }
    

    // Command line number: 649
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1258291199 as u32)];
        let result = runtime.call_exported_function("f32.trunc", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1258291198 as u32)));
    }
    

    // Command line number: 650
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14064741636278059007 as u64)];
        let result = runtime.call_exported_function("f64.trunc", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14064741636278059006 as u64)));
    }
    

    // Command line number: 651
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4841369599423283199 as u64)];
        let result = runtime.call_exported_function("f64.trunc", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4841369599423283198 as u64)));
    }
    

    // Command line number: 656
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1258291201 as u32)];
        let result = runtime.call_exported_function("f32.nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1258291201 as u32)));
    }
    

    // Command line number: 657
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1258291202 as u32)];
        let result = runtime.call_exported_function("f32.nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1258291202 as u32)));
    }
    

    // Command line number: 658
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964607 as u32)];
        let result = runtime.call_exported_function("f32.nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 659
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1468006399 as u32)];
        let result = runtime.call_exported_function("f32.nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1468006399 as u32)));
    }
    

    // Command line number: 660
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4841369599423283201 as u64)];
        let result = runtime.call_exported_function("f64.nearest", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4841369599423283201 as u64)));
    }
    

    // Command line number: 661
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4841369599423283202 as u64)];
        let result = runtime.call_exported_function("f64.nearest", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4841369599423283202 as u64)));
    }
    

    // Command line number: 662
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4602678819172646911 as u64)];
        let result = runtime.call_exported_function("f64.nearest", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 663
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5084563979301289983 as u64)];
        let result = runtime.call_exported_function("f64.nearest", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5084563979301289983 as u64)));
    }
    

    // Command line number: 667
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1083179008 as u32)];
        let result = runtime.call_exported_function("f32.nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1082130432 as u32)));
    }
    

    // Command line number: 668
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3230662656 as u32)];
        let result = runtime.call_exported_function("f32.nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3229614080 as u32)));
    }
    

    // Command line number: 669
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3227516928 as u32)];
        let result = runtime.call_exported_function("f32.nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3229614080 as u32)));
    }
    

    // Command line number: 670
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4616752568008179712 as u64)];
        let result = runtime.call_exported_function("f64.nearest", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4616189618054758400 as u64)));
    }
    

    // Command line number: 671
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13840124604862955520 as u64)];
        let result = runtime.call_exported_function("f64.nearest", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13839561654909534208 as u64)));
    }
    

    // Command line number: 672
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13838435755002691584 as u64)];
        let result = runtime.call_exported_function("f64.nearest", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13839561654909534208 as u64)));
    }
    

    // Command line number: 675
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3405774847 as u32)];
        let result = runtime.call_exported_function("f32.nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3405774848 as u32)));
    }
    

    // Command line number: 676
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1258291199 as u32)];
        let result = runtime.call_exported_function("f32.nearest", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1258291200 as u32)));
    }
    

    // Command line number: 677
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14064741636278059007 as u64)];
        let result = runtime.call_exported_function("f64.nearest", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14064741636278059008 as u64)));
    }
    

    // Command line number: 678
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4841369599423283199 as u64)];
        let result = runtime.call_exported_function("f64.nearest", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4841369599423283200 as u64)));
    }
    
}

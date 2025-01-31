
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.69.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 1727
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2304917983 as u32),Immediate::Word(301403678 as u32),Immediate::Word(331350955 as u32),Immediate::Word(3251297465 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_mul_divs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(148760966 as u32)));
    }
    

    // Command line number: 1728
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4068974897 as u32),Immediate::Word(1276265036 as u32),Immediate::Word(930821438 as u32),Immediate::Word(1044692964 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_mul_divs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3742862674 as u32)));
    }
    

    // Command line number: 1729
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3496980369 as u32),Immediate::Word(3548280607 as u32),Immediate::Word(3461305482 as u32),Immediate::Word(3298174616 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_mul_divs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1176926862 as u32)));
    }
    

    // Command line number: 1730
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4135236702 as u32),Immediate::Word(787270424 as u32),Immediate::Word(932959293 as u32),Immediate::Word(1724950821 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_mul_divs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1731
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(622783177 as u32),Immediate::Word(2677642769 as u32),Immediate::Word(307759154 as u32),Immediate::Word(768171421 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_mul_divs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2844661464 as u32)));
    }
    

    // Command line number: 1733
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(10143060558527560466 as u64),Immediate::DoubleWord(11745059379675007839 as u64),Immediate::DoubleWord(16295837305232663584 as u64),Immediate::DoubleWord(5444961058358534642 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_mul_divs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13856326607560224491 as u64)));
    }
    

    // Command line number: 1734
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14349445329289351080 as u64),Immediate::DoubleWord(468238185841254727 as u64),Immediate::DoubleWord(15463559257629249878 as u64),Immediate::DoubleWord(15937497686185055572 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_mul_divs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1735
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(15220380342429201729 as u64),Immediate::DoubleWord(14697937818549468616 as u64),Immediate::DoubleWord(13203624158275174657 as u64),Immediate::DoubleWord(17131104131485469546 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_mul_divs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1202126128702318245 as u64)));
    }
    

    // Command line number: 1736
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14414969397981384765 as u64),Immediate::DoubleWord(12269327994486371199 as u64),Immediate::DoubleWord(298707625567048656 as u64),Immediate::DoubleWord(5613107161545919917 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_mul_divs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1737
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4529089342618677929 as u64),Immediate::DoubleWord(3361245300043094097 as u64),Immediate::DoubleWord(1815899012046749567 as u64),Immediate::DoubleWord(15418396504351552390 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_mul_divs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10619033301585441215 as u64)));
    }
    
}


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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.74.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 1837
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3900330981 as u32),Immediate::Word(1843416431 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_mul_sqrt_div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1838
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2210946958 as u32),Immediate::Word(256302916 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_mul_sqrt_div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1840
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3576537897 as u32),Immediate::Word(2010442638 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_mul_sqrt_div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3104219421 as u32)));
    }
    

    // Command line number: 1841
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3284697858 as u32),Immediate::Word(1124488329 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_mul_sqrt_div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3255461622 as u32)));
    }
    

    // Command line number: 1843
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(7751219282814906463 as u64),Immediate::DoubleWord(8023732701704228537 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_mul_sqrt_div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 1844
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(10108528314069607083 as u64),Immediate::DoubleWord(1595930056995453707 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_mul_sqrt_div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 1845
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2695209648295623224 as u64),Immediate::DoubleWord(7133480874314061811 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_mul_sqrt_div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1432338140829931582 as u64)));
    }
    

    // Command line number: 1846
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(15416524255949334213 as u64),Immediate::DoubleWord(2434442666062773630 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_mul_sqrt_div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(16502590179898118478 as u64)));
    }
    

    // Command line number: 1847
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5076901024782455083 as u64),Immediate::DoubleWord(8399438310541178654 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_mul_sqrt_div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3180744754328846996 as u64)));
    }
    
}

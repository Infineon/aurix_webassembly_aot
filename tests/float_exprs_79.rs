
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.79.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 1972
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1073741824 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1068827891 as u32)));
    }
    

    // Command line number: 1973
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1077936128 as u32),Immediate::Word(1084227584 as u32),Immediate::Word(1078530011 as u32),Immediate::Word(1088421888 as u32)];
        let result = runtime.call_exported_function("f32.xkcd_sqrt_2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1068827946 as u32)));
    }
    

    // Command line number: 1974
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1077936128 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1071494103 as u32)));
    }
    

    // Command line number: 1975
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1073741824 as u32),Immediate::Word(1076754516 as u32),Immediate::Word(1078530011 as u32)];
        let result = runtime.call_exported_function("f32.xkcd_sqrt_3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1071481194 as u32)));
    }
    

    // Command line number: 1976
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1084227584 as u32)];
        let result = runtime.call_exported_function("f32.sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1074731965 as u32)));
    }
    

    // Command line number: 1977
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1073741824 as u32),Immediate::Word(1076754516 as u32),Immediate::Word(1077936128 as u32)];
        let result = runtime.call_exported_function("f32.xkcd_sqrt_5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1074730668 as u32)));
    }
    

    // Command line number: 1978
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1095761920 as u32),Immediate::Word(1082130432 as u32),Immediate::Word(1078530011 as u32),Immediate::Word(1103101952 as u32)];
        let result = runtime.call_exported_function("f32.xkcd_better_sqrt_5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1074731965 as u32)));
    }
    

    // Command line number: 1980
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4611686018427387904 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4609047870845172685 as u64)));
    }
    

    // Command line number: 1981
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4613937818241073152 as u64),Immediate::DoubleWord(4617315517961601024 as u64),Immediate::DoubleWord(4614256656552045848 as u64),Immediate::DoubleWord(4619567317775286272 as u64)];
        let result = runtime.call_exported_function("f64.xkcd_sqrt_2", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4609047900099118431 as u64)));
    }
    

    // Command line number: 1982
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4613937818241073152 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4610479282544200874 as u64)));
    }
    

    // Command line number: 1983
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4611686018427387904 as u64),Immediate::DoubleWord(4613303445314885481 as u64),Immediate::DoubleWord(4614256656552045848 as u64)];
        let result = runtime.call_exported_function("f64.xkcd_sqrt_3", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4610472352185749397 as u64)));
    }
    

    // Command line number: 1984
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4617315517961601024 as u64)];
        let result = runtime.call_exported_function("f64.sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4612217596255138984 as u64)));
    }
    

    // Command line number: 1985
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4611686018427387904 as u64),Immediate::DoubleWord(4613303445314885481 as u64),Immediate::DoubleWord(4613937818241073152 as u64)];
        let result = runtime.call_exported_function("f64.xkcd_sqrt_5", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4612216900234722254 as u64)));
    }
    

    // Command line number: 1986
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4623507967449235456 as u64),Immediate::DoubleWord(4616189618054758400 as u64),Immediate::DoubleWord(4614256656552045848 as u64),Immediate::DoubleWord(4627448617123184640 as u64)];
        let result = runtime.call_exported_function("f64.xkcd_better_sqrt_5", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4612217595876713891 as u64)));
    }
    
}

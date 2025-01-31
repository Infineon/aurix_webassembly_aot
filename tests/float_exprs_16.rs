
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.16.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 240
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3634023955 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_div_3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3620628505 as u32)));
    }
    

    // Command line number: 241
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4000459555 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_div_3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3986780695 as u32)));
    }
    

    // Command line number: 242
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2517965963 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_div_3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2504446137 as u32)));
    }
    

    // Command line number: 243
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2173683100 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_div_3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2160046629 as u32)));
    }
    

    // Command line number: 244
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2750097330 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_div_3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2736571681 as u32)));
    }
    

    // Command line number: 245
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(16679796490173820099 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_3", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(16672802667330368301 as u64)));
    }
    

    // Command line number: 246
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13081777497422760306 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_3", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13074664638073319671 as u64)));
    }
    

    // Command line number: 247
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(674365394458900388 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_3", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(667250911628840899 as u64)));
    }
    

    // Command line number: 248
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18365700772251870524 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_3", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18358201936817915643 as u64)));
    }
    

    // Command line number: 249
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6476267216527259981 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_3", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(6468791534604471399 as u64)));
    }
    
}


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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.67.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 1680
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4046243078 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_6x_via_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4068578245 as u32)));
    }
    

    // Command line number: 1681
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2573857750 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_6x_via_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2595190497 as u32)));
    }
    

    // Command line number: 1682
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(419462401 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_6x_via_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(440449921 as u32)));
    }
    

    // Command line number: 1683
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2955475482 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_6x_via_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2977789734 as u32)));
    }
    

    // Command line number: 1684
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3883931973 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_6x_via_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3904906727 as u32)));
    }
    

    // Command line number: 1686
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14137662215323058150 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_6x_via_add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14149352706895019994 as u64)));
    }
    

    // Command line number: 1687
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(11424134044545165748 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_6x_via_add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(11435767596137037638 as u64)));
    }
    

    // Command line number: 1688
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(15055410132664937138 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_6x_via_add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(15066699987142021125 as u64)));
    }
    

    // Command line number: 1689
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(7991451501228919438 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_6x_via_add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(8003319959635773419 as u64)));
    }
    

    // Command line number: 1690
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14886926859367497770 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_6x_via_add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14898679235615764511 as u64)));
    }
    
}

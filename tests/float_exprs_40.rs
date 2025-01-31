
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.40.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 784
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1097963930 as u32)];
        let result = runtime.call_exported_function("init", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 785
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32),Immediate::Word(1098068787 as u32)];
        let result = runtime.call_exported_function("init", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 786
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8 as u32),Immediate::Word(1098173645 as u32)];
        let result = runtime.call_exported_function("init", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 787
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(12 as u32),Immediate::Word(1098278502 as u32)];
        let result = runtime.call_exported_function("init", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 788
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("check", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1097963930 as u32)));
    }
    

    // Command line number: 789
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32)];
        let result = runtime.call_exported_function("check", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1098068787 as u32)));
    }
    

    // Command line number: 790
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8 as u32)];
        let result = runtime.call_exported_function("check", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1098173645 as u32)));
    }
    

    // Command line number: 791
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(12 as u32)];
        let result = runtime.call_exported_function("check", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1098278502 as u32)));
    }
    

    // Command line number: 792
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(16 as u32),Immediate::Word(1077936128 as u32)];
        let result = runtime.call_exported_function("run", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 793
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("check", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1084297489 as u32)));
    }
    

    // Command line number: 794
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32)];
        let result = runtime.call_exported_function("check", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1084367394 as u32)));
    }
    

    // Command line number: 795
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8 as u32)];
        let result = runtime.call_exported_function("check", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1084437299 as u32)));
    }
    

    // Command line number: 796
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(12 as u32)];
        let result = runtime.call_exported_function("check", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1084507204 as u32)));
    }
    
}


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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_memory.2.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 67
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32.load", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2141192192 as u32)));
    }
    

    // Command line number: 68
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.load", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2141192192 as u32)));
    }
    

    // Command line number: 69
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("reset", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 70
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32.load", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 71
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.load", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 72
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.store", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 73
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32.load", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2141192192 as u32)));
    }
    

    // Command line number: 74
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.load", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2141192192 as u32)));
    }
    

    // Command line number: 75
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("reset", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 76
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32.load", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 77
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.load", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 78
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32.store", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 79
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("i32.load", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2141192192 as u32)));
    }
    

    // Command line number: 80
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("f32.load", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2141192192 as u32)));
    }
    
}

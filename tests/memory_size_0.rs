
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "memory_size.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 7
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("size", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 8
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("grow", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 9
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("size", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 10
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32)];
        let result = runtime.call_exported_function("grow", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 11
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("size", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 12
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("grow", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 13
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("size", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    
}

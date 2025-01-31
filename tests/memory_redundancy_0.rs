
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "memory_redundancy.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 59
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("test_store_to_load", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(128 as u32)));
    }
    

    // Command line number: 60
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("zero_everything", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 61
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("test_redundant_load", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(128 as u32)));
    }
    

    // Command line number: 62
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("zero_everything", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 63
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("test_dead_store", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(35 as u32)));
    }
    

    // Command line number: 64
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("zero_everything", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 65
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("malloc_aliasing", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(43 as u32)));
    }
    
}

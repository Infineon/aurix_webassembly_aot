
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.60.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 1430
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(32 as u32),Immediate::Word(64 as u32)];
        let result = runtime.call_exported_function("f64.simple_x4_sum", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 1431
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(64 as u32)];
        let result = runtime.call_exported_function("f64.load", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2 as u64)));
    }
    

    // Command line number: 1432
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(72 as u32)];
        let result = runtime.call_exported_function("f64.load", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1433
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(80 as u32)];
        let result = runtime.call_exported_function("f64.load", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 1434
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(88 as u32)];
        let result = runtime.call_exported_function("f64.load", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775809 as u64)));
    }
    
}

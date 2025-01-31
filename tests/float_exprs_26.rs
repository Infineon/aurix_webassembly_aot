
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.26.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 452
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3130294363 as u32)];
        let result = runtime.call_exported_function("f32.no_approximate_reciprocal", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3294406762 as u32)));
    }
    

    // Command line number: 454
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2434880051 as u32)];
        let result = runtime.call_exported_function("f32.no_approximate_reciprocal", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3989512051 as u32)));
    }
    

    // Command line number: 455
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1705936409 as u32)];
        let result = runtime.call_exported_function("f32.no_approximate_reciprocal", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(423346609 as u32)));
    }
    

    // Command line number: 456
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2528120561 as u32)];
        let result = runtime.call_exported_function("f32.no_approximate_reciprocal", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3896123071 as u32)));
    }
    
}

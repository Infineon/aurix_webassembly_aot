
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.28.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 486
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1574069443 as u32)];
        let result = runtime.call_exported_function("f32.no_approximate_sqrt_reciprocal", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(810003811 as u32)));
    }
    

    // Command line number: 487
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(992487567 as u32)];
        let result = runtime.call_exported_function("f32.no_approximate_sqrt_reciprocal", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1100869283 as u32)));
    }
    

    // Command line number: 488
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1644769121 as u32)];
        let result = runtime.call_exported_function("f32.no_approximate_sqrt_reciprocal", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(774822585 as u32)));
    }
    

    // Command line number: 489
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1180509736 as u32)];
        let result = runtime.call_exported_function("f32.no_approximate_sqrt_reciprocal", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1007269771 as u32)));
    }
    

    // Command line number: 490
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1940205041 as u32)];
        let result = runtime.call_exported_function("f32.no_approximate_sqrt_reciprocal", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(627137240 as u32)));
    }
    
}

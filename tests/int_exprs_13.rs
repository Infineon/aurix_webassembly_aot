
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "int_exprs.13.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 238
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(71 as u32)];
        let result = runtime.call_exported_function("i32.div_s_5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(14 as u32)));
    }
    

    // Command line number: 239
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1342177280 as u32)];
        let result = runtime.call_exported_function("i32.div_s_5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(268435456 as u32)));
    }
    

    // Command line number: 240
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(71 as u32)];
        let result = runtime.call_exported_function("i32.div_u_5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(14 as u32)));
    }
    

    // Command line number: 241
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2684354560 as u32)];
        let result = runtime.call_exported_function("i32.div_u_5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(536870912 as u32)));
    }
    

    // Command line number: 242
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(71 as u64)];
        let result = runtime.call_exported_function("i64.div_s_5", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14 as u64)));
    }
    

    // Command line number: 243
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5764607523034234880 as u64)];
        let result = runtime.call_exported_function("i64.div_s_5", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1152921504606846976 as u64)));
    }
    

    // Command line number: 244
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(71 as u64)];
        let result = runtime.call_exported_function("i64.div_u_5", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14 as u64)));
    }
    

    // Command line number: 245
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(11529215046068469760 as u64)];
        let result = runtime.call_exported_function("i64.div_u_5", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2305843009213693952 as u64)));
    }
    
}

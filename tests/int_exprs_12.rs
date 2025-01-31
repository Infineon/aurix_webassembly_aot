
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "int_exprs.12.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 215
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(71 as u32)];
        let result = runtime.call_exported_function("i32.div_s_3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(23 as u32)));
    }
    

    // Command line number: 216
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1610612736 as u32)];
        let result = runtime.call_exported_function("i32.div_s_3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(536870912 as u32)));
    }
    

    // Command line number: 217
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(71 as u32)];
        let result = runtime.call_exported_function("i32.div_u_3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(23 as u32)));
    }
    

    // Command line number: 218
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3221225472 as u32)];
        let result = runtime.call_exported_function("i32.div_u_3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1073741824 as u32)));
    }
    

    // Command line number: 219
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(71 as u64)];
        let result = runtime.call_exported_function("i64.div_s_3", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(23 as u64)));
    }
    

    // Command line number: 220
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(3458764513820540928 as u64)];
        let result = runtime.call_exported_function("i64.div_s_3", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1152921504606846976 as u64)));
    }
    

    // Command line number: 221
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(71 as u64)];
        let result = runtime.call_exported_function("i64.div_u_3", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(23 as u64)));
    }
    

    // Command line number: 222
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13835058055282163712 as u64)];
        let result = runtime.call_exported_function("i64.div_u_3", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4611686018427387904 as u64)));
    }
    
}


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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.41.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 819
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::DoubleWord(4624690162351420211 as u64)];
        let result = runtime.call_exported_function("init", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 820
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8 as u32),Immediate::DoubleWord(4624746457346762342 as u64)];
        let result = runtime.call_exported_function("init", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 821
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(16 as u32),Immediate::DoubleWord(4624802752342104474 as u64)];
        let result = runtime.call_exported_function("init", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 822
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(24 as u32),Immediate::DoubleWord(4624859047337446605 as u64)];
        let result = runtime.call_exported_function("init", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 823
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("check", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4624690162351420211 as u64)));
    }
    

    // Command line number: 824
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8 as u32)];
        let result = runtime.call_exported_function("check", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4624746457346762342 as u64)));
    }
    

    // Command line number: 825
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(16 as u32)];
        let result = runtime.call_exported_function("check", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4624802752342104474 as u64)));
    }
    

    // Command line number: 826
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(24 as u32)];
        let result = runtime.call_exported_function("check", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4624859047337446605 as u64)));
    }
    

    // Command line number: 827
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(32 as u32),Immediate::DoubleWord(4613937818241073152 as u64)];
        let result = runtime.call_exported_function("run", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 828
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("check", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4617353047958495778 as u64)));
    }
    

    // Command line number: 829
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8 as u32)];
        let result = runtime.call_exported_function("check", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4617390577955390532 as u64)));
    }
    

    // Command line number: 830
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(16 as u32)];
        let result = runtime.call_exported_function("check", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4617428107952285287 as u64)));
    }
    

    // Command line number: 831
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(24 as u32)];
        let result = runtime.call_exported_function("check", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4617465637949180041 as u64)));
    }
    
}

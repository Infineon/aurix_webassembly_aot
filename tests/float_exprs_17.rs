
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.17.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 260
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3550941609 as u32),Immediate::Word(3628209942 as u32),Immediate::Word(1568101121 as u32)];
        let result = runtime.call_exported_function("f32.no_factor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4131116008 as u32)));
    }
    

    // Command line number: 261
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3168433147 as u32),Immediate::Word(1028017286 as u32),Immediate::Word(3141035521 as u32)];
        let result = runtime.call_exported_function("f32.no_factor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3095417249 as u32)));
    }
    

    // Command line number: 262
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2869115159 as u32),Immediate::Word(536308199 as u32),Immediate::Word(2100177580 as u32)];
        let result = runtime.call_exported_function("f32.no_factor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3904015703 as u32)));
    }
    

    // Command line number: 263
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2684117842 as u32),Immediate::Word(369386499 as u32),Immediate::Word(2061166438 as u32)];
        let result = runtime.call_exported_function("f32.no_factor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3679965352 as u32)));
    }
    

    // Command line number: 264
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2510116111 as u32),Immediate::Word(476277495 as u32),Immediate::Word(1237750930 as u32)];
        let result = runtime.call_exported_function("f32.no_factor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(649094375 as u32)));
    }
    

    // Command line number: 265
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2698691837980592503 as u64),Immediate::DoubleWord(2529920934327896545 as u64),Immediate::DoubleWord(12819783413251458936 as u64)];
        let result = runtime.call_exported_function("f64.no_factor", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10911876679403600666 as u64)));
    }
    

    // Command line number: 266
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1626864102540432200 as u64),Immediate::DoubleWord(9287829620889669687 as u64),Immediate::DoubleWord(9524500187773169472 as u64)];
        let result = runtime.call_exported_function("f64.no_factor", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 267
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(12326480769054961745 as u64),Immediate::DoubleWord(12563546453737163926 as u64),Immediate::DoubleWord(15990519985875741037 as u64)];
        let result = runtime.call_exported_function("f64.no_factor", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5500432744005058080 as u64)));
    }
    

    // Command line number: 268
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(12532477544855171977 as u64),Immediate::DoubleWord(3439526350000314825 as u64),Immediate::DoubleWord(12694541248380731909 as u64)];
        let result = runtime.call_exported_function("f64.no_factor", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(11527035460272583044 as u64)));
    }
    

    // Command line number: 269
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1871759566187673434 as u64),Immediate::DoubleWord(2002968319587025494 as u64),Immediate::DoubleWord(16033202089880281080 as u64)];
        let result = runtime.call_exported_function("f64.no_factor", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13429277897969282899 as u64)));
    }
    
}

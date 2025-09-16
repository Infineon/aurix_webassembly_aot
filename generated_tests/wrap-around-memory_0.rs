
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

            let wasm_code = include_bytes!(concat!("../wasm_json_from_wast/", "wrap-around-memory.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 40
    #[test]
    #[cfg(feature="address-masking")]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(131072 as u32)];
        let result = runtime.call_exported_function("load8", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 43
    #[test]
    #[cfg(feature="address-masking")]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(131072 as u32)];
        let result = runtime.call_exported_function("load16", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(513 as u32)));
    }
    

    // Command line number: 46
    #[test]
    #[cfg(feature="address-masking")]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(131072 as u32)];
        let result = runtime.call_exported_function("load32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(67305985 as u32)));
    }
    

    // Command line number: 50
    #[test]
    #[cfg(feature="address-masking")]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(131069 as u32)];
        let result = runtime.call_exported_function("load32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2232831 as u32)));
    }
    

    // Command line number: 53
    #[test]
    #[cfg(feature="address-masking")]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(131070 as u32)];
        let result = runtime.call_exported_function("load32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8721 as u32)));
    }
    

    // Command line number: 56
    #[test]
    #[cfg(feature="address-masking")]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(131071 as u32)];
        let result = runtime.call_exported_function("load32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(34 as u32)));
    }
    

    // Command line number: 59
    #[test]
    #[cfg(feature="address-masking")]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(131071 as u32)];
        let result = runtime.call_exported_function("load16", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(34 as u32)));
    }
    

    // Command line number: 62
    #[test]
    #[cfg(feature="address-masking")]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(131071 as u32)];
        let result = runtime.call_exported_function("load8", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(34 as u32)));
    }
    

    // Command line number: 65
    #[test]
    #[cfg(feature="address-masking")]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967292 as u32)];
        let result = runtime.call_exported_function("load32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(571604974 as u32)));
    }
    

    // Command line number: 73
    #[test]
    #[cfg(feature="address-masking")]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("load64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(578437695752307201 as u64)));
    }
    

    // Command line number: 79
    #[test]
    #[cfg(feature="address-masking")]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(131064 as u32)];
        let result = runtime.call_exported_function("load64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2455024673282112426 as u64)));
    }
    

    // Command line number: 85
    #[test]
    #[cfg(feature="address-masking")]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(131065 as u32)];
        let result = runtime.call_exported_function("load64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9589940130008251 as u64)));
    }
    

    // Command line number: 91
    #[test]
    #[cfg(feature="address-masking")]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(131070 as u32)];
        let result = runtime.call_exported_function("load64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(8721 as u64)));
    }
    

    // Command line number: 97
    #[test]
    #[cfg(feature="address-masking")]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(131071 as u32)];
        let result = runtime.call_exported_function("load64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(34 as u64)));
    }
    

    // Command line number: 103
    #[test]
    #[cfg(feature="address-masking")]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(131072 as u32)];
        let result = runtime.call_exported_function("load64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(578437695752307201 as u64)));
    }
    

    // Command line number: 109
    #[test]
    #[cfg(feature="address-masking")]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(262142 as u32)];
        let result = runtime.call_exported_function("load64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(8721 as u64)));
    }
    

    // Command line number: 115
    #[test]
    #[cfg(feature="address-masking")]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(196606 as u32)];
        let result = runtime.call_exported_function("load64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10521 as u64)));
    }
    

    // Command line number: 125
    #[test]
    #[cfg(feature="address-masking")]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(131072 as u32),Immediate::Word(3735928559 as u32)];
        let result = runtime.call_exported_function("store32", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 126
    #[test]
    #[cfg(feature="address-masking")]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("load32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3735928559 as u32)));
    }
    

    // Command line number: 129
    #[test]
    #[cfg(feature="address-masking")]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(131070 as u32),Immediate::DoubleWord(1234605616436508552 as u64)];
        let result = runtime.call_exported_function("store64", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 130
    #[test]
    #[cfg(feature="address-masking")]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(131070 as u32)];
        let result = runtime.call_exported_function("load16", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(30600 as u32)));
    }
    

    // Command line number: 131
    #[test]
    #[cfg(feature="address-masking")]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(131070 as u32)];
        let result = runtime.call_exported_function("load64", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1234605616436508552 as u64)));
    }
    

    // Command line number: 134
    #[test]
    #[cfg(feature="address-masking")]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(131069 as u32),Immediate::Word(2864434397 as u32)];
        let result = runtime.call_exported_function("store32", args, None);
        assert_eq!(result, None);
    }
    

    // Command line number: 135
    #[test]
    #[cfg(feature="address-masking")]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(131069 as u32)];
        let result = runtime.call_exported_function("load32", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2864434397 as u32)));
    }
    
}

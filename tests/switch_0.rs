
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "switch.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 120
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("stmt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 121
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("stmt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 122
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("stmt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967294 as u32)));
    }
    

    // Command line number: 123
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("stmt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967293 as u32)));
    }
    

    // Command line number: 124
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32)];
        let result = runtime.call_exported_function("stmt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(100 as u32)));
    }
    

    // Command line number: 125
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32)];
        let result = runtime.call_exported_function("stmt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(101 as u32)));
    }
    

    // Command line number: 126
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("stmt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(102 as u32)));
    }
    

    // Command line number: 127
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(7 as u32)];
        let result = runtime.call_exported_function("stmt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(100 as u32)));
    }
    

    // Command line number: 128
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967286 as u32)];
        let result = runtime.call_exported_function("stmt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(102 as u32)));
    }
    

    // Command line number: 130
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("expr", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 131
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("expr", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551615 as u64)));
    }
    

    // Command line number: 132
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2 as u64)];
        let result = runtime.call_exported_function("expr", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551614 as u64)));
    }
    

    // Command line number: 133
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(3 as u64)];
        let result = runtime.call_exported_function("expr", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551613 as u64)));
    }
    

    // Command line number: 134
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6 as u64)];
        let result = runtime.call_exported_function("expr", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(101 as u64)));
    }
    

    // Command line number: 135
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(7 as u64)];
        let result = runtime.call_exported_function("expr", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551611 as u64)));
    }
    

    // Command line number: 136
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709551606 as u64)];
        let result = runtime.call_exported_function("expr", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(100 as u64)));
    }
    

    // Command line number: 138
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("arg", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(110 as u32)));
    }
    

    // Command line number: 139
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("arg", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(12 as u32)));
    }
    

    // Command line number: 140
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("arg", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4 as u32)));
    }
    

    // Command line number: 141
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("arg", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1116 as u32)));
    }
    

    // Command line number: 142
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4 as u32)];
        let result = runtime.call_exported_function("arg", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(118 as u32)));
    }
    

    // Command line number: 143
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32)];
        let result = runtime.call_exported_function("arg", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(20 as u32)));
    }
    

    // Command line number: 144
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(6 as u32)];
        let result = runtime.call_exported_function("arg", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(12 as u32)));
    }
    

    // Command line number: 145
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(7 as u32)];
        let result = runtime.call_exported_function("arg", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1124 as u32)));
    }
    

    // Command line number: 146
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8 as u32)];
        let result = runtime.call_exported_function("arg", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(126 as u32)));
    }
    

    // Command line number: 148
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("corner", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    
}

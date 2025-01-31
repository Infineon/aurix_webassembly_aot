
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.1.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 26
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2111029761 as u32),Immediate::Word(879215268 as u32),Immediate::Word(1967953261 as u32)];
        let result = runtime.call_exported_function("f32.no_fma", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1968345878 as u32)));
    }
    

    // Command line number: 27
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(838240978 as u32),Immediate::Word(2796592697 as u32),Immediate::Word(329493464 as u32)];
        let result = runtime.call_exported_function("f32.no_fma", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2569667420 as u32)));
    }
    

    // Command line number: 28
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1381446097 as u32),Immediate::Word(962187981 as u32),Immediate::Word(1155576972 as u32)];
        let result = runtime.call_exported_function("f32.no_fma", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1278680110 as u32)));
    }
    

    // Command line number: 29
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(999635965 as u32),Immediate::Word(3403528619 as u32),Immediate::Word(3222888213 as u32)];
        let result = runtime.call_exported_function("f32.no_fma", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3338748778 as u32)));
    }
    

    // Command line number: 30
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2123679707 as u32),Immediate::Word(2625733638 as u32),Immediate::Word(3500197619 as u32)];
        let result = runtime.call_exported_function("f32.no_fma", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3684076259 as u32)));
    }
    

    // Command line number: 31
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(7118716943724900052 as u64),Immediate::DoubleWord(6546073043412611735 as u64),Immediate::DoubleWord(18275705786238687882 as u64)];
        let result = runtime.call_exported_function("f64.no_fma", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9054581441422375136 as u64)));
    }
    

    // Command line number: 32
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(7984371788751700236 as u64),Immediate::DoubleWord(4021745400549737956 as u64),Immediate::DoubleWord(7188568268293775252 as u64)];
        let result = runtime.call_exported_function("f64.no_fma", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(7398962198428541884 as u64)));
    }
    

    // Command line number: 33
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1362668175782178275 as u64),Immediate::DoubleWord(18385570095786966502 as u64),Immediate::DoubleWord(5677031731722859914 as u64)];
        let result = runtime.call_exported_function("f64.no_fma", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(15141616602947129037 as u64)));
    }
    

    // Command line number: 34
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(12093403956019835987 as u64),Immediate::DoubleWord(15826077508588652458 as u64),Immediate::DoubleWord(4856562394320338043 as u64)];
        let result = runtime.call_exported_function("f64.no_fma", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4867219230351674394 as u64)));
    }
    

    // Command line number: 35
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4843589256781277081 as u64),Immediate::DoubleWord(7695653093478086834 as u64),Immediate::DoubleWord(16938438850771988744 as u64)];
        let result = runtime.call_exported_function("f64.no_fma", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(7932313162666085329 as u64)));
    }
    
}

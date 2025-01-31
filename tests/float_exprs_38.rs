
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.38.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 680
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4979303437048015281 as u64),Immediate::Word(1583535740 as u32)];
        let result = runtime.call_exported_function("no_demote_mixed_sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1758482618 as u32)));
    }
    

    // Command line number: 681
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13967600632962086462 as u64),Immediate::Word(1214924370 as u32)];
        let result = runtime.call_exported_function("no_demote_mixed_sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3468107136 as u32)));
    }
    

    // Command line number: 682
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13860263758943608426 as u64),Immediate::Word(969848030 as u32)];
        let result = runtime.call_exported_function("no_demote_mixed_sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3268174805 as u32)));
    }
    

    // Command line number: 683
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4364064588997139903 as u64),Immediate::Word(472962692 as u32)];
        let result = runtime.call_exported_function("no_demote_mixed_sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(612510881 as u32)));
    }
    

    // Command line number: 684
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4673175763235896759 as u64),Immediate::Word(1198952676 as u32)];
        let result = runtime.call_exported_function("no_demote_mixed_sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3339501185 as u32)));
    }
    
}

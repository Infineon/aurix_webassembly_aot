
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.37.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 661
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4183652368636204281 as u64),Immediate::Word(69183310 as u32)];
        let result = runtime.call_exported_function("no_demote_mixed_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(276467023 as u32)));
    }
    

    // Command line number: 662
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4773927428111915216 as u64),Immediate::Word(1387972204 as u32)];
        let result = runtime.call_exported_function("no_demote_mixed_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1392270651 as u32)));
    }
    

    // Command line number: 663
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4072985553596038423 as u64),Immediate::Word(2202918851 as u32)];
        let result = runtime.call_exported_function("no_demote_mixed_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(66813087 as u32)));
    }
    

    // Command line number: 664
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13740716732336801211 as u64),Immediate::Word(822392741 as u32)];
        let result = runtime.call_exported_function("no_demote_mixed_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3045484077 as u32)));
    }
    

    // Command line number: 665
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13742514716462174325 as u64),Immediate::Word(2870112826 as u32)];
        let result = runtime.call_exported_function("no_demote_mixed_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3048850075 as u32)));
    }
    

    // Command line number: 667
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(69183310 as u32),Immediate::DoubleWord(4183652368636204281 as u64)];
        let result = runtime.call_exported_function("no_demote_mixed_add_commuted", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(276467023 as u32)));
    }
    

    // Command line number: 668
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1387972204 as u32),Immediate::DoubleWord(4773927428111915216 as u64)];
        let result = runtime.call_exported_function("no_demote_mixed_add_commuted", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1392270651 as u32)));
    }
    

    // Command line number: 669
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2202918851 as u32),Immediate::DoubleWord(4072985553596038423 as u64)];
        let result = runtime.call_exported_function("no_demote_mixed_add_commuted", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(66813087 as u32)));
    }
    

    // Command line number: 670
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(822392741 as u32),Immediate::DoubleWord(13740716732336801211 as u64)];
        let result = runtime.call_exported_function("no_demote_mixed_add_commuted", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3045484077 as u32)));
    }
    

    // Command line number: 671
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2870112826 as u32),Immediate::DoubleWord(13742514716462174325 as u64)];
        let result = runtime.call_exported_function("no_demote_mixed_add_commuted", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3048850075 as u32)));
    }
    
}

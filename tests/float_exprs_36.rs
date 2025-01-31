
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.36.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 638
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("no_fold_promote_demote", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 639
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("no_fold_promote_demote", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 640
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("no_fold_promote_demote", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 641
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("no_fold_promote_demote", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 642
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("no_fold_promote_demote", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 643
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388607 as u32)];
        let result = runtime.call_exported_function("no_fold_promote_demote", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388607 as u32)));
    }
    

    // Command line number: 644
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872255 as u32)];
        let result = runtime.call_exported_function("no_fold_promote_demote", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872255 as u32)));
    }
    

    // Command line number: 645
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("no_fold_promote_demote", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 646
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("no_fold_promote_demote", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 647
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("no_fold_promote_demote", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 648
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("no_fold_promote_demote", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    

    // Command line number: 649
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("no_fold_promote_demote", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 650
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("no_fold_promote_demote", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    
}

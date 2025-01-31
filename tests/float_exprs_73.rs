
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.73.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 1815
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3428799709 as u32),Immediate::Word(2733489079 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_div_sqrts", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1816
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1339867611 as u32),Immediate::Word(1296568207 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_div_sqrts", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086203643 as u32)));
    }
    

    // Command line number: 1817
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65679161 as u32),Immediate::Word(1196795110 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_div_sqrts", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(498959746 as u32)));
    }
    

    // Command line number: 1818
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1566143010 as u32),Immediate::Word(816694667 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_div_sqrts", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1439333972 as u32)));
    }
    

    // Command line number: 1819
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(130133331 as u32),Immediate::Word(208189588 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_div_sqrts", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1025844032 as u32)));
    }
    

    // Command line number: 1821
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(10629913473787695463 as u64),Immediate::DoubleWord(12991130264919696663 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_sqrts", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1822
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1966780663211935584 as u64),Immediate::DoubleWord(7043916066229883379 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_sqrts", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2068364230648818889 as u64)));
    }
    

    // Command line number: 1823
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6965599900716272009 as u64),Immediate::DoubleWord(4118781927977980600 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_sqrts", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(6030491425828883991 as u64)));
    }
    

    // Command line number: 1824
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(962551478168675351 as u64),Immediate::DoubleWord(5918292176617055751 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_sqrts", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2129092583060403799 as u64)));
    }
    

    // Command line number: 1825
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1056821405580891413 as u64),Immediate::DoubleWord(8865548665903786673 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_sqrts", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(702724841785532050 as u64)));
    }
    
}

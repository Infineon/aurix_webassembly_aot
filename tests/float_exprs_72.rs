
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.72.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 1793
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(24047316 as u32),Immediate::Word(2517821717 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_mul_sqrts", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1794
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(295749258 as u32),Immediate::Word(803416494 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_mul_sqrts", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(549395357 as u32)));
    }
    

    // Command line number: 1795
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(329708528 as u32),Immediate::Word(1120042892 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_mul_sqrts", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(724841268 as u32)));
    }
    

    // Command line number: 1796
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1916535951 as u32),Immediate::Word(994115420 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_mul_sqrts", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1455324620 as u32)));
    }
    

    // Command line number: 1797
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(598482176 as u32),Immediate::Word(990534933 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_mul_sqrts", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(794443079 as u32)));
    }
    

    // Command line number: 1799
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(10974446854152441278 as u64),Immediate::DoubleWord(13797896470155574122 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_mul_sqrts", args, Some(ValueSize::DoubleWord));
        assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());
    }
    

    // Command line number: 1800
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1712959863583927241 as u64),Immediate::DoubleWord(2792003944717853898 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_mul_sqrts", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2252469008297979510 as u64)));
    }
    

    // Command line number: 1801
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4208351758938831157 as u64),Immediate::DoubleWord(497361189565243603 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_mul_sqrts", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2352856462697312748 as u64)));
    }
    

    // Command line number: 1802
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2976792199849816182 as u64),Immediate::DoubleWord(2030444188042608984 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_mul_sqrts", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2503613111125550255 as u64)));
    }
    

    // Command line number: 1803
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4717634334691577101 as u64),Immediate::DoubleWord(6919598687070693285 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_mul_sqrts", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5818898567902921651 as u64)));
    }
    
}


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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.70.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 1750
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2690073844 as u32),Immediate::Word(2809448479 as u32),Immediate::Word(3608905030 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_add_divs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(264862203 as u32)));
    }
    

    // Command line number: 1751
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2830184964 as u32),Immediate::Word(530019033 as u32),Immediate::Word(3623253973 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_add_divs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(272108594 as u32)));
    }
    

    // Command line number: 1752
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2365787800 as u32),Immediate::Word(245111369 as u32),Immediate::Word(3952003433 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_add_divs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1753
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(982471119 as u32),Immediate::Word(1045692415 as u32),Immediate::Word(37216954 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_add_divs", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2073319791 as u32)));
    }
    

    // Command line number: 1755
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(15770585325769044278 as u64),Immediate::DoubleWord(6564157675451289455 as u64),Immediate::DoubleWord(8712254759989822359 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_add_divs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2458462832069881218 as u64)));
    }
    

    // Command line number: 1756
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14069844870254671283 as u64),Immediate::DoubleWord(4634122757084803708 as u64),Immediate::DoubleWord(9524897388132352235 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_add_divs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9152039358940941283 as u64)));
    }
    

    // Command line number: 1757
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9479648703296052622 as u64),Immediate::DoubleWord(214573661502224386 as u64),Immediate::DoubleWord(6877551490107761946 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_add_divs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 1758
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6019502660029506228 as u64),Immediate::DoubleWord(15316513033818836241 as u64),Immediate::DoubleWord(4039967192182502935 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_add_divs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(15883525310425977300 as u64)));
    }
    

    // Command line number: 1759
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(10555667216821129841 as u64),Immediate::DoubleWord(1207418919037494573 as u64),Immediate::DoubleWord(4296330408727545598 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_add_divs", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10866511466898347555 as u64)));
    }
    
}

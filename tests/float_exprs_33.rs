
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.33.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 593
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3538825650 as u32),Immediate::Word(1315641462 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_div_mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3538825649 as u32)));
    }
    

    // Command line number: 594
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2777664539 as u32),Immediate::Word(3062588018 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_div_mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2777664540 as u32)));
    }
    

    // Command line number: 597
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(250394049 as u32),Immediate::Word(1296755844 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_div_mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(250394050 as u32)));
    }
    

    // Command line number: 599
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(665690489208775809 as u64),Immediate::DoubleWord(14660005164454413124 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(665690577722002880 as u64)));
    }
    

    // Command line number: 600
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(10617267697387344269 as u64),Immediate::DoubleWord(4370684778829606254 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10617267697387344270 as u64)));
    }
    

    // Command line number: 601
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13179336828827425934 as u64),Immediate::DoubleWord(6536345148565138764 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13179336828827425933 as u64)));
    }
    

    // Command line number: 602
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(12582623625647949669 as u64),Immediate::DoubleWord(15106746174896642041 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(12582623625647949668 as u64)));
    }
    

    // Command line number: 603
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(16624217782795067216 as u64),Immediate::DoubleWord(9062205521150975866 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(16624217782795067215 as u64)));
    }
    
}

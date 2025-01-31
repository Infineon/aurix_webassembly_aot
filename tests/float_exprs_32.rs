
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.32.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 572
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3672556237 as u32),Immediate::Word(674649243 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_mul_div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3672556236 as u32)));
    }
    

    // Command line number: 574
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2817764014 as u32),Immediate::Word(3620253920 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_mul_div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2817764013 as u32)));
    }
    

    // Command line number: 575
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1507152519 as u32),Immediate::Word(3723483599 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_mul_div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1507152518 as u32)));
    }
    

    // Command line number: 578
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(10062123074470382106 as u64),Immediate::DoubleWord(12910565991996555404 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_mul_div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10062123074470422078 as u64)));
    }
    

    // Command line number: 579
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6340937764684870564 as u64),Immediate::DoubleWord(7244253720027059594 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_mul_div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(6340937764684870565 as u64)));
    }
    

    // Command line number: 580
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14905228263410157971 as u64),Immediate::DoubleWord(11346251643264732732 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_mul_div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14905228263410157970 as u64)));
    }
    

    // Command line number: 581
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(3862352046163709780 as u64),Immediate::DoubleWord(531112307488385734 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_mul_div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3862079437827029803 as u64)));
    }
    

    // Command line number: 582
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(16807035693954817236 as u64),Immediate::DoubleWord(12360222454864961326 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_mul_div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(16807035693954817237 as u64)));
    }
    
}

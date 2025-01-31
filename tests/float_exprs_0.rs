
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 11
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13369472591878845359 as u64),Immediate::DoubleWord(7598224971858294334 as u64),Immediate::DoubleWord(7009968021366006149 as u64)];
        let result = runtime.call_exported_function("f64.no_contraction", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(16360919150252594323 as u64)));
    }
    

    // Command line number: 12
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4845207016438394692 as u64),Immediate::DoubleWord(3163224970157846858 as u64),Immediate::DoubleWord(3251145870828527841 as u64)];
        let result = runtime.call_exported_function("f64.no_contraction", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3401457070760597396 as u64)));
    }
    

    // Command line number: 13
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(11159707324127586240 as u64),Immediate::DoubleWord(7011538096610110295 as u64),Immediate::DoubleWord(4140382893275160737 as u64)];
        let result = runtime.call_exported_function("f64.no_contraction", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13564076370790560102 as u64)));
    }
    

    // Command line number: 14
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4300281701552927458 as u64),Immediate::DoubleWord(13379479906516703876 as u64),Immediate::DoubleWord(3629658278272971302 as u64)];
        let result = runtime.call_exported_function("f64.no_contraction", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13072631228492738408 as u64)));
    }
    

    // Command line number: 15
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9554523352352050493 as u64),Immediate::DoubleWord(18042841594766434431 as u64),Immediate::DoubleWord(4368037109959396445 as u64)];
        let result = runtime.call_exported_function("f64.no_contraction", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4544162191519938727 as u64)));
    }
    
}

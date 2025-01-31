
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.68.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 1703
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3875242260 as u32),Immediate::Word(3086869257 as u32),Immediate::Word(3301317576 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_div_div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3911440926 as u32)));
    }
    

    // Command line number: 1704
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(485052055 as u32),Immediate::Word(1996083391 as u32),Immediate::Word(2276616712 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_div_div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1705
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1430470604 as u32),Immediate::Word(186144382 as u32),Immediate::Word(1953564780 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_div_div", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 1709
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6128077243319875447 as u64),Immediate::DoubleWord(7240092044185667120 as u64),Immediate::DoubleWord(10312472494987686942 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(16236150182064455170 as u64)));
    }
    

    // Command line number: 1710
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(17395933367696573535 as u64),Immediate::DoubleWord(4478922858584402707 as u64),Immediate::DoubleWord(6032094754408482817 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(16098470347548634769 as u64)));
    }
    

    // Command line number: 1711
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13843263185226986279 as u64),Immediate::DoubleWord(17796742619038211051 as u64),Immediate::DoubleWord(5375701731263473827 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(44472927 as u64)));
    }
    

    // Command line number: 1712
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(17547288444310957340 as u64),Immediate::DoubleWord(911654786857739111 as u64),Immediate::DoubleWord(8937284546802896640 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18442240474082181120 as u64)));
    }
    

    // Command line number: 1713
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9835707468114203513 as u64),Immediate::DoubleWord(1924400690116523912 as u64),Immediate::DoubleWord(13208934041167870811 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_div_div", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3916014548332337260 as u64)));
    }
    
}

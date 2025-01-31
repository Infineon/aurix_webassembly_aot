
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.25.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 437
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3255678581 as u32),Immediate::Word(1210720351 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_to_hypot", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1210720352 as u32)));
    }
    

    // Command line number: 439
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1743351192697472785 as u64),Immediate::DoubleWord(2202602366606243153 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_to_hypot", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2202599296765198670 as u64)));
    }
    

    // Command line number: 440
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6389333765198869657 as u64),Immediate::DoubleWord(15677343373020056630 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_to_hypot", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(6453971336171062178 as u64)));
    }
    

    // Command line number: 441
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2195337108264055819 as u64),Immediate::DoubleWord(10384237061545402288 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_to_hypot", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2195504818343116800 as u64)));
    }
    

    // Command line number: 442
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(11486582223361829725 as u64),Immediate::DoubleWord(1308532122426122043 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_to_hypot", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2263210186506929210 as u64)));
    }
    

    // Command line number: 443
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1591440107418864392 as u64),Immediate::DoubleWord(11515806374387309036 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_to_hypot", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2292434337532533215 as u64)));
    }
    
}

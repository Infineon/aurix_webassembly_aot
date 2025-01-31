
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.90.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 2430
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1289068416 as u32),Immediate::Word(1203982336 as u32),Immediate::Word(980151802 as u32)];
        let result = runtime.call_exported_function("f32.division_by_small_number", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1230570368 as u32)));
    }
    

    // Command line number: 2431
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4727288602252279808 as u64),Immediate::DoubleWord(4681608360884174848 as u64),Immediate::DoubleWord(4561440258104740754 as u64)];
        let result = runtime.call_exported_function("f64.division_by_small_number", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4695882709507797376 as u64)));
    }
    
}

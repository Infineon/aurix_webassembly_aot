
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.53.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 1244
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9007199254740992 as u64),Immediate::DoubleWord(4607182418800017407 as u64),Immediate::DoubleWord(4611686018427387904 as u64),Immediate::DoubleWord(4602678819172646912 as u64)];
        let result = runtime.call_exported_function("thepast0", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9007199254740991 as u64)));
    }
    

    // Command line number: 1245
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4363988038922010624 as u64),Immediate::DoubleWord(4607182418800017407 as u64),Immediate::DoubleWord(4363988038922010624 as u64)];
        let result = runtime.call_exported_function("thepast1", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13348669295526150144 as u64)));
    }
    

    // Command line number: 1246
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(16777216 as u32),Immediate::Word(1056964608 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("thepast2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    
}

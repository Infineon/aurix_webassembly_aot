
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.30.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 530
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(677030386 as u32),Immediate::Word(2998136214 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_add_sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(677380096 as u32)));
    }
    

    // Command line number: 531
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3025420904 as u32),Immediate::Word(913921807 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_add_sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3025420912 as u32)));
    }
    

    // Command line number: 532
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3908960888 as u32),Immediate::Word(4063404061 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_add_sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3909091328 as u32)));
    }
    

    // Command line number: 533
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(415467473 as u32),Immediate::Word(602055819 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_add_sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(415236096 as u32)));
    }
    

    // Command line number: 534
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2307650739 as u32),Immediate::Word(2511328013 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_add_sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2315255808 as u32)));
    }
    

    // Command line number: 536
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9894695622864460712 as u64),Immediate::DoubleWord(747900745977727688 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_add_sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9894695622864404480 as u64)));
    }
    

    // Command line number: 537
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2152218683357821298 as u64),Immediate::DoubleWord(2238360073507307376 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_add_sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2152218683357790208 as u64)));
    }
    

    // Command line number: 538
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13697521605206502242 as u64),Immediate::DoubleWord(13818850255013161909 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_add_sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13697521605247238144 as u64)));
    }
    

    // Command line number: 539
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(12298280617237492384 as u64),Immediate::DoubleWord(3233965342858558382 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_add_sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(12298280617463775232 as u64)));
    }
    

    // Command line number: 540
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(11043298296128683688 as u64),Immediate::DoubleWord(11182857345495207592 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_add_sub", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(11043298296775835648 as u64)));
    }
    
}

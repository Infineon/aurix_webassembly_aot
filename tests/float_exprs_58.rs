
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.58.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 1343
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(684135946 as u32),Immediate::Word(744319693 as u32)];
        let result = runtime.call_exported_function("f32.no_algebraic_factoring", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2571075368 as u32)));
    }
    

    // Command line number: 1344
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3560929481 as u32),Immediate::Word(3496840229 as u32)];
        let result = runtime.call_exported_function("f32.no_algebraic_factoring", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1762604185 as u32)));
    }
    

    // Command line number: 1345
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(968841772 as u32),Immediate::Word(3106497100 as u32)];
        let result = runtime.call_exported_function("f32.no_algebraic_factoring", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(870712803 as u32)));
    }
    

    // Command line number: 1346
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(697514723 as u32),Immediate::Word(2834753933 as u32)];
        let result = runtime.call_exported_function("f32.no_algebraic_factoring", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(327914662 as u32)));
    }
    

    // Command line number: 1347
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1498230729 as u32),Immediate::Word(3650453580 as u32)];
        let result = runtime.call_exported_function("f32.no_algebraic_factoring", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4080583891 as u32)));
    }
    

    // Command line number: 1349
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6212515167506370409 as u64),Immediate::DoubleWord(15348474890798978273 as u64)];
        let result = runtime.call_exported_function("f64.no_algebraic_factoring", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(7818515589337550196 as u64)));
    }
    

    // Command line number: 1350
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(15222970140370015722 as u64),Immediate::DoubleWord(15325207139996136125 as u64)];
        let result = runtime.call_exported_function("f64.no_algebraic_factoring", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(16819892485880140289 as u64)));
    }
    

    // Command line number: 1351
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4797439202963874050 as u64),Immediate::DoubleWord(14009643534571442918 as u64)];
        let result = runtime.call_exported_function("f64.no_algebraic_factoring", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4987747999326390045 as u64)));
    }
    

    // Command line number: 1352
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14653559129294038194 as u64),Immediate::DoubleWord(14581996260169223461 as u64)];
        let result = runtime.call_exported_function("f64.no_algebraic_factoring", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(6253339631158964222 as u64)));
    }
    

    // Command line number: 1353
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(12768321634751930140 as u64),Immediate::DoubleWord(12767602092732820937 as u64)];
        let result = runtime.call_exported_function("f64.no_algebraic_factoring", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2473652960990319032 as u64)));
    }
    
}

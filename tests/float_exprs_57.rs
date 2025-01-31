
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.57.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 1319
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2759308231 as u32),Immediate::Word(618704988 as u32)];
        let result = runtime.call_exported_function("f32.no_algebraic_factoring", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2315864577 as u32)));
    }
    

    // Command line number: 1320
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3415653214 as u32),Immediate::Word(1274676302 as u32)];
        let result = runtime.call_exported_function("f32.no_algebraic_factoring", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3625675853 as u32)));
    }
    

    // Command line number: 1321
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1446924633 as u32),Immediate::Word(3607373982 as u32)];
        let result = runtime.call_exported_function("f32.no_algebraic_factoring", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4000155759 as u32)));
    }
    

    // Command line number: 1322
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1212067608 as u32),Immediate::Word(3278094810 as u32)];
        let result = runtime.call_exported_function("f32.no_algebraic_factoring", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1359874131 as u32)));
    }
    

    // Command line number: 1323
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3278732464 as u32),Immediate::Word(3379389272 as u32)];
        let result = runtime.call_exported_function("f32.no_algebraic_factoring", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3546030359 as u32)));
    }
    

    // Command line number: 1325
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2467435761933928117 as u64),Immediate::DoubleWord(2526113756828458004 as u64)];
        let result = runtime.call_exported_function("f64.no_algebraic_factoring", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9668435399096543331 as u64)));
    }
    

    // Command line number: 1326
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2911983657790464931 as u64),Immediate::DoubleWord(2814431682419759911 as u64)];
        let result = runtime.call_exported_function("f64.no_algebraic_factoring", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1217162942843921803 as u64)));
    }
    

    // Command line number: 1327
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(12131637044948792058 as u64),Immediate::DoubleWord(12170782965730311956 as u64)];
        let result = runtime.call_exported_function("f64.no_algebraic_factoring", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10511676135434922533 as u64)));
    }
    

    // Command line number: 1328
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14639789466354372633 as u64),Immediate::DoubleWord(5456963169336729236 as u64)];
        let result = runtime.call_exported_function("f64.no_algebraic_factoring", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(15530333405173431543 as u64)));
    }
    

    // Command line number: 1329
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5121779675912507154 as u64),Immediate::DoubleWord(14237286623175920791 as u64)];
        let result = runtime.call_exported_function("f64.no_algebraic_factoring", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5636689734063865714 as u64)));
    }
    
}

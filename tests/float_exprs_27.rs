
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.27.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 467
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(708147349 as u32)];
        let result = runtime.call_exported_function("f32.no_approximate_reciprocal_sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1243088746 as u32)));
    }
    

    // Command line number: 468
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1005852643 as u32)];
        let result = runtime.call_exported_function("f32.no_approximate_reciprocal_sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1094279611 as u32)));
    }
    

    // Command line number: 469
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(517799246 as u32)];
        let result = runtime.call_exported_function("f32.no_approximate_reciprocal_sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1338168541 as u32)));
    }
    

    // Command line number: 470
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(704281251 as u32)];
        let result = runtime.call_exported_function("f32.no_approximate_reciprocal_sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1245118689 as u32)));
    }
    

    // Command line number: 471
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(347001813 as u32)];
        let result = runtime.call_exported_function("f32.no_approximate_reciprocal_sqrt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1423641701 as u32)));
    }
    

    // Command line number: 473
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(8611259114887405475 as u64)];
        let result = runtime.call_exported_function("f64.no_fuse_reciprocal_sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2604695339663988000 as u64)));
    }
    

    // Command line number: 474
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6008428610859539631 as u64)];
        let result = runtime.call_exported_function("f64.no_fuse_reciprocal_sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3906084647186679832 as u64)));
    }
    

    // Command line number: 475
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5077495674931581012 as u64)];
        let result = runtime.call_exported_function("f64.no_fuse_reciprocal_sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4371518865190387497 as u64)));
    }
    

    // Command line number: 476
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(7616219057857077123 as u64)];
        let result = runtime.call_exported_function("f64.no_fuse_reciprocal_sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3102407657946187309 as u64)));
    }
    

    // Command line number: 477
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5267858027841559467 as u64)];
        let result = runtime.call_exported_function("f64.no_fuse_reciprocal_sqrt", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4276321761661248681 as u64)));
    }
    
}

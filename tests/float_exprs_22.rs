
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.22.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 361
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(598526039 as u32),Immediate::Word(4072603010 as u32),Immediate::Word(2166864805 as u32),Immediate::Word(3802968051 as u32)];
        let result = runtime.call_exported_function("f32.no_reassociate_mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3152274558 as u32)));
    }
    

    // Command line number: 362
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(666201298 as u32),Immediate::Word(3678968917 as u32),Immediate::Word(2879732647 as u32),Immediate::Word(1703934016 as u32)];
        let result = runtime.call_exported_function("f32.no_reassociate_mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1439591542 as u32)));
    }
    

    // Command line number: 363
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(191948150 as u32),Immediate::Word(1717012201 as u32),Immediate::Word(3682645872 as u32),Immediate::Word(3713382507 as u32)];
        let result = runtime.call_exported_function("f32.no_reassociate_mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1814709127 as u32)));
    }
    

    // Command line number: 365
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(10830726381612138752 as u64),Immediate::DoubleWord(18293529276079591087 as u64),Immediate::DoubleWord(12137662286027993114 as u64),Immediate::DoubleWord(16821646709291069775 as u64)];
        let result = runtime.call_exported_function("f64.no_reassociate_mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(7368793799369880819 as u64)));
    }
    

    // Command line number: 366
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6653164799371160764 as u64),Immediate::DoubleWord(2285295038358358170 as u64),Immediate::DoubleWord(9783304669150272403 as u64),Immediate::DoubleWord(16266005085991502709 as u64)];
        let result = runtime.call_exported_function("f64.no_reassociate_mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2720645287366687760 as u64)));
    }
    

    // Command line number: 367
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2352911459797566465 as u64),Immediate::DoubleWord(17379873157362463143 as u64),Immediate::DoubleWord(1179129869275935356 as u64),Immediate::DoubleWord(14228398113747850351 as u64)];
        let result = runtime.call_exported_function("f64.no_reassociate_mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2873103656912958703 as u64)));
    }
    

    // Command line number: 368
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(7724499817746503804 as u64),Immediate::DoubleWord(2704005046640722176 as u64),Immediate::DoubleWord(5612860422806321751 as u64),Immediate::DoubleWord(13727818095548724091 as u64)];
        let result = runtime.call_exported_function("f64.no_reassociate_mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(15948568678460814092 as u64)));
    }
    

    // Command line number: 369
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(3553622953022765407 as u64),Immediate::DoubleWord(1044040287824900408 as u64),Immediate::DoubleWord(17112762794520509437 as u64),Immediate::DoubleWord(11134095486440145773 as u64)];
        let result = runtime.call_exported_function("f64.no_reassociate_mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(576919682754813073 as u64)));
    }
    
}

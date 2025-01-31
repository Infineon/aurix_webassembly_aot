
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.21.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 340
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3585064686 as u32),Immediate::Word(1354934024 as u32),Immediate::Word(3612934982 as u32),Immediate::Word(3557837641 as u32)];
        let result = runtime.call_exported_function("f32.no_reassociate_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3614520891 as u32)));
    }
    

    // Command line number: 341
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(997006780 as u32),Immediate::Word(3156314493 as u32),Immediate::Word(1031916275 as u32),Immediate::Word(3157700435 as u32)];
        let result = runtime.call_exported_function("f32.no_reassociate_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1027365261 as u32)));
    }
    

    // Command line number: 342
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3506363549 as u32),Immediate::Word(3562765939 as u32),Immediate::Word(1440782572 as u32),Immediate::Word(1388583643 as u32)];
        let result = runtime.call_exported_function("f32.no_reassociate_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1439168977 as u32)));
    }
    

    // Command line number: 343
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1460378878 as u32),Immediate::Word(1481791683 as u32),Immediate::Word(3506843934 as u32),Immediate::Word(1493913729 as u32)];
        let result = runtime.call_exported_function("f32.no_reassociate_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1497931771 as u32)));
    }
    

    // Command line number: 344
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1975099005 as u32),Immediate::Word(4120668550 as u32),Immediate::Word(1947708458 as u32),Immediate::Word(4008073260 as u32)];
        let result = runtime.call_exported_function("f32.no_reassociate_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1958779787 as u32)));
    }
    

    // Command line number: 345
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(17619937326421449126 as u64),Immediate::DoubleWord(8424880666975634327 as u64),Immediate::DoubleWord(8461713040394112626 as u64),Immediate::DoubleWord(17692076622886930107 as u64)];
        let result = runtime.call_exported_function("f64.no_reassociate_add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(17689770886425413754 as u64)));
    }
    

    // Command line number: 346
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2161744272815763681 as u64),Immediate::DoubleWord(2160815018984030177 as u64),Immediate::DoubleWord(11389452991481170854 as u64),Immediate::DoubleWord(11158554735757873927 as u64)];
        let result = runtime.call_exported_function("f64.no_reassociate_add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(11367213592018398582 as u64)));
    }
    

    // Command line number: 347
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(15816220208145029204 as u64),Immediate::DoubleWord(6443786499090728432 as u64),Immediate::DoubleWord(15798639273395365185 as u64),Immediate::DoubleWord(6395820899158300605 as u64)];
        let result = runtime.call_exported_function("f64.no_reassociate_add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(15816713260997571051 as u64)));
    }
    

    // Command line number: 348
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(12406188505172681730 as u64),Immediate::DoubleWord(3227622722685619614 as u64),Immediate::DoubleWord(12653209142287077985 as u64),Immediate::DoubleWord(3439058911346459774 as u64)];
        let result = runtime.call_exported_function("f64.no_reassociate_add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(3437283564188778523 as u64)));
    }
    

    // Command line number: 349
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(16720963389015391005 as u64),Immediate::DoubleWord(16597092572968550980 as u64),Immediate::DoubleWord(7518944085377596897 as u64),Immediate::DoubleWord(16733407756820198530 as u64)];
        let result = runtime.call_exported_function("f64.no_reassociate_add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(7516931113564586278 as u64)));
    }
    
}

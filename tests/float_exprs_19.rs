
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.19.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 300
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2249624147 as u32),Immediate::Word(2678828342 as u32),Immediate::Word(95319815 as u32)];
        let result = runtime.call_exported_function("f32.no_regroup_div_mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(538190437 as u32)));
    }
    

    // Command line number: 301
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3978470300 as u32),Immediate::Word(2253997363 as u32),Immediate::Word(3824852100 as u32)];
        let result = runtime.call_exported_function("f32.no_regroup_div_mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 302
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3350590135 as u32),Immediate::Word(3042588643 as u32),Immediate::Word(2186448635 as u32)];
        let result = runtime.call_exported_function("f32.no_regroup_div_mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4206661932 as u32)));
    }
    

    // Command line number: 303
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2430706172 as u32),Immediate::Word(1685220483 as u32),Immediate::Word(1642018044 as u32)];
        let result = runtime.call_exported_function("f32.no_regroup_div_mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2473922297 as u32)));
    }
    

    // Command line number: 304
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2011387707 as u32),Immediate::Word(1274956446 as u32),Immediate::Word(3811596788 as u32)];
        let result = runtime.call_exported_function("f32.no_regroup_div_mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3768838261 as u32)));
    }
    

    // Command line number: 305
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2703215631877943472 as u64),Immediate::DoubleWord(13295603997208052007 as u64),Immediate::DoubleWord(1719211436532588593 as u64)];
        let result = runtime.call_exported_function("f64.no_regroup_div_mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14279677686886620461 as u64)));
    }
    

    // Command line number: 306
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(6126139291059848917 as u64),Immediate::DoubleWord(2596039250849921421 as u64),Immediate::DoubleWord(17423258659719899654 as u64)];
        let result = runtime.call_exported_function("f64.no_regroup_div_mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 307
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2451868557331674239 as u64),Immediate::DoubleWord(8672326445062988097 as u64),Immediate::DoubleWord(2593279393835739385 as u64)];
        let result = runtime.call_exported_function("f64.no_regroup_div_mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9218868437227405312 as u64)));
    }
    

    // Command line number: 308
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(15994259208199847538 as u64),Immediate::DoubleWord(16584156163346075677 as u64),Immediate::DoubleWord(17596923907238870430 as u64)];
        let result = runtime.call_exported_function("f64.no_regroup_div_mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14981548491626301009 as u64)));
    }
    

    // Command line number: 309
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1912002771029783751 as u64),Immediate::DoubleWord(655387110450354003 as u64),Immediate::DoubleWord(10060746190138762841 as u64)];
        let result = runtime.call_exported_function("f64.no_regroup_div_mul", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10953754119023888080 as u64)));
    }
    
}


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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "float_exprs.31.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 551
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3291029084 as u32),Immediate::Word(1137280182 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_sub_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3291029085 as u32)));
    }
    

    // Command line number: 552
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2287045896 as u32),Immediate::Word(272248696 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_sub_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2287075328 as u32)));
    }
    

    // Command line number: 553
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1285466516 as u32),Immediate::Word(1361849144 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_sub_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1285466624 as u32)));
    }
    

    // Command line number: 554
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(740009747 as u32),Immediate::Word(2989707904 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_sub_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(740007936 as u32)));
    }
    

    // Command line number: 555
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1041827798 as u32),Immediate::Word(3335914317 as u32)];
        let result = runtime.call_exported_function("f32.no_fold_sub_add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1041891328 as u32)));
    }
    

    // Command line number: 557
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5758126085282503565 as u64),Immediate::DoubleWord(14997141603873875659 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_sub_add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5758126085282503568 as u64)));
    }
    

    // Command line number: 558
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1609380455481879691 as u64),Immediate::DoubleWord(1695875689930159213 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_sub_add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1609380455482130432 as u64)));
    }
    

    // Command line number: 559
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(5738179408840599949 as u64),Immediate::DoubleWord(15186085143903012996 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_sub_add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(5738148875223433216 as u64)));
    }
    

    // Command line number: 560
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4492841470376833908 as u64),Immediate::DoubleWord(13773869588765591068 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_sub_add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4492841470376837120 as u64)));
    }
    

    // Command line number: 561
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2955729038738127538 as u64),Immediate::DoubleWord(12208627806665035010 as u64)];
        let result = runtime.call_exported_function("f64.no_fold_sub_add", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2955729038738127552 as u64)));
    }
    
}

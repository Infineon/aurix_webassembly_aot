
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "i32.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 35
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 36
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 37
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967294 as u32)));
    }
    

    // Command line number: 38
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 39
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 40
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483647 as u32)));
    }
    

    // Command line number: 41
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 42
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1073741823 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("add", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1073741824 as u32)));
    }
    

    // Command line number: 44
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 45
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 46
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 47
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 48
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483647 as u32)));
    }
    

    // Command line number: 49
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 50
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1073741823 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("sub", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1073741824 as u32)));
    }
    

    // Command line number: 52
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 53
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 54
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 55
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(268435456 as u32),Immediate::Word(4096 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 56
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 57
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 58
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 59
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(19088743 as u32),Immediate::Word(1985229328 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(898528368 as u32)));
    }
    

    // Command line number: 60
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("mul", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 66
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("div_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 67
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("div_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 68
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("div_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 69
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("div_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 70
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("div_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3221225472 as u32)));
    }
    

    // Command line number: 71
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(1000 as u32)];
        let result = runtime.call_exported_function("div_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4292819813 as u32)));
    }
    

    // Command line number: 72
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("div_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 73
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967291 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("div_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967294 as u32)));
    }
    

    // Command line number: 74
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32),Immediate::Word(4294967294 as u32)];
        let result = runtime.call_exported_function("div_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967294 as u32)));
    }
    

    // Command line number: 75
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967291 as u32),Immediate::Word(4294967294 as u32)];
        let result = runtime.call_exported_function("div_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 76
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(7 as u32),Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("div_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 77
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967289 as u32),Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("div_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967294 as u32)));
    }
    

    // Command line number: 78
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(7 as u32),Immediate::Word(4294967293 as u32)];
        let result = runtime.call_exported_function("div_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967294 as u32)));
    }
    

    // Command line number: 79
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967289 as u32),Immediate::Word(4294967293 as u32)];
        let result = runtime.call_exported_function("div_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 80
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(11 as u32),Immediate::Word(5 as u32)];
        let result = runtime.call_exported_function("div_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 81
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(17 as u32),Immediate::Word(7 as u32)];
        let result = runtime.call_exported_function("div_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 85
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("div_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 86
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("div_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 87
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("div_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 88
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("div_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 89
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("div_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1073741824 as u32)));
    }
    

    // Command line number: 90
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2414874608 as u32),Immediate::Word(65537 as u32)];
        let result = runtime.call_exported_function("div_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(36847 as u32)));
    }
    

    // Command line number: 91
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(1000 as u32)];
        let result = runtime.call_exported_function("div_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483 as u32)));
    }
    

    // Command line number: 92
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("div_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 93
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967291 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("div_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483645 as u32)));
    }
    

    // Command line number: 94
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32),Immediate::Word(4294967294 as u32)];
        let result = runtime.call_exported_function("div_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 95
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967291 as u32),Immediate::Word(4294967294 as u32)];
        let result = runtime.call_exported_function("div_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 96
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(7 as u32),Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("div_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 97
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(11 as u32),Immediate::Word(5 as u32)];
        let result = runtime.call_exported_function("div_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 98
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(17 as u32),Immediate::Word(7 as u32)];
        let result = runtime.call_exported_function("div_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 102
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("rem_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 103
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("rem_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 104
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("rem_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 105
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("rem_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 106
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("rem_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 107
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("rem_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 108
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("rem_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 109
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(1000 as u32)];
        let result = runtime.call_exported_function("rem_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294966649 as u32)));
    }
    

    // Command line number: 110
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("rem_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 111
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967291 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("rem_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 112
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32),Immediate::Word(4294967294 as u32)];
        let result = runtime.call_exported_function("rem_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 113
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967291 as u32),Immediate::Word(4294967294 as u32)];
        let result = runtime.call_exported_function("rem_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 114
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(7 as u32),Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("rem_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 115
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967289 as u32),Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("rem_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 116
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(7 as u32),Immediate::Word(4294967293 as u32)];
        let result = runtime.call_exported_function("rem_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 117
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967289 as u32),Immediate::Word(4294967293 as u32)];
        let result = runtime.call_exported_function("rem_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 118
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(11 as u32),Immediate::Word(5 as u32)];
        let result = runtime.call_exported_function("rem_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 119
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(17 as u32),Immediate::Word(7 as u32)];
        let result = runtime.call_exported_function("rem_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 123
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("rem_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 124
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("rem_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 125
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("rem_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 126
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("rem_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 127
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("rem_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 128
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2414874608 as u32),Immediate::Word(65537 as u32)];
        let result = runtime.call_exported_function("rem_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(32769 as u32)));
    }
    

    // Command line number: 129
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(1000 as u32)];
        let result = runtime.call_exported_function("rem_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(649 as u32)));
    }
    

    // Command line number: 130
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("rem_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 131
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967291 as u32),Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("rem_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 132
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(5 as u32),Immediate::Word(4294967294 as u32)];
        let result = runtime.call_exported_function("rem_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(5 as u32)));
    }
    

    // Command line number: 133
    #[test]
    fn test_82(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967291 as u32),Immediate::Word(4294967294 as u32)];
        let result = runtime.call_exported_function("rem_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967291 as u32)));
    }
    

    // Command line number: 134
    #[test]
    fn test_83(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(7 as u32),Immediate::Word(3 as u32)];
        let result = runtime.call_exported_function("rem_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 135
    #[test]
    fn test_84(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(11 as u32),Immediate::Word(5 as u32)];
        let result = runtime.call_exported_function("rem_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 136
    #[test]
    fn test_85(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(17 as u32),Immediate::Word(7 as u32)];
        let result = runtime.call_exported_function("rem_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3 as u32)));
    }
    

    // Command line number: 138
    #[test]
    fn test_86(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("and", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 139
    #[test]
    fn test_87(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("and", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 140
    #[test]
    fn test_88(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("and", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 141
    #[test]
    fn test_89(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("and", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 142
    #[test]
    fn test_90(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("and", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 143
    #[test]
    fn test_91(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("and", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483647 as u32)));
    }
    

    // Command line number: 144
    #[test]
    fn test_92(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4042326015 as u32),Immediate::Word(4294963440 as u32)];
        let result = runtime.call_exported_function("and", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4042322160 as u32)));
    }
    

    // Command line number: 145
    #[test]
    fn test_93(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("and", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 147
    #[test]
    fn test_94(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("or", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 148
    #[test]
    fn test_95(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("or", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 149
    #[test]
    fn test_96(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("or", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 150
    #[test]
    fn test_97(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("or", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 151
    #[test]
    fn test_98(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("or", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 152
    #[test]
    fn test_99(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("or", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 153
    #[test]
    fn test_100(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4042326015 as u32),Immediate::Word(4294963440 as u32)];
        let result = runtime.call_exported_function("or", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 154
    #[test]
    fn test_101(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("or", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 156
    #[test]
    fn test_102(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("xor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 157
    #[test]
    fn test_103(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("xor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 158
    #[test]
    fn test_104(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("xor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 159
    #[test]
    fn test_105(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("xor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 160
    #[test]
    fn test_106(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("xor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 161
    #[test]
    fn test_107(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("xor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 162
    #[test]
    fn test_108(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("xor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483647 as u32)));
    }
    

    // Command line number: 163
    #[test]
    fn test_109(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("xor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 164
    #[test]
    fn test_110(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4042326015 as u32),Immediate::Word(4294963440 as u32)];
        let result = runtime.call_exported_function("xor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(252645135 as u32)));
    }
    

    // Command line number: 165
    #[test]
    fn test_111(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("xor", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 167
    #[test]
    fn test_112(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("shl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 168
    #[test]
    fn test_113(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("shl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 169
    #[test]
    fn test_114(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("shl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967294 as u32)));
    }
    

    // Command line number: 170
    #[test]
    fn test_115(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("shl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967294 as u32)));
    }
    

    // Command line number: 171
    #[test]
    fn test_116(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("shl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 172
    #[test]
    fn test_117(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1073741824 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("shl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 173
    #[test]
    fn test_118(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(31 as u32)];
        let result = runtime.call_exported_function("shl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 174
    #[test]
    fn test_119(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(32 as u32)];
        let result = runtime.call_exported_function("shl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 175
    #[test]
    fn test_120(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(33 as u32)];
        let result = runtime.call_exported_function("shl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 176
    #[test]
    fn test_121(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("shl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 177
    #[test]
    fn test_122(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("shl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 179
    #[test]
    fn test_123(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("shr_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 180
    #[test]
    fn test_124(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("shr_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 181
    #[test]
    fn test_125(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("shr_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 182
    #[test]
    fn test_126(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("shr_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1073741823 as u32)));
    }
    

    // Command line number: 183
    #[test]
    fn test_127(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("shr_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3221225472 as u32)));
    }
    

    // Command line number: 184
    #[test]
    fn test_128(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1073741824 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("shr_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(536870912 as u32)));
    }
    

    // Command line number: 185
    #[test]
    fn test_129(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(32 as u32)];
        let result = runtime.call_exported_function("shr_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 186
    #[test]
    fn test_130(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(33 as u32)];
        let result = runtime.call_exported_function("shr_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 187
    #[test]
    fn test_131(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("shr_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 188
    #[test]
    fn test_132(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("shr_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 189
    #[test]
    fn test_133(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("shr_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 190
    #[test]
    fn test_134(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(31 as u32)];
        let result = runtime.call_exported_function("shr_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 191
    #[test]
    fn test_135(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(32 as u32)];
        let result = runtime.call_exported_function("shr_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 192
    #[test]
    fn test_136(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(33 as u32)];
        let result = runtime.call_exported_function("shr_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 193
    #[test]
    fn test_137(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("shr_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 194
    #[test]
    fn test_138(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("shr_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 195
    #[test]
    fn test_139(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("shr_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 197
    #[test]
    fn test_140(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("shr_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 198
    #[test]
    fn test_141(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("shr_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 199
    #[test]
    fn test_142(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("shr_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483647 as u32)));
    }
    

    // Command line number: 200
    #[test]
    fn test_143(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("shr_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1073741823 as u32)));
    }
    

    // Command line number: 201
    #[test]
    fn test_144(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("shr_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1073741824 as u32)));
    }
    

    // Command line number: 202
    #[test]
    fn test_145(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1073741824 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("shr_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(536870912 as u32)));
    }
    

    // Command line number: 203
    #[test]
    fn test_146(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(32 as u32)];
        let result = runtime.call_exported_function("shr_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 204
    #[test]
    fn test_147(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(33 as u32)];
        let result = runtime.call_exported_function("shr_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 205
    #[test]
    fn test_148(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("shr_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 206
    #[test]
    fn test_149(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("shr_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 207
    #[test]
    fn test_150(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("shr_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 208
    #[test]
    fn test_151(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(31 as u32)];
        let result = runtime.call_exported_function("shr_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 209
    #[test]
    fn test_152(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(32 as u32)];
        let result = runtime.call_exported_function("shr_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 210
    #[test]
    fn test_153(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(33 as u32)];
        let result = runtime.call_exported_function("shr_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483647 as u32)));
    }
    

    // Command line number: 211
    #[test]
    fn test_154(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("shr_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 212
    #[test]
    fn test_155(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("shr_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 213
    #[test]
    fn test_156(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("shr_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 215
    #[test]
    fn test_157(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("rotl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 216
    #[test]
    fn test_158(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("rotl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 217
    #[test]
    fn test_159(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("rotl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 218
    #[test]
    fn test_160(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(32 as u32)];
        let result = runtime.call_exported_function("rotl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 219
    #[test]
    fn test_161(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2882377846 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("rotl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1469788397 as u32)));
    }
    

    // Command line number: 220
    #[test]
    fn test_162(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4261469184 as u32),Immediate::Word(4 as u32)];
        let result = runtime.call_exported_function("rotl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3758997519 as u32)));
    }
    

    // Command line number: 221
    #[test]
    fn test_163(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2965492451 as u32),Immediate::Word(5 as u32)];
        let result = runtime.call_exported_function("rotl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(406477942 as u32)));
    }
    

    // Command line number: 222
    #[test]
    fn test_164(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(32768 as u32),Immediate::Word(37 as u32)];
        let result = runtime.call_exported_function("rotl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1048576 as u32)));
    }
    

    // Command line number: 223
    #[test]
    fn test_165(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2965492451 as u32),Immediate::Word(65285 as u32)];
        let result = runtime.call_exported_function("rotl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(406477942 as u32)));
    }
    

    // Command line number: 224
    #[test]
    fn test_166(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1989852383 as u32),Immediate::Word(4294967277 as u32)];
        let result = runtime.call_exported_function("rotl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1469837011 as u32)));
    }
    

    // Command line number: 225
    #[test]
    fn test_167(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1989852383 as u32),Immediate::Word(2147483661 as u32)];
        let result = runtime.call_exported_function("rotl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1469837011 as u32)));
    }
    

    // Command line number: 226
    #[test]
    fn test_168(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(31 as u32)];
        let result = runtime.call_exported_function("rotl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 227
    #[test]
    fn test_169(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("rotl", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 229
    #[test]
    fn test_170(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("rotr", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 230
    #[test]
    fn test_171(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("rotr", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 231
    #[test]
    fn test_172(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("rotr", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 232
    #[test]
    fn test_173(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(32 as u32)];
        let result = runtime.call_exported_function("rotr", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 233
    #[test]
    fn test_174(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4278242304 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("rotr", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139121152 as u32)));
    }
    

    // Command line number: 234
    #[test]
    fn test_175(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(524288 as u32),Immediate::Word(4 as u32)];
        let result = runtime.call_exported_function("rotr", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(32768 as u32)));
    }
    

    // Command line number: 235
    #[test]
    fn test_176(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2965492451 as u32),Immediate::Word(5 as u32)];
        let result = runtime.call_exported_function("rotr", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(495324823 as u32)));
    }
    

    // Command line number: 236
    #[test]
    fn test_177(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(32768 as u32),Immediate::Word(37 as u32)];
        let result = runtime.call_exported_function("rotr", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1024 as u32)));
    }
    

    // Command line number: 237
    #[test]
    fn test_178(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2965492451 as u32),Immediate::Word(65285 as u32)];
        let result = runtime.call_exported_function("rotr", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(495324823 as u32)));
    }
    

    // Command line number: 238
    #[test]
    fn test_179(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1989852383 as u32),Immediate::Word(4294967277 as u32)];
        let result = runtime.call_exported_function("rotr", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3875255509 as u32)));
    }
    

    // Command line number: 239
    #[test]
    fn test_180(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1989852383 as u32),Immediate::Word(2147483661 as u32)];
        let result = runtime.call_exported_function("rotr", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3875255509 as u32)));
    }
    

    // Command line number: 240
    #[test]
    fn test_181(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(31 as u32)];
        let result = runtime.call_exported_function("rotr", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 241
    #[test]
    fn test_182(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(31 as u32)];
        let result = runtime.call_exported_function("rotr", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 243
    #[test]
    fn test_183(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("clz", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 244
    #[test]
    fn test_184(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("clz", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(32 as u32)));
    }
    

    // Command line number: 245
    #[test]
    fn test_185(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(32768 as u32)];
        let result = runtime.call_exported_function("clz", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(16 as u32)));
    }
    

    // Command line number: 246
    #[test]
    fn test_186(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(255 as u32)];
        let result = runtime.call_exported_function("clz", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(24 as u32)));
    }
    

    // Command line number: 247
    #[test]
    fn test_187(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("clz", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 248
    #[test]
    fn test_188(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("clz", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(31 as u32)));
    }
    

    // Command line number: 249
    #[test]
    fn test_189(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2 as u32)];
        let result = runtime.call_exported_function("clz", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(30 as u32)));
    }
    

    // Command line number: 250
    #[test]
    fn test_190(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("clz", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 252
    #[test]
    fn test_191(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("ctz", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 253
    #[test]
    fn test_192(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("ctz", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(32 as u32)));
    }
    

    // Command line number: 254
    #[test]
    fn test_193(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(32768 as u32)];
        let result = runtime.call_exported_function("ctz", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(15 as u32)));
    }
    

    // Command line number: 255
    #[test]
    fn test_194(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65536 as u32)];
        let result = runtime.call_exported_function("ctz", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(16 as u32)));
    }
    

    // Command line number: 256
    #[test]
    fn test_195(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("ctz", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(31 as u32)));
    }
    

    // Command line number: 257
    #[test]
    fn test_196(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("ctz", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 259
    #[test]
    fn test_197(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("popcnt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(32 as u32)));
    }
    

    // Command line number: 260
    #[test]
    fn test_198(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("popcnt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 261
    #[test]
    fn test_199(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(32768 as u32)];
        let result = runtime.call_exported_function("popcnt", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    
}


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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "conversions.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 29
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("i64.extend_i32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 30
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(10000 as u32)];
        let result = runtime.call_exported_function("i64.extend_i32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10000 as u64)));
    }
    

    // Command line number: 31
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294957296 as u32)];
        let result = runtime.call_exported_function("i64.extend_i32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709541616 as u64)));
    }
    

    // Command line number: 32
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("i64.extend_i32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551615 as u64)));
    }
    

    // Command line number: 33
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("i64.extend_i32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2147483647 as u64)));
    }
    

    // Command line number: 34
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("i64.extend_i32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744071562067968 as u64)));
    }
    

    // Command line number: 36
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("i64.extend_i32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 37
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(10000 as u32)];
        let result = runtime.call_exported_function("i64.extend_i32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10000 as u64)));
    }
    

    // Command line number: 38
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294957296 as u32)];
        let result = runtime.call_exported_function("i64.extend_i32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4294957296 as u64)));
    }
    

    // Command line number: 39
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("i64.extend_i32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4294967295 as u64)));
    }
    

    // Command line number: 40
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("i64.extend_i32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2147483647 as u64)));
    }
    

    // Command line number: 41
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("i64.extend_i32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2147483648 as u64)));
    }
    

    // Command line number: 43
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709551615 as u64)];
        let result = runtime.call_exported_function("i32.wrap_i64", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 44
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709451616 as u64)];
        let result = runtime.call_exported_function("i32.wrap_i64", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294867296 as u32)));
    }
    

    // Command line number: 45
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(2147483648 as u64)];
        let result = runtime.call_exported_function("i32.wrap_i64", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 46
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744071562067967 as u64)];
        let result = runtime.call_exported_function("i32.wrap_i64", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483647 as u32)));
    }
    

    // Command line number: 47
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744069414584320 as u64)];
        let result = runtime.call_exported_function("i32.wrap_i64", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 48
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744069414584319 as u64)];
        let result = runtime.call_exported_function("i32.wrap_i64", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 49
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744069414584321 as u64)];
        let result = runtime.call_exported_function("i32.wrap_i64", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 50
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("i32.wrap_i64", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 51
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1311768467463790320 as u64)];
        let result = runtime.call_exported_function("i32.wrap_i64", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2596069104 as u32)));
    }
    

    // Command line number: 52
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4294967295 as u64)];
        let result = runtime.call_exported_function("i32.wrap_i64", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 53
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4294967296 as u64)];
        let result = runtime.call_exported_function("i32.wrap_i64", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 54
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4294967297 as u64)];
        let result = runtime.call_exported_function("i32.wrap_i64", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 56
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 57
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 58
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 59
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 60
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 61
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1066192077 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 62
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1069547520 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 63
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 64
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3213675725 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 65
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3217031168 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 66
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3220386611 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 67
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3221225472 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967294 as u32)));
    }
    

    // Command line number: 68
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1325400063 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483520 as u32)));
    }
    

    // Command line number: 69
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3472883712 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 79
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 80
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 81
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 82
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 83
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 84
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1066192077 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 85
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1069547520 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 86
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1072902963 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 87
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1073741824 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 88
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1325400064 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 89
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1333788671 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967040 as u32)));
    }
    

    // Command line number: 90
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3211159142 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 91
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836863 as u32)];
        let result = runtime.call_exported_function("i32.trunc_f32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 101
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 102
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 103
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 104
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 105
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 106
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607632778762754458 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 107
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4609434218613702656 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 108
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 109
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13831004815617530266 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 110
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13832806255468478464 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 111
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13834607695319426662 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 112
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13835058055282163712 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967294 as u32)));
    }
    

    // Command line number: 113
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4746794007244308480 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483647 as u32)));
    }
    

    // Command line number: 114
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13970166044103278592 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 124
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 125
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 126
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 127
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 128
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 129
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607632778762754458 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 130
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4609434218613702656 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 131
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4611235658464650854 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 132
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4611686018427387904 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2 as u32)));
    }
    

    // Command line number: 133
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4746794007248502784 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 134
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4751297606873776128 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 135
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13829653735729319117 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 136
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793215 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 137
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4726483295884279808 as u64)];
        let result = runtime.call_exported_function("i32.trunc_f64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(100000000 as u32)));
    }
    

    // Command line number: 150
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 151
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 152
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 153
    #[test]
    fn test_82(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 154
    #[test]
    fn test_83(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 155
    #[test]
    fn test_84(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1066192077 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 156
    #[test]
    fn test_85(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1069547520 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 157
    #[test]
    fn test_86(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551615 as u64)));
    }
    

    // Command line number: 158
    #[test]
    fn test_87(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3213675725 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551615 as u64)));
    }
    

    // Command line number: 159
    #[test]
    fn test_88(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3217031168 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551615 as u64)));
    }
    

    // Command line number: 160
    #[test]
    fn test_89(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3220386611 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551615 as u64)));
    }
    

    // Command line number: 161
    #[test]
    fn test_90(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3221225472 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551614 as u64)));
    }
    

    // Command line number: 162
    #[test]
    fn test_91(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1333788672 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4294967296 as u64)));
    }
    

    // Command line number: 163
    #[test]
    fn test_92(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3481272320 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744069414584320 as u64)));
    }
    

    // Command line number: 164
    #[test]
    fn test_93(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1593835519 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223371487098961920 as u64)));
    }
    

    // Command line number: 165
    #[test]
    fn test_94(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3741319168 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 175
    #[test]
    fn test_95(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 176
    #[test]
    fn test_96(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 177
    #[test]
    fn test_97(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 178
    #[test]
    fn test_98(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 179
    #[test]
    fn test_99(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 180
    #[test]
    fn test_100(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1066192077 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 181
    #[test]
    fn test_101(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1069547520 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 182
    #[test]
    fn test_102(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1333788672 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4294967296 as u64)));
    }
    

    // Command line number: 183
    #[test]
    fn test_103(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1602224127 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446742974197923840 as u64)));
    }
    

    // Command line number: 184
    #[test]
    fn test_104(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3211159142 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 185
    #[test]
    fn test_105(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836863 as u32)];
        let result = runtime.call_exported_function("i64.trunc_f32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 195
    #[test]
    fn test_106(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 196
    #[test]
    fn test_107(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 197
    #[test]
    fn test_108(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 198
    #[test]
    fn test_109(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 199
    #[test]
    fn test_110(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 200
    #[test]
    fn test_111(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607632778762754458 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 201
    #[test]
    fn test_112(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4609434218613702656 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 202
    #[test]
    fn test_113(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793216 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551615 as u64)));
    }
    

    // Command line number: 203
    #[test]
    fn test_114(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13831004815617530266 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551615 as u64)));
    }
    

    // Command line number: 204
    #[test]
    fn test_115(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13832806255468478464 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551615 as u64)));
    }
    

    // Command line number: 205
    #[test]
    fn test_116(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13834607695319426662 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551615 as u64)));
    }
    

    // Command line number: 206
    #[test]
    fn test_117(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13835058055282163712 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551614 as u64)));
    }
    

    // Command line number: 207
    #[test]
    fn test_118(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4751297606875873280 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4294967296 as u64)));
    }
    

    // Command line number: 208
    #[test]
    fn test_119(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13974669643730649088 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744069414584320 as u64)));
    }
    

    // Command line number: 209
    #[test]
    fn test_120(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4890909195324358655 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854774784 as u64)));
    }
    

    // Command line number: 210
    #[test]
    fn test_121(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(14114281232179134464 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 220
    #[test]
    fn test_122(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 221
    #[test]
    fn test_123(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 222
    #[test]
    fn test_124(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 223
    #[test]
    fn test_125(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775809 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 224
    #[test]
    fn test_126(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 225
    #[test]
    fn test_127(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607632778762754458 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 226
    #[test]
    fn test_128(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4609434218613702656 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1 as u64)));
    }
    

    // Command line number: 227
    #[test]
    fn test_129(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4751297606873776128 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4294967295 as u64)));
    }
    

    // Command line number: 228
    #[test]
    fn test_130(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4751297606875873280 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4294967296 as u64)));
    }
    

    // Command line number: 229
    #[test]
    fn test_131(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4895412794951729151 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709549568 as u64)));
    }
    

    // Command line number: 230
    #[test]
    fn test_132(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13829653735729319117 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 231
    #[test]
    fn test_133(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(13830554455654793215 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 232
    #[test]
    fn test_134(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4726483295884279808 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(100000000 as u64)));
    }
    

    // Command line number: 233
    #[test]
    fn test_135(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4846369599423283200 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(10000000000000000 as u64)));
    }
    

    // Command line number: 234
    #[test]
    fn test_136(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4890909195324358656 as u64)];
        let result = runtime.call_exported_function("i64.trunc_f64_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(9223372036854775808 as u64)));
    }
    

    // Command line number: 244
    #[test]
    fn test_137(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 245
    #[test]
    fn test_138(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 246
    #[test]
    fn test_139(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 247
    #[test]
    fn test_140(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1325400064 as u32)));
    }
    

    // Command line number: 248
    #[test]
    fn test_141(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3472883712 as u32)));
    }
    

    // Command line number: 249
    #[test]
    fn test_142(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1234567890 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1318267910 as u32)));
    }
    

    // Command line number: 251
    #[test]
    fn test_143(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(16777217 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1266679808 as u32)));
    }
    

    // Command line number: 252
    #[test]
    fn test_144(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4278190079 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3414163456 as u32)));
    }
    

    // Command line number: 253
    #[test]
    fn test_145(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(16777219 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1266679810 as u32)));
    }
    

    // Command line number: 254
    #[test]
    fn test_146(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4278190077 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3414163458 as u32)));
    }
    

    // Command line number: 256
    #[test]
    fn test_147(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 257
    #[test]
    fn test_148(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709551615 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 258
    #[test]
    fn test_149(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 259
    #[test]
    fn test_150(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775807 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1593835520 as u32)));
    }
    

    // Command line number: 260
    #[test]
    fn test_151(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3741319168 as u32)));
    }
    

    // Command line number: 261
    #[test]
    fn test_152(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(314159265358979 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1468980468 as u32)));
    }
    

    // Command line number: 263
    #[test]
    fn test_153(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(16777217 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1266679808 as u32)));
    }
    

    // Command line number: 264
    #[test]
    fn test_154(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073692774399 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3414163456 as u32)));
    }
    

    // Command line number: 265
    #[test]
    fn test_155(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(16777219 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1266679810 as u32)));
    }
    

    // Command line number: 266
    #[test]
    fn test_156(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073692774397 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3414163458 as u32)));
    }
    

    // Command line number: 268
    #[test]
    fn test_157(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223371212221054977 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1593835519 as u32)));
    }
    

    // Command line number: 269
    #[test]
    fn test_158(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372311732682753 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3741319167 as u32)));
    }
    

    // Command line number: 270
    #[test]
    fn test_159(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9007199791611905 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1509949441 as u32)));
    }
    

    // Command line number: 271
    #[test]
    fn test_160(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18437736873917939711 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3657433089 as u32)));
    }
    

    // Command line number: 273
    #[test]
    fn test_161(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("f64.convert_i32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 274
    #[test]
    fn test_162(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("f64.convert_i32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13830554455654793216 as u64)));
    }
    

    // Command line number: 275
    #[test]
    fn test_163(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("f64.convert_i32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 276
    #[test]
    fn test_164(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("f64.convert_i32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4746794007244308480 as u64)));
    }
    

    // Command line number: 277
    #[test]
    fn test_165(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("f64.convert_i32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13970166044103278592 as u64)));
    }
    

    // Command line number: 278
    #[test]
    fn test_166(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(987654321 as u32)];
        let result = runtime.call_exported_function("f64.convert_i32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4741568253304766464 as u64)));
    }
    

    // Command line number: 280
    #[test]
    fn test_167(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("f64.convert_i64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4607182418800017408 as u64)));
    }
    

    // Command line number: 281
    #[test]
    fn test_168(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709551615 as u64)];
        let result = runtime.call_exported_function("f64.convert_i64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(13830554455654793216 as u64)));
    }
    

    // Command line number: 282
    #[test]
    fn test_169(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("f64.convert_i64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(0 as u64)));
    }
    

    // Command line number: 283
    #[test]
    fn test_170(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775807 as u64)];
        let result = runtime.call_exported_function("f64.convert_i64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4890909195324358656 as u64)));
    }
    

    // Command line number: 284
    #[test]
    fn test_171(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("f64.convert_i64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14114281232179134464 as u64)));
    }
    

    // Command line number: 285
    #[test]
    fn test_172(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4669201609102990 as u64)];
        let result = runtime.call_exported_function("f64.convert_i64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4841535201405015694 as u64)));
    }
    

    // Command line number: 287
    #[test]
    fn test_173(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9007199254740993 as u64)];
        let result = runtime.call_exported_function("f64.convert_i64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4845873199050653696 as u64)));
    }
    

    // Command line number: 288
    #[test]
    fn test_174(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18437736874454810623 as u64)];
        let result = runtime.call_exported_function("f64.convert_i64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14069245235905429504 as u64)));
    }
    

    // Command line number: 289
    #[test]
    fn test_175(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9007199254740995 as u64)];
        let result = runtime.call_exported_function("f64.convert_i64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4845873199050653698 as u64)));
    }
    

    // Command line number: 290
    #[test]
    fn test_176(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18437736874454810621 as u64)];
        let result = runtime.call_exported_function("f64.convert_i64_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(14069245235905429506 as u64)));
    }
    

    // Command line number: 292
    #[test]
    fn test_177(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 293
    #[test]
    fn test_178(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 294
    #[test]
    fn test_179(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483647 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1325400064 as u32)));
    }
    

    // Command line number: 295
    #[test]
    fn test_180(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1325400064 as u32)));
    }
    

    // Command line number: 296
    #[test]
    fn test_181(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(305419896 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1301390004 as u32)));
    }
    

    // Command line number: 297
    #[test]
    fn test_182(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1333788672 as u32)));
    }
    

    // Command line number: 298
    #[test]
    fn test_183(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483776 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1325400064 as u32)));
    }
    

    // Command line number: 299
    #[test]
    fn test_184(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483777 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1325400065 as u32)));
    }
    

    // Command line number: 300
    #[test]
    fn test_185(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483778 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1325400065 as u32)));
    }
    

    // Command line number: 301
    #[test]
    fn test_186(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294966912 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1333788670 as u32)));
    }
    

    // Command line number: 302
    #[test]
    fn test_187(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294966913 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1333788671 as u32)));
    }
    

    // Command line number: 303
    #[test]
    fn test_188(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294966914 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1333788671 as u32)));
    }
    

    // Command line number: 305
    #[test]
    fn test_189(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(16777217 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1266679808 as u32)));
    }
    

    // Command line number: 306
    #[test]
    fn test_190(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(16777219 as u32)];
        let result = runtime.call_exported_function("f32.convert_i32_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1266679810 as u32)));
    }
    

    // Command line number: 308
    #[test]
    fn test_191(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 309
    #[test]
    fn test_192(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(0 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 310
    #[test]
    fn test_193(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775807 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1593835520 as u32)));
    }
    

    // Command line number: 311
    #[test]
    fn test_194(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223372036854775808 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1593835520 as u32)));
    }
    

    // Command line number: 312
    #[test]
    fn test_195(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709551615 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1602224128 as u32)));
    }
    

    // Command line number: 314
    #[test]
    fn test_196(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(16777217 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1266679808 as u32)));
    }
    

    // Command line number: 315
    #[test]
    fn test_197(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(16777219 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1266679810 as u32)));
    }
    

    // Command line number: 317
    #[test]
    fn test_198(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9007199791611905 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1509949441 as u32)));
    }
    

    // Command line number: 318
    #[test]
    fn test_199(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(9223371761976868863 as u64)];
        let result = runtime.call_exported_function("f32.convert_i64_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1593835519 as u32)));
    }
    
}

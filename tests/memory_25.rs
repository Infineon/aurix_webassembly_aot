
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "memory.25.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 165
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("data", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 166
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![];
        let result = runtime.call_exported_function("cast", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4631107791820423168 as u64)));
    }
    

    // Command line number: 168
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("i32_load8_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 169
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("i32_load8_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(255 as u32)));
    }
    

    // Command line number: 170
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("i32_load16_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967295 as u32)));
    }
    

    // Command line number: 171
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4294967295 as u32)];
        let result = runtime.call_exported_function("i32_load16_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(65535 as u32)));
    }
    

    // Command line number: 173
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(100 as u32)];
        let result = runtime.call_exported_function("i32_load8_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(100 as u32)));
    }
    

    // Command line number: 174
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(200 as u32)];
        let result = runtime.call_exported_function("i32_load8_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(200 as u32)));
    }
    

    // Command line number: 175
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(20000 as u32)];
        let result = runtime.call_exported_function("i32_load16_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(20000 as u32)));
    }
    

    // Command line number: 176
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(40000 as u32)];
        let result = runtime.call_exported_function("i32_load16_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(40000 as u32)));
    }
    

    // Command line number: 178
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4275856707 as u32)];
        let result = runtime.call_exported_function("i32_load8_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(67 as u32)));
    }
    

    // Command line number: 179
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(878104047 as u32)];
        let result = runtime.call_exported_function("i32_load8_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294967279 as u32)));
    }
    

    // Command line number: 180
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4275856707 as u32)];
        let result = runtime.call_exported_function("i32_load8_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(67 as u32)));
    }
    

    // Command line number: 181
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(878104047 as u32)];
        let result = runtime.call_exported_function("i32_load8_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(239 as u32)));
    }
    

    // Command line number: 182
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4275856707 as u32)];
        let result = runtime.call_exported_function("i32_load16_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(25923 as u32)));
    }
    

    // Command line number: 183
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(878104047 as u32)];
        let result = runtime.call_exported_function("i32_load16_s", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4294954479 as u32)));
    }
    

    // Command line number: 184
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4275856707 as u32)];
        let result = runtime.call_exported_function("i32_load16_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(25923 as u32)));
    }
    

    // Command line number: 185
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(878104047 as u32)];
        let result = runtime.call_exported_function("i32_load16_u", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(52719 as u32)));
    }
    

    // Command line number: 187
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709551615 as u64)];
        let result = runtime.call_exported_function("i64_load8_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551615 as u64)));
    }
    

    // Command line number: 188
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709551615 as u64)];
        let result = runtime.call_exported_function("i64_load8_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(255 as u64)));
    }
    

    // Command line number: 189
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709551615 as u64)];
        let result = runtime.call_exported_function("i64_load16_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551615 as u64)));
    }
    

    // Command line number: 190
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709551615 as u64)];
        let result = runtime.call_exported_function("i64_load16_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(65535 as u64)));
    }
    

    // Command line number: 191
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709551615 as u64)];
        let result = runtime.call_exported_function("i64_load32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551615 as u64)));
    }
    

    // Command line number: 192
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18446744073709551615 as u64)];
        let result = runtime.call_exported_function("i64_load32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4294967295 as u64)));
    }
    

    // Command line number: 194
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(100 as u64)];
        let result = runtime.call_exported_function("i64_load8_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(100 as u64)));
    }
    

    // Command line number: 195
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(200 as u64)];
        let result = runtime.call_exported_function("i64_load8_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(200 as u64)));
    }
    

    // Command line number: 196
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(20000 as u64)];
        let result = runtime.call_exported_function("i64_load16_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(20000 as u64)));
    }
    

    // Command line number: 197
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(40000 as u64)];
        let result = runtime.call_exported_function("i64_load16_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(40000 as u64)));
    }
    

    // Command line number: 198
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(20000 as u64)];
        let result = runtime.call_exported_function("i64_load32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(20000 as u64)));
    }
    

    // Command line number: 199
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(40000 as u64)];
        let result = runtime.call_exported_function("i64_load32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(40000 as u64)));
    }
    

    // Command line number: 201
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18364758543954109763 as u64)];
        let result = runtime.call_exported_function("i64_load8_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(67 as u64)));
    }
    

    // Command line number: 202
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(3771275841602506223 as u64)];
        let result = runtime.call_exported_function("i64_load8_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709551599 as u64)));
    }
    

    // Command line number: 203
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18364758543954109763 as u64)];
        let result = runtime.call_exported_function("i64_load8_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(67 as u64)));
    }
    

    // Command line number: 204
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(3771275841602506223 as u64)];
        let result = runtime.call_exported_function("i64_load8_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(239 as u64)));
    }
    

    // Command line number: 205
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18364758543954109763 as u64)];
        let result = runtime.call_exported_function("i64_load16_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(25923 as u64)));
    }
    

    // Command line number: 206
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(3771275841602506223 as u64)];
        let result = runtime.call_exported_function("i64_load16_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744073709538799 as u64)));
    }
    

    // Command line number: 207
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18364758543954109763 as u64)];
        let result = runtime.call_exported_function("i64_load16_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(25923 as u64)));
    }
    

    // Command line number: 208
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(3771275841602506223 as u64)];
        let result = runtime.call_exported_function("i64_load16_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(52719 as u64)));
    }
    

    // Command line number: 209
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18364758543954109763 as u64)];
        let result = runtime.call_exported_function("i64_load32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1446274371 as u64)));
    }
    

    // Command line number: 210
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(3771275841602506223 as u64)];
        let result = runtime.call_exported_function("i64_load32_s", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(18446744071976963567 as u64)));
    }
    

    // Command line number: 211
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(18364758543954109763 as u64)];
        let result = runtime.call_exported_function("i64_load32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(1446274371 as u64)));
    }
    

    // Command line number: 212
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(3771275841602506223 as u64)];
        let result = runtime.call_exported_function("i64_load32_u", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(2562379247 as u64)));
    }
    
}

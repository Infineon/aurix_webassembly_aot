
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "address.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 104
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("8u_good1", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(97 as u32)));
    }
    

    // Command line number: 105
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("8u_good2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(97 as u32)));
    }
    

    // Command line number: 106
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("8u_good3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(98 as u32)));
    }
    

    // Command line number: 107
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("8u_good4", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(99 as u32)));
    }
    

    // Command line number: 108
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("8u_good5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(122 as u32)));
    }
    

    // Command line number: 110
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("8s_good1", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(97 as u32)));
    }
    

    // Command line number: 111
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("8s_good2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(97 as u32)));
    }
    

    // Command line number: 112
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("8s_good3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(98 as u32)));
    }
    

    // Command line number: 113
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("8s_good4", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(99 as u32)));
    }
    

    // Command line number: 114
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("8s_good5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(122 as u32)));
    }
    

    // Command line number: 116
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("16u_good1", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(25185 as u32)));
    }
    

    // Command line number: 117
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("16u_good2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(25185 as u32)));
    }
    

    // Command line number: 118
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("16u_good3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(25442 as u32)));
    }
    

    // Command line number: 119
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("16u_good4", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(25699 as u32)));
    }
    

    // Command line number: 120
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("16u_good5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(122 as u32)));
    }
    

    // Command line number: 122
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("16s_good1", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(25185 as u32)));
    }
    

    // Command line number: 123
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("16s_good2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(25185 as u32)));
    }
    

    // Command line number: 124
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("16s_good3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(25442 as u32)));
    }
    

    // Command line number: 125
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("16s_good4", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(25699 as u32)));
    }
    

    // Command line number: 126
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("16s_good5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(122 as u32)));
    }
    

    // Command line number: 128
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("32_good1", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1684234849 as u32)));
    }
    

    // Command line number: 129
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("32_good2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1684234849 as u32)));
    }
    

    // Command line number: 130
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("32_good3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1701077858 as u32)));
    }
    

    // Command line number: 131
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("32_good4", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1717920867 as u32)));
    }
    

    // Command line number: 132
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("32_good5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(122 as u32)));
    }
    

    // Command line number: 134
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("8u_good1", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 135
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("8u_good2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 136
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("8u_good3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 137
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("8u_good4", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 138
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("8u_good5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 140
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("8s_good1", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 141
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("8s_good2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 142
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("8s_good3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 143
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("8s_good4", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 144
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("8s_good5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 146
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("16u_good1", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 147
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("16u_good2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 148
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("16u_good3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 149
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("16u_good4", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 150
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("16u_good5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 152
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("16s_good1", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 153
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("16s_good2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 154
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("16s_good3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 155
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("16s_good4", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 156
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("16s_good5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 158
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("32_good1", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 159
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("32_good2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 160
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("32_good3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 161
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("32_good4", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 162
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65507 as u32)];
        let result = runtime.call_exported_function("32_good5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 164
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("8u_good1", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 165
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("8u_good2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 166
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("8u_good3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 167
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("8u_good4", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 168
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("8u_good5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 170
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("8s_good1", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 171
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("8s_good2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 172
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("8s_good3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 173
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("8s_good4", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 174
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("8s_good5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 176
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("16u_good1", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 177
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("16u_good2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 178
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("16u_good3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 179
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("16u_good4", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 180
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("16u_good5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 182
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("16s_good1", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 183
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("16s_good2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 184
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("16s_good3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 185
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("16s_good4", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 186
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("16s_good5", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 188
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("32_good1", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 189
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("32_good2", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 190
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("32_good3", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 191
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(65508 as u32)];
        let result = runtime.call_exported_function("32_good4", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    
}

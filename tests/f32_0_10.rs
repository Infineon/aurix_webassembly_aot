
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "f32.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 2164
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2165
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2166
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2167
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 2168
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2169
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2170
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2171
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2172
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2173
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2174
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2175
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2176
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2177
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2178
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2179
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2180
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2181
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2182
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2183
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 2184
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 2185
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2186
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2187
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 2188
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2189
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2190
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2191
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 2192
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2193
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2194
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2195
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 2196
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2197
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2198
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2199
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 2200
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2201
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2202
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2203
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 2204
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2205
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2206
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2207
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 2208
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2209
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2210
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2211
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2212
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2213
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2214
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3212836864 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2215
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2216
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2217
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2218
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2219
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2220
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2221
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2222
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2223
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 2224
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 2225
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2226
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2227
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 2228
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2229
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2230
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2231
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 2232
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2233
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2234
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2235
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 2236
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2237
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2238
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2239
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3234402267 as u32)));
    }
    

    // Command line number: 2240
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2241
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2242
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2243
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3234402267 as u32)));
    }
    

    // Command line number: 2244
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2245
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2246
    #[test]
    fn test_82(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2247
    #[test]
    fn test_83(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3234402267 as u32)));
    }
    

    // Command line number: 2248
    #[test]
    fn test_84(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2249
    #[test]
    fn test_85(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2250
    #[test]
    fn test_86(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2251
    #[test]
    fn test_87(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2252
    #[test]
    fn test_88(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2253
    #[test]
    fn test_89(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2254
    #[test]
    fn test_90(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3234402267 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2255
    #[test]
    fn test_91(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2256
    #[test]
    fn test_92(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2257
    #[test]
    fn test_93(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2258
    #[test]
    fn test_94(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1086918619 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2259
    #[test]
    fn test_95(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2260
    #[test]
    fn test_96(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2261
    #[test]
    fn test_97(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2262
    #[test]
    fn test_98(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2263
    #[test]
    fn test_99(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 2264
    #[test]
    fn test_100(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 2265
    #[test]
    fn test_101(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2266
    #[test]
    fn test_102(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2267
    #[test]
    fn test_103(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 2268
    #[test]
    fn test_104(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2269
    #[test]
    fn test_105(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2270
    #[test]
    fn test_106(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2271
    #[test]
    fn test_107(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 2272
    #[test]
    fn test_108(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2273
    #[test]
    fn test_109(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2274
    #[test]
    fn test_110(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2275
    #[test]
    fn test_111(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 2276
    #[test]
    fn test_112(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2277
    #[test]
    fn test_113(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2278
    #[test]
    fn test_114(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2279
    #[test]
    fn test_115(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3234402267 as u32)));
    }
    

    // Command line number: 2280
    #[test]
    fn test_116(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2281
    #[test]
    fn test_117(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2282
    #[test]
    fn test_118(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2283
    #[test]
    fn test_119(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    

    // Command line number: 2284
    #[test]
    fn test_120(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2285
    #[test]
    fn test_121(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2286
    #[test]
    fn test_122(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2287
    #[test]
    fn test_123(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    

    // Command line number: 2288
    #[test]
    fn test_124(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2289
    #[test]
    fn test_125(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2290
    #[test]
    fn test_126(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2291
    #[test]
    fn test_127(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2292
    #[test]
    fn test_128(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2293
    #[test]
    fn test_129(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2294
    #[test]
    fn test_130(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578687 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2295
    #[test]
    fn test_131(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2296
    #[test]
    fn test_132(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2297
    #[test]
    fn test_133(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2298
    #[test]
    fn test_134(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095039 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2299
    #[test]
    fn test_135(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2300
    #[test]
    fn test_136(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2301
    #[test]
    fn test_137(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2302
    #[test]
    fn test_138(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2303
    #[test]
    fn test_139(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 2304
    #[test]
    fn test_140(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 2305
    #[test]
    fn test_141(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2306
    #[test]
    fn test_142(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2307
    #[test]
    fn test_143(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 2308
    #[test]
    fn test_144(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2309
    #[test]
    fn test_145(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2310
    #[test]
    fn test_146(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2311
    #[test]
    fn test_147(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 2312
    #[test]
    fn test_148(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2313
    #[test]
    fn test_149(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2314
    #[test]
    fn test_150(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2315
    #[test]
    fn test_151(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 2316
    #[test]
    fn test_152(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2317
    #[test]
    fn test_153(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2318
    #[test]
    fn test_154(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2319
    #[test]
    fn test_155(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3234402267 as u32)));
    }
    

    // Command line number: 2320
    #[test]
    fn test_156(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2321
    #[test]
    fn test_157(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2322
    #[test]
    fn test_158(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2323
    #[test]
    fn test_159(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    

    // Command line number: 2324
    #[test]
    fn test_160(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2325
    #[test]
    fn test_161(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2326
    #[test]
    fn test_162(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2327
    #[test]
    fn test_163(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 2328
    #[test]
    fn test_164(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2329
    #[test]
    fn test_165(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2330
    #[test]
    fn test_166(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2331
    #[test]
    fn test_167(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2332
    #[test]
    fn test_168(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2333
    #[test]
    fn test_169(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2334
    #[test]
    fn test_170(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4286578688 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2335
    #[test]
    fn test_171(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2336
    #[test]
    fn test_172(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2337
    #[test]
    fn test_173(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2338
    #[test]
    fn test_174(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2139095040 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2339
    #[test]
    fn test_175(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2340
    #[test]
    fn test_176(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2341
    #[test]
    fn test_177(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2342
    #[test]
    fn test_178(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2343
    #[test]
    fn test_179(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2344
    #[test]
    fn test_180(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2345
    #[test]
    fn test_181(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2346
    #[test]
    fn test_182(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2347
    #[test]
    fn test_183(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2348
    #[test]
    fn test_184(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2349
    #[test]
    fn test_185(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2350
    #[test]
    fn test_186(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2351
    #[test]
    fn test_187(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2352
    #[test]
    fn test_188(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2353
    #[test]
    fn test_189(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2354
    #[test]
    fn test_190(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2355
    #[test]
    fn test_191(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2356
    #[test]
    fn test_192(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2357
    #[test]
    fn test_193(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2358
    #[test]
    fn test_194(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2359
    #[test]
    fn test_195(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2360
    #[test]
    fn test_196(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2361
    #[test]
    fn test_197(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2362
    #[test]
    fn test_198(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2363
    #[test]
    fn test_199(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    
}

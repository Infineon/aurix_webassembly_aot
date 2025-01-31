
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

    
    // Command line number: 1964
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1965
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1966
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1967
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1968
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1969
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1970
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1971
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1972
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1973
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1974
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1975
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1976
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1977
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1978
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1979
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1980
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1981
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1982
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1983
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1984
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1985
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1986
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1987
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1988
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1989
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1990
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1991
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1992
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1993
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1994
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1995
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1996
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1997
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1998
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1999
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2000
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2001
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2002
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2003
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2004
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2005
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2006
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2007
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2008
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2009
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2010
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2011
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2012
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2013
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2014
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2015
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2016
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2017
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2018
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2019
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2020
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2021
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2022
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2023
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2024
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 2025
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2026
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 2027
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2028
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2029
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2030
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2031
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2032
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2033
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2034
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2035
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2036
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2037
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2038
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2039
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2040
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2041
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2042
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2043
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2044
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2045
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2046
    #[test]
    fn test_82(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2047
    #[test]
    fn test_83(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2048
    #[test]
    fn test_84(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2049
    #[test]
    fn test_85(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2050
    #[test]
    fn test_86(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2051
    #[test]
    fn test_87(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2052
    #[test]
    fn test_88(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2053
    #[test]
    fn test_89(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2054
    #[test]
    fn test_90(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2055
    #[test]
    fn test_91(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2056
    #[test]
    fn test_92(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2057
    #[test]
    fn test_93(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2058
    #[test]
    fn test_94(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2059
    #[test]
    fn test_95(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2060
    #[test]
    fn test_96(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2061
    #[test]
    fn test_97(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 2062
    #[test]
    fn test_98(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 2063
    #[test]
    fn test_99(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 2064
    #[test]
    fn test_100(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 2065
    #[test]
    fn test_101(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 2066
    #[test]
    fn test_102(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 2067
    #[test]
    fn test_103(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 2068
    #[test]
    fn test_104(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2069
    #[test]
    fn test_105(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 2070
    #[test]
    fn test_106(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2071
    #[test]
    fn test_107(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 2072
    #[test]
    fn test_108(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2073
    #[test]
    fn test_109(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 2074
    #[test]
    fn test_110(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2075
    #[test]
    fn test_111(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 2076
    #[test]
    fn test_112(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2077
    #[test]
    fn test_113(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 2078
    #[test]
    fn test_114(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2079
    #[test]
    fn test_115(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 2080
    #[test]
    fn test_116(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2081
    #[test]
    fn test_117(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 2082
    #[test]
    fn test_118(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2083
    #[test]
    fn test_119(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 2084
    #[test]
    fn test_120(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2085
    #[test]
    fn test_121(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 2086
    #[test]
    fn test_122(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2087
    #[test]
    fn test_123(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 2088
    #[test]
    fn test_124(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2089
    #[test]
    fn test_125(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 2090
    #[test]
    fn test_126(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2091
    #[test]
    fn test_127(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2092
    #[test]
    fn test_128(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2093
    #[test]
    fn test_129(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2094
    #[test]
    fn test_130(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2095
    #[test]
    fn test_131(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2096
    #[test]
    fn test_132(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2097
    #[test]
    fn test_133(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2098
    #[test]
    fn test_134(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2099
    #[test]
    fn test_135(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2100
    #[test]
    fn test_136(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2101
    #[test]
    fn test_137(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2102
    #[test]
    fn test_138(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2103
    #[test]
    fn test_139(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 2104
    #[test]
    fn test_140(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 2105
    #[test]
    fn test_141(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2106
    #[test]
    fn test_142(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2107
    #[test]
    fn test_143(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 2108
    #[test]
    fn test_144(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2109
    #[test]
    fn test_145(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2110
    #[test]
    fn test_146(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2111
    #[test]
    fn test_147(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 2112
    #[test]
    fn test_148(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2113
    #[test]
    fn test_149(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2114
    #[test]
    fn test_150(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2115
    #[test]
    fn test_151(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 2116
    #[test]
    fn test_152(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2117
    #[test]
    fn test_153(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2118
    #[test]
    fn test_154(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2119
    #[test]
    fn test_155(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 2120
    #[test]
    fn test_156(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2121
    #[test]
    fn test_157(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2122
    #[test]
    fn test_158(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2123
    #[test]
    fn test_159(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 2124
    #[test]
    fn test_160(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2125
    #[test]
    fn test_161(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2126
    #[test]
    fn test_162(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095039 as u32)));
    }
    

    // Command line number: 2127
    #[test]
    fn test_163(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 2128
    #[test]
    fn test_164(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2129
    #[test]
    fn test_165(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2130
    #[test]
    fn test_166(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2139095040 as u32)));
    }
    

    // Command line number: 2131
    #[test]
    fn test_167(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2132
    #[test]
    fn test_168(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2133
    #[test]
    fn test_169(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2134
    #[test]
    fn test_170(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2135
    #[test]
    fn test_171(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2136
    #[test]
    fn test_172(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2137
    #[test]
    fn test_173(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 2138
    #[test]
    fn test_174(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 2139
    #[test]
    fn test_175(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 2140
    #[test]
    fn test_176(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 2141
    #[test]
    fn test_177(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2142
    #[test]
    fn test_178(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2143
    #[test]
    fn test_179(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 2144
    #[test]
    fn test_180(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 2145
    #[test]
    fn test_181(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2146
    #[test]
    fn test_182(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2147
    #[test]
    fn test_183(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 2148
    #[test]
    fn test_184(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 2149
    #[test]
    fn test_185(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2150
    #[test]
    fn test_186(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2151
    #[test]
    fn test_187(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 2152
    #[test]
    fn test_188(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2153
    #[test]
    fn test_189(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2154
    #[test]
    fn test_190(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2155
    #[test]
    fn test_191(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 2156
    #[test]
    fn test_192(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2157
    #[test]
    fn test_193(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2158
    #[test]
    fn test_194(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1065353216 as u32)));
    }
    

    // Command line number: 2159
    #[test]
    fn test_195(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 2160
    #[test]
    fn test_196(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2161
    #[test]
    fn test_197(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 2162
    #[test]
    fn test_198(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1086918619 as u32)));
    }
    

    // Command line number: 2163
    #[test]
    fn test_199(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("max", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    
}

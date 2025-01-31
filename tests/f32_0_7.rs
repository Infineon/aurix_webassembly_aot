
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

    
    // Command line number: 1564
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1565
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1566
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1567
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1568
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1569
    #[test]
    fn test_5(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1570
    #[test]
    fn test_6(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1571
    #[test]
    fn test_7(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1572
    #[test]
    fn test_8(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1573
    #[test]
    fn test_9(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1574
    #[test]
    fn test_10(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1575
    #[test]
    fn test_11(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1576
    #[test]
    fn test_12(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1577
    #[test]
    fn test_13(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1578
    #[test]
    fn test_14(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1579
    #[test]
    fn test_15(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1580
    #[test]
    fn test_16(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1581
    #[test]
    fn test_17(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1582
    #[test]
    fn test_18(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1583
    #[test]
    fn test_19(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1584
    #[test]
    fn test_20(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1585
    #[test]
    fn test_21(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1586
    #[test]
    fn test_22(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1587
    #[test]
    fn test_23(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1588
    #[test]
    fn test_24(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1589
    #[test]
    fn test_25(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1590
    #[test]
    fn test_26(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1591
    #[test]
    fn test_27(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1592
    #[test]
    fn test_28(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1593
    #[test]
    fn test_29(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1594
    #[test]
    fn test_30(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1595
    #[test]
    fn test_31(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1596
    #[test]
    fn test_32(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1597
    #[test]
    fn test_33(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1598
    #[test]
    fn test_34(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1599
    #[test]
    fn test_35(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1600
    #[test]
    fn test_36(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1601
    #[test]
    fn test_37(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1602
    #[test]
    fn test_38(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1603
    #[test]
    fn test_39(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1604
    #[test]
    fn test_40(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1605
    #[test]
    fn test_41(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1606
    #[test]
    fn test_42(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1607
    #[test]
    fn test_43(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1608
    #[test]
    fn test_44(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1609
    #[test]
    fn test_45(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4290772992 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1610
    #[test]
    fn test_46(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(4288675840 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1611
    #[test]
    fn test_47(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1612
    #[test]
    fn test_48(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1613
    #[test]
    fn test_49(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1614
    #[test]
    fn test_50(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1615
    #[test]
    fn test_51(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1616
    #[test]
    fn test_52(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1617
    #[test]
    fn test_53(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2143289344 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1618
    #[test]
    fn test_54(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2141192192 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("div", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1619
    #[test]
    fn test_55(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1620
    #[test]
    fn test_56(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1621
    #[test]
    fn test_57(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1622
    #[test]
    fn test_58(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1623
    #[test]
    fn test_59(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 1624
    #[test]
    fn test_60(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1625
    #[test]
    fn test_61(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 1626
    #[test]
    fn test_62(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1627
    #[test]
    fn test_63(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 1628
    #[test]
    fn test_64(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1629
    #[test]
    fn test_65(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 1630
    #[test]
    fn test_66(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1631
    #[test]
    fn test_67(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 1632
    #[test]
    fn test_68(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1633
    #[test]
    fn test_69(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 1634
    #[test]
    fn test_70(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1635
    #[test]
    fn test_71(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 1636
    #[test]
    fn test_72(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1637
    #[test]
    fn test_73(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 1638
    #[test]
    fn test_74(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1639
    #[test]
    fn test_75(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3234402267 as u32)));
    }
    

    // Command line number: 1640
    #[test]
    fn test_76(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1641
    #[test]
    fn test_77(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3234402267 as u32)));
    }
    

    // Command line number: 1642
    #[test]
    fn test_78(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1643
    #[test]
    fn test_79(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    

    // Command line number: 1644
    #[test]
    fn test_80(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1645
    #[test]
    fn test_81(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    

    // Command line number: 1646
    #[test]
    fn test_82(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1647
    #[test]
    fn test_83(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1648
    #[test]
    fn test_84(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1649
    #[test]
    fn test_85(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1650
    #[test]
    fn test_86(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1651
    #[test]
    fn test_87(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1652
    #[test]
    fn test_88(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1653
    #[test]
    fn test_89(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1654
    #[test]
    fn test_90(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483648 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1655
    #[test]
    fn test_91(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1656
    #[test]
    fn test_92(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1657
    #[test]
    fn test_93(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1658
    #[test]
    fn test_94(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(0 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1659
    #[test]
    fn test_95(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 1660
    #[test]
    fn test_96(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 1661
    #[test]
    fn test_97(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1662
    #[test]
    fn test_98(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1663
    #[test]
    fn test_99(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 1664
    #[test]
    fn test_100(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 1665
    #[test]
    fn test_101(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 1666
    #[test]
    fn test_102(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1667
    #[test]
    fn test_103(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 1668
    #[test]
    fn test_104(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 1669
    #[test]
    fn test_105(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 1670
    #[test]
    fn test_106(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1671
    #[test]
    fn test_107(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 1672
    #[test]
    fn test_108(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 1673
    #[test]
    fn test_109(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 1674
    #[test]
    fn test_110(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1675
    #[test]
    fn test_111(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 1676
    #[test]
    fn test_112(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 1677
    #[test]
    fn test_113(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 1678
    #[test]
    fn test_114(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1679
    #[test]
    fn test_115(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3234402267 as u32)));
    }
    

    // Command line number: 1680
    #[test]
    fn test_116(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 1681
    #[test]
    fn test_117(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3234402267 as u32)));
    }
    

    // Command line number: 1682
    #[test]
    fn test_118(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1683
    #[test]
    fn test_119(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    

    // Command line number: 1684
    #[test]
    fn test_120(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 1685
    #[test]
    fn test_121(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    

    // Command line number: 1686
    #[test]
    fn test_122(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1687
    #[test]
    fn test_123(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1688
    #[test]
    fn test_124(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 1689
    #[test]
    fn test_125(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1690
    #[test]
    fn test_126(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1691
    #[test]
    fn test_127(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1692
    #[test]
    fn test_128(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1693
    #[test]
    fn test_129(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1694
    #[test]
    fn test_130(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2147483649 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1695
    #[test]
    fn test_131(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1696
    #[test]
    fn test_132(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1697
    #[test]
    fn test_133(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1698
    #[test]
    fn test_134(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1699
    #[test]
    fn test_135(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 1700
    #[test]
    fn test_136(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 1701
    #[test]
    fn test_137(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1702
    #[test]
    fn test_138(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1703
    #[test]
    fn test_139(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 1704
    #[test]
    fn test_140(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 1705
    #[test]
    fn test_141(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 1706
    #[test]
    fn test_142(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1707
    #[test]
    fn test_143(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 1708
    #[test]
    fn test_144(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 1709
    #[test]
    fn test_145(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 1710
    #[test]
    fn test_146(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 1711
    #[test]
    fn test_147(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 1712
    #[test]
    fn test_148(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 1713
    #[test]
    fn test_149(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 1714
    #[test]
    fn test_150(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 1715
    #[test]
    fn test_151(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 1716
    #[test]
    fn test_152(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 1717
    #[test]
    fn test_153(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 1718
    #[test]
    fn test_154(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 1719
    #[test]
    fn test_155(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3234402267 as u32)));
    }
    

    // Command line number: 1720
    #[test]
    fn test_156(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 1721
    #[test]
    fn test_157(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3234402267 as u32)));
    }
    

    // Command line number: 1722
    #[test]
    fn test_158(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 1723
    #[test]
    fn test_159(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    

    // Command line number: 1724
    #[test]
    fn test_160(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 1725
    #[test]
    fn test_161(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    

    // Command line number: 1726
    #[test]
    fn test_162(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2139095039 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 1727
    #[test]
    fn test_163(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1728
    #[test]
    fn test_164(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 1729
    #[test]
    fn test_165(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(4286578688 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578688 as u32)));
    }
    

    // Command line number: 1730
    #[test]
    fn test_166(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2139095040 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 1731
    #[test]
    fn test_167(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1732
    #[test]
    fn test_168(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1733
    #[test]
    fn test_169(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1734
    #[test]
    fn test_170(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(2155872256 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1735
    #[test]
    fn test_171(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(4290772992 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1736
    #[test]
    fn test_172(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(4288675840 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1737
    #[test]
    fn test_173(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2143289344 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());
    }
    

    // Command line number: 1738
    #[test]
    fn test_174(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(8388608 as u32),Immediate::Word(2141192192 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());
    }
    

    // Command line number: 1739
    #[test]
    fn test_175(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 1740
    #[test]
    fn test_176(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 1741
    #[test]
    fn test_177(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(2147483648 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483648 as u32)));
    }
    

    // Command line number: 1742
    #[test]
    fn test_178(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(0 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(0 as u32)));
    }
    

    // Command line number: 1743
    #[test]
    fn test_179(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 1744
    #[test]
    fn test_180(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 1745
    #[test]
    fn test_181(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(2147483649 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2147483649 as u32)));
    }
    

    // Command line number: 1746
    #[test]
    fn test_182(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(1 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1 as u32)));
    }
    

    // Command line number: 1747
    #[test]
    fn test_183(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 1748
    #[test]
    fn test_184(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 1749
    #[test]
    fn test_185(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(2155872256 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(2155872256 as u32)));
    }
    

    // Command line number: 1750
    #[test]
    fn test_186(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(8388608 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(8388608 as u32)));
    }
    

    // Command line number: 1751
    #[test]
    fn test_187(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 1752
    #[test]
    fn test_188(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 1753
    #[test]
    fn test_189(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(3204448256 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 1754
    #[test]
    fn test_190(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(1056964608 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 1755
    #[test]
    fn test_191(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 1756
    #[test]
    fn test_192(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 1757
    #[test]
    fn test_193(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(3212836864 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3212836864 as u32)));
    }
    

    // Command line number: 1758
    #[test]
    fn test_194(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(1065353216 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 1759
    #[test]
    fn test_195(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3234402267 as u32)));
    }
    

    // Command line number: 1760
    #[test]
    fn test_196(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3204448256 as u32)));
    }
    

    // Command line number: 1761
    #[test]
    fn test_197(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(3234402267 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(3234402267 as u32)));
    }
    

    // Command line number: 1762
    #[test]
    fn test_198(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1056964608 as u32),Immediate::Word(1086918619 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1056964608 as u32)));
    }
    

    // Command line number: 1763
    #[test]
    fn test_199(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(3204448256 as u32),Immediate::Word(4286578687 as u32)];
        let result = runtime.call_exported_function("min", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(4286578687 as u32)));
    }
    
}


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

            let wasm_code = include_bytes!(concat!("../wasm_json_from_wast/", "spill.0.wasm"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    
    // Command line number: 388
    #[test]
    fn test_0(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2 as u32),Immediate::Word(3 as u32),Immediate::Word(4 as u32),Immediate::Word(5 as u32),Immediate::Word(6 as u32),Immediate::Word(7 as u32),Immediate::Word(8 as u32),Immediate::Word(9 as u32),Immediate::Word(10 as u32),Immediate::Word(11 as u32),Immediate::Word(12 as u32),Immediate::Word(13 as u32),Immediate::Word(14 as u32),Immediate::Word(15 as u32),Immediate::Word(16 as u32),Immediate::Word(17 as u32),Immediate::Word(18 as u32),Immediate::Word(19 as u32),Immediate::Word(20 as u32),Immediate::Word(21 as u32),Immediate::Word(22 as u32),Immediate::Word(23 as u32),Immediate::Word(24 as u32),Immediate::Word(25 as u32),Immediate::Word(26 as u32),Immediate::Word(27 as u32),Immediate::Word(28 as u32),Immediate::Word(29 as u32),Immediate::Word(30 as u32),Immediate::Word(31 as u32),Immediate::Word(32 as u32),Immediate::Word(33 as u32),Immediate::Word(34 as u32),Immediate::Word(35 as u32),Immediate::Word(36 as u32),Immediate::Word(37 as u32),Immediate::Word(38 as u32),Immediate::Word(39 as u32),Immediate::Word(40 as u32)];
        let result = runtime.call_exported_function("i32_right_deep_40", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(821 as u32)));
    }
    

    // Command line number: 400
    #[test]
    fn test_1(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1 as u32),Immediate::Word(2 as u32),Immediate::Word(3 as u32),Immediate::Word(4 as u32),Immediate::Word(5 as u32),Immediate::Word(6 as u32),Immediate::Word(7 as u32),Immediate::Word(8 as u32),Immediate::Word(9 as u32),Immediate::Word(10 as u32),Immediate::Word(11 as u32),Immediate::Word(12 as u32),Immediate::Word(13 as u32),Immediate::Word(14 as u32),Immediate::Word(15 as u32),Immediate::Word(16 as u32),Immediate::Word(17 as u32),Immediate::Word(18 as u32),Immediate::Word(19 as u32),Immediate::Word(20 as u32),Immediate::Word(21 as u32),Immediate::Word(22 as u32),Immediate::Word(23 as u32),Immediate::Word(24 as u32),Immediate::Word(25 as u32),Immediate::Word(26 as u32),Immediate::Word(27 as u32),Immediate::Word(28 as u32),Immediate::Word(29 as u32),Immediate::Word(30 as u32),Immediate::Word(31 as u32),Immediate::Word(32 as u32),Immediate::Word(33 as u32),Immediate::Word(34 as u32),Immediate::Word(35 as u32),Immediate::Word(36 as u32),Immediate::Word(37 as u32),Immediate::Word(38 as u32),Immediate::Word(39 as u32),Immediate::Word(40 as u32)];
        let result = runtime.call_exported_function("i32_left_deep_40", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(821 as u32)));
    }
    

    // Command line number: 412
    #[test]
    fn test_2(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(1 as u64),Immediate::DoubleWord(2 as u64),Immediate::DoubleWord(3 as u64),Immediate::DoubleWord(4 as u64),Immediate::DoubleWord(5 as u64),Immediate::DoubleWord(6 as u64),Immediate::DoubleWord(7 as u64),Immediate::DoubleWord(8 as u64),Immediate::DoubleWord(9 as u64),Immediate::DoubleWord(10 as u64),Immediate::DoubleWord(11 as u64),Immediate::DoubleWord(12 as u64),Immediate::DoubleWord(13 as u64),Immediate::DoubleWord(14 as u64),Immediate::DoubleWord(15 as u64),Immediate::DoubleWord(16 as u64),Immediate::DoubleWord(17 as u64),Immediate::DoubleWord(18 as u64),Immediate::DoubleWord(19 as u64),Immediate::DoubleWord(20 as u64),Immediate::DoubleWord(21 as u64),Immediate::DoubleWord(22 as u64),Immediate::DoubleWord(23 as u64),Immediate::DoubleWord(24 as u64),Immediate::DoubleWord(25 as u64),Immediate::DoubleWord(26 as u64),Immediate::DoubleWord(27 as u64),Immediate::DoubleWord(28 as u64),Immediate::DoubleWord(29 as u64),Immediate::DoubleWord(30 as u64),Immediate::DoubleWord(31 as u64),Immediate::DoubleWord(32 as u64),Immediate::DoubleWord(33 as u64),Immediate::DoubleWord(34 as u64),Immediate::DoubleWord(35 as u64),Immediate::DoubleWord(36 as u64),Immediate::DoubleWord(37 as u64),Immediate::DoubleWord(38 as u64),Immediate::DoubleWord(39 as u64),Immediate::DoubleWord(40 as u64)];
        let result = runtime.call_exported_function("i64_right_deep_40", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(821 as u64)));
    }
    

    // Command line number: 424
    #[test]
    fn test_3(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::Word(1065353216 as u32),Immediate::Word(1073741824 as u32),Immediate::Word(1077936128 as u32),Immediate::Word(1082130432 as u32),Immediate::Word(1084227584 as u32),Immediate::Word(1086324736 as u32),Immediate::Word(1088421888 as u32),Immediate::Word(1090519040 as u32),Immediate::Word(1091567616 as u32),Immediate::Word(1092616192 as u32),Immediate::Word(1093664768 as u32),Immediate::Word(1094713344 as u32),Immediate::Word(1095761920 as u32),Immediate::Word(1096810496 as u32),Immediate::Word(1097859072 as u32),Immediate::Word(1098907648 as u32),Immediate::Word(1099431936 as u32),Immediate::Word(1099956224 as u32),Immediate::Word(1100480512 as u32),Immediate::Word(1101004800 as u32),Immediate::Word(1101529088 as u32),Immediate::Word(1102053376 as u32),Immediate::Word(1102577664 as u32),Immediate::Word(1103101952 as u32),Immediate::Word(1103626240 as u32),Immediate::Word(1104150528 as u32),Immediate::Word(1104674816 as u32),Immediate::Word(1105199104 as u32),Immediate::Word(1105723392 as u32),Immediate::Word(1106247680 as u32),Immediate::Word(1106771968 as u32),Immediate::Word(1107296256 as u32),Immediate::Word(1107558400 as u32),Immediate::Word(1107820544 as u32),Immediate::Word(1108082688 as u32),Immediate::Word(1108344832 as u32),Immediate::Word(1108606976 as u32),Immediate::Word(1108869120 as u32),Immediate::Word(1109131264 as u32),Immediate::Word(1109393408 as u32)];
        let result = runtime.call_exported_function("f32_right_deep_40", args, Some(ValueSize::Word));
        assert_eq!(result, Some(Immediate::Word(1145913344 as u32)));
    }
    

    // Command line number: 436
    #[test]
    fn test_4(runtime : &mut WasmRuntime<'static>){
        let args = vec![Immediate::DoubleWord(4607182418800017408 as u64),Immediate::DoubleWord(4611686018427387904 as u64),Immediate::DoubleWord(4613937818241073152 as u64),Immediate::DoubleWord(4616189618054758400 as u64),Immediate::DoubleWord(4617315517961601024 as u64),Immediate::DoubleWord(4618441417868443648 as u64),Immediate::DoubleWord(4619567317775286272 as u64),Immediate::DoubleWord(4620693217682128896 as u64),Immediate::DoubleWord(4621256167635550208 as u64),Immediate::DoubleWord(4621819117588971520 as u64),Immediate::DoubleWord(4622382067542392832 as u64),Immediate::DoubleWord(4622945017495814144 as u64),Immediate::DoubleWord(4623507967449235456 as u64),Immediate::DoubleWord(4624070917402656768 as u64),Immediate::DoubleWord(4624633867356078080 as u64),Immediate::DoubleWord(4625196817309499392 as u64),Immediate::DoubleWord(4625478292286210048 as u64),Immediate::DoubleWord(4625759767262920704 as u64),Immediate::DoubleWord(4626041242239631360 as u64),Immediate::DoubleWord(4626322717216342016 as u64),Immediate::DoubleWord(4626604192193052672 as u64),Immediate::DoubleWord(4626885667169763328 as u64),Immediate::DoubleWord(4627167142146473984 as u64),Immediate::DoubleWord(4627448617123184640 as u64),Immediate::DoubleWord(4627730092099895296 as u64),Immediate::DoubleWord(4628011567076605952 as u64),Immediate::DoubleWord(4628293042053316608 as u64),Immediate::DoubleWord(4628574517030027264 as u64),Immediate::DoubleWord(4628855992006737920 as u64),Immediate::DoubleWord(4629137466983448576 as u64),Immediate::DoubleWord(4629418941960159232 as u64),Immediate::DoubleWord(4629700416936869888 as u64),Immediate::DoubleWord(4629841154425225216 as u64),Immediate::DoubleWord(4629981891913580544 as u64),Immediate::DoubleWord(4630122629401935872 as u64),Immediate::DoubleWord(4630263366890291200 as u64),Immediate::DoubleWord(4630404104378646528 as u64),Immediate::DoubleWord(4630544841867001856 as u64),Immediate::DoubleWord(4630685579355357184 as u64),Immediate::DoubleWord(4630826316843712512 as u64)];
        let result = runtime.call_exported_function("f64_right_deep_40", args, Some(ValueSize::DoubleWord));
        assert_eq!(result, Some(Immediate::DoubleWord(4650432808190214144 as u64)));
    }
    
}

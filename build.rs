use std::env;
use std::fs;
use std::path::Path;


static CODE_TEMPLATE: &str = r#"
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

            let wasm_code = include_bytes!(concat!("../mvp-tests/", "{filename}"));
            assert!(runtime.parse_and_translate(wasm_code).is_ok());
            runtime
        
    }

    {tests}
}
"#;

enum ReturnValue {
    ArithmeticNanF32,
    ArithmeticNanF64,
    CanonicalNanF32,
    CanonicalNanF64,
    Value(String),
}

fn escape_string_literal(input: &str) -> String {
    input.chars().map(|c| match c {
        '\\' => "\\\\".to_string(),
        '"' => "\\\"".to_string(),
        '\n' => "\\n".to_string(),
        '\r' => "\\r".to_string(),
        '\t' => "\\t".to_string(),
        '\u{20}'..='\u{7e}' => c.to_string(), // Printable ASCII range
        _ => format!("\\u{{{:x}}}", c as u32),
    }).collect()
}


fn main() {
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mvp_tests_dir = Path::new(&src_dir).join("mvp-tests");
    // get all json files in mvp_tests

    let json_files = fs::read_dir(&mvp_tests_dir).unwrap().filter_map(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            Some(path)
        } else {
            None
        }
    });
    for json_tests_file in json_files {
        let json_file = fs::read(&json_tests_file).unwrap();
        let json_tests = json_file.as_slice();

        //parse to json object
        let tests: serde_json::Value = serde_json::from_slice(json_tests).unwrap();
        //iterate over tests.commands
        let commands:Vec<_> = tests["commands"].as_array().unwrap().iter().map(|f|f.as_object().unwrap()).collect();
        
        let wasm_modules = commands.iter().filter(|cmd| cmd["type"].as_str().unwrap() == "module")
                                        .map(|cmd| cmd["filename"].as_str().unwrap()).collect::<Vec<_>>();                            



         commands.split(|cmd| cmd["type"].as_str().unwrap() == "module").skip(1).enumerate().for_each( |(wasm_module_index, test_suite)| {
            let filename = wasm_modules[wasm_module_index];
            println!("filename: {}", filename);
            let binding = test_suite.iter().filter(|cmd| vec!["assert_return","action"].contains(&cmd["type"].as_str().unwrap())).collect::<Vec<_>>();
            let testsuites = if binding.is_empty() {
                            vec![vec![]].into_iter()
                        } else {
                            binding.chunks(200).map(|chunk| chunk.to_vec()).collect::<Vec<_>>().into_iter()
                        };
            for (testsuite_index,test_suite) in testsuites.enumerate() { 
            let test_code = test_suite.iter().enumerate().map(|(test_index,cmd)| {
                    let line_number = cmd["line"].as_u64().unwrap();
                    let action = cmd["action"].as_object().unwrap();
                    let function_name = action["field"].as_str().unwrap();
                    let args = action["args"].as_array().unwrap().iter().map(|arg|{
                        let arg = arg.as_object().unwrap();
                        let arg_type = arg["type"].as_str().unwrap();
                        let arg_value = arg["value"].as_str().unwrap();
                        match arg_type{
                            "i32" => format!("Immediate::Word({} as u32)", arg_value),
                            "f32" => format!("Immediate::Word({} as u32)", arg_value),
                            "i64" | "f64" => format!("Immediate::DoubleWord({} as u64)", arg_value),
                            ty => panic!("Unknown type {}", ty)
                        }
                    }).collect::<Vec<_>>().join(",");
                    let (return_type,expected) = cmd["expected"].as_array().unwrap().iter().nth(0).map(|arg|{
                        let arg = arg.as_object().unwrap();
                        let arg_type: &str = arg["type"].as_str().unwrap();
                        let arg_value = arg["value"].as_str().unwrap();

                        let return_type = match arg_type{
                            "i32" | "f32" => "Some(ValueSize::Word)",
                            "i64" | "f64" => "Some(ValueSize::DoubleWord)",
                            ty => panic!("Unknown type {}", ty)
                        };

                        let expected = match (arg_type, arg_value){
                            ("f32", "nan:canonical") => ReturnValue::CanonicalNanF32,
                            ("f32", "nan:arithmetic") => ReturnValue::ArithmeticNanF32,
                            ("f64", "nan:canonical") => ReturnValue::CanonicalNanF64,
                            ("f64", "nan:arithmetic") => ReturnValue::ArithmeticNanF64,
                            ("f32",value) | ("i32", value) => ReturnValue::Value(format!("Some(Immediate::Word({} as u32))", value)),
                            ("f64",value) | ("i64", value) => ReturnValue::Value(format!("Some(Immediate::DoubleWord({} as u64))", value)),
                            (ty,_) => panic!("Unknown type {}", ty)
                        };

                         (return_type,expected)}).unwrap_or(("None", ReturnValue::Value("None".to_string())));
                    format!(
    r#"
    // Command line number: {line_number}
    #[test]
    fn test_{test_name}(runtime : &mut WasmRuntime<'static>){{
        let args = vec![{args}];
        let result = runtime.call_exported_function("{function_name}", args, {return_type});
        {assertion}
    }}
    "#,
                        line_number = line_number,
                        test_name = test_index,
                        args = args,
                        function_name = escape_string_literal(function_name),
                        return_type = return_type,
                        assertion = match expected {
                            ReturnValue::Value(expected) => format!("assert_eq!(result, {});", expected),
                            ReturnValue::ArithmeticNanF32 => format!("assert!(f32::from_bits(result.unwrap().as_u32()).is_nan());"),
                            ReturnValue::ArithmeticNanF64 => format!("assert!(f64::from_bits(result.unwrap().as_u64()).is_nan());"),
                            ReturnValue::CanonicalNanF32 => format!("assert!(result.unwrap().as_u32()==f32::NAN.to_bits() || result.unwrap().as_u32()==(-f32::NAN).to_bits());"),
                            ReturnValue::CanonicalNanF64 => format!("assert!(result.unwrap().as_u64()==f64::NAN.to_bits() || result.unwrap().as_u64()==(-f64::NAN).to_bits());")
                        }
                    )}).collect::<Vec<_>>().join("\n");


            let filename_parts: Vec<&str> = filename.split('.').collect();
            let output_path = Path::new(&src_dir).join("tests").join(if testsuite_index == 0  { format!("{}.rs", filename_parts[0..2].join("_")) } else {format!("{}_{}.rs", filename_parts[0..2].join("_"), testsuite_index)});
            let output = CODE_TEMPLATE.replace("{tests}", &test_code).replace("{filename}", filename);
            fs::write(&output_path, output).unwrap();
        }
    });
  }
}
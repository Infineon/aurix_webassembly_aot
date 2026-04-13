# Aurix Wasm Aot

## Required tools

- [Hightec Compiler for Aurix](https://hightec-rt.com/rust) min version 1.77.2-dev2
- [cargo-make](https://crates.io/crates/cargo-make)
- [wat2json included in WABT tools 1.36](https://github.com/WebAssembly/wabt/releases/tag/1.0.36) (Optional: Required if it is required to regenerate test cases from wast files store in wast_tests)


### Using instruction simulator

- Tricore Instruction simulator version >= 1.18.196
  - TSIM is included in PLS debugger. 
  - [A standalone free distribution is available from Infineon Developer center](https://softwaretools.infineon.com/tools/com.ifx.tb.tool.tsimtricoreinstructionsetsimulator). This is not automatically installed by `cargo make`
  - folder `<TSIM_path>/bin/<host architecture>` shall be added to the `PATH`
- [rustfilt](https://crates.io/crates/rustfilt)
  - This is installed automatically by `cargo-make` when running `cargo make test-board` or `cargo make test-tsim`
- [defmt-print](https://crates.io/crates/defmt-print) 
  - Install using latest rust stable version (no Hightec 1.0.0 because it is too old)
  - This is installed automatically by `cargo-make` when running `cargo make test-board` or `cargo make test-tsim`

### Using Aurix lite kit v2 board
- [Board Aurix lite kit v2 board](https://www.infineon.com/cms/en/product/promopages/AURIX-microcontroller-boards/low-cost-arduino-kits/aurix-tc375-lite-kit/)
- [tricore-probe version >= 2.1](https://github.com/veecle/tricore-probe)
  - This is not automatically installed by `cargo make` but it will check for its availability

### VSCode `rust-analyzer` extension

1.77.2 Rust compiler and related rust-analyzer is not compatible with latest version of VSCode `rust-analyzer` extension. It is recommended to use version `0.3.2011`

## Tests

Beside some unit tests most of test cases are generated from `*.wast` files of [w3c-1.0 core test](https://github.com/WebAssembly/spec/tree/w3c-1.0/test/core) (commit: f750d21dcc4903280b4db80ca81795968c5557f4) using `wast2json` . From json files are generated Rust integration test stored in `generated_tests`.

For your convenience generated files are stored in the repository to simply the enviroment setup.

In case you need to regenerate tests, `wast2json` tool proper version need to be available in the path.(see previous section for required tools.)

`cargo-make` will take care to regenerate required files in case of any change in `.wast` files when `cargo make <test-tsim|test-board>` are invoked 

Which test files are skipped and motivations are described [wast_tests/README.md](./wast_tests/README.md) and in *.wast file comments.



### How to run regression tests
Given the multitude of existing regression tests, it is recommended to run the tests on the instruction simulator rather than on physical hardware, especially if all or many tests are to be run. The regression tests do not test any performance and are used only to test the soundness of the implementation. Given that each test file/module is in a seperate binary, running all the tests involves flashing thousands of binaries to the memory.

To run all the tests:
```
cargo make <test-tsim|test-board>
```

To run a specific test or a specific subset of the tests:

```
cargo make <test-tsim|test-board> --test <name of integration test>
```

## How to run benchmark tests
The benchmark aims to provide time measures of running different algorithms. Hence, it is recommended to run it on the hardware.

To run the benchmark use the following command:
```
cargo make <run-benchmark-tsim|run-benchmark-board>
```

> All previous command accept extra cargo arguments that are appended to cargo command e.g `cargo make test-tsim --test address_0`

# How to use the AOT AURIX WASM Runtime:

The `WasmRuntime` trait provides 3 API functions that should be called in the listed order:

- `new`: a constructor that returns a `WasmRuntime` after being provided of memory arrays that will be used to host the main components of the runtime : compiled machine instructions, linear memory, (extern calls) table and global space. 
- `parse_and_translate`: given a Wasm module in binary form as a `&[u8]` as input, the method instantiates the module. Currently, the runtime does not support instantiating more than one module at a time. Therefore, once called, the function removes any existing module instance. The instantiation of a module initializes the linear memory, the table and the global space and compiles the WebAssembly functions to native code. Currently, the function supposes that the WASM module is valid and uses only features of the MVP. The instantiation does also run the start function if existent.
- `call_exported_function` given a function name, arguments and an expected result type, this method runs the function that is exported with the given name, after providing it with the arguments. Depending on the expected result type, the method returns the result provided by the function. Arguments and results are represented as unsigned integers with the same bit-size as their respective value type. i.e: `i32` and `f32` are represented as `u32` integers wrapped in a `Immediate::Word` constructor and `i64` and `f64` are represented as `f64` integers wrapped in a `Immediate::DoubleWord` constructor. Currently, the runtime assumes that an exported function under the given name exists and that the provided arguments and result type correspond to that function's type.

The Runtime uses heap allocation for storing the IR (intermediate representation), also known as Valent-Blocks (VB). Therefore, a global allocator is necessary for running the translation. We recommend using `embedded-alloc` like in the benchmark example.

The benchmark code provides an example for constructing the Wasm Runtime, instantiating a module and calling an exported function.

# Limitations

Not all features of standard Webassembly runtime and translator are implemented.
Below a list of major limitation and deviation from Webassembly standard

* No features beyond WebAssembly 1.0 (exception for page size 64KB)
* No trap e.g:
  * Load and store sandbox implemented by address masking. No trap generated code continue to be executed.
  * Division by zero
  * ....
* No time protection. It is not possible to limit the execution time of a Webassembly function.
* Denormal floating point are not supported. Denormal floating point are not supported by Aurix FPU
* No module linking. 
  * Limited support for calling host function (only for demo purpose)
* Presently the generated code calls functions in libm and the core library.


# License 

This is software is licensed under MIT(Infineon) and BOOST license (Hightec)

This is software includes part of software developed by Hightec

[License text available here](./LICENSE.txt)

# Alternatives

- [A WASM AoT traslator implemented in C++ and supporting Aurix/x86_64/aarch64](https://github.com/wasm-ecosystem/wasm-compiler)

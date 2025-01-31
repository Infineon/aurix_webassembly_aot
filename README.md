# Aurix Wasm Aot

## Required tools

- [Hightec Compiler for Aurix](https://hightec-rt.com/rust) 

### Using instruction simulator

- Tricore Instruction simulator version >= 1.18.196
  - TSIM is included in PLS debugger. A standalone free distribution is in development. In the meantime, please create an issue [here](https://mycases.infineon.com/) mentioning that this is required for Aurix WebAssembly AoT to get TSIM.
  - folder `<TSIM_path>/bin/<host architecture>` shall be added to the `PATH`
- [rustfilt](https://crates.io/crates/rustfilt)
- [defmt-print](https://crates.io/crates/defmt-print) 
  - Install using latest rust stable version (no Hightec 1.0.0 because it is too old)

### Using Aurix lite kit v2 board
- [Board Aurix lite kit v2 board](https://www.infineon.com/cms/en/product/promopages/AURIX-microcontroller-boards/low-cost-arduino-kits/aurix-tc375-lite-kit/)
- [tricore-probe version >= 2.1](https://github.com/veecle/tricore-probe)


## How to switch from simulator execution to board execution

By default the project is configured to be compiled and executed for TSIM. To switch to board execution follow the following steps:

- Edit file `.cargo/config.toml`, comment out `runner = ".\\instruction_simulator_runner.bat"` and comment in  `runner = "tricore-probe.exe -c 1"`


## How to use the AOT AURIX WASM Runtime:

The `WasmRuntime` trait provides 3 API functions that should be called in the listed order:

- `new`: a constructor that returns a `WasmRuntime` after being provided of memory arrays that will be used to host the main components of the runtime : compiled machine instructions, linear memory, (extern calls) table and global space. 
- `parse_and_translate`: given a Wasm module in binary form as a `&[u8]` as input, the method instantiates the module. Currently, the runtime does not support instantiating more than one module at a time. Therefore, once called, the function removes any existing module instance. The instantiation of a module initializes the linear memory, the table and the global space and compiles the WebAssembly functions to native code. Currently, the function supposes that the WASM module is valid and uses only features of the MVP. The instantiation does also run the start function if existent.
- `call_exported_function` given a function name, arguments and an expected result type, this method runs the function that is exported with the given name, after providing it with the arguments. Depending on the expected result type, the method returns the result provided by the function. Arguments and results are represented as unsigned integers with the same bit-size as their respective value type. i.e: `i32` and `f32` are represented as `u32` integers wrapped in a `Immediate::Word` constructor and `i64` and `f64` are represented as `f64` integers wrapped in a `Immediate::DoubleWord` constructor. Currently, the runtime assumes that an exported function under the given name exists and that the provided arguments and result type correspond to that function's type.

The Runtime uses heap allocation for storing the IR (intermediate representation), also known as Valent-Blocks (VB). Therefore, a global allocator is necessary for running the translation. We recommend using `embedded-alloc` like in the benchmark example.

The benchmark code provides an example for constructing the Wasm Runtime, instantiating a module and calling an exported function.

## How to run regression tests
Given the multitude of existing regression tests, it is recommended to run the tests on the instruction simulator rather than on physical hardware, especially if all or many tests are to be run. The regression tests do not test any performance and are used only to test the soundness of the implementation. Given that each test file/module is in a seperate binary, running all the tests involves flashing thousands of binaries to the memory.

To run all the tests:
```
cargo test --features tsim 
```

To run a specific test or a specific subset of the tests:

```
cargo test --features=<tsim|board> --test <name of integration test>
```

## How to run benchmark tests
The benchmark aims to provide time measures of running different algorithms. Hence, it is recommended to run it on the hardware.

To run the benchmark use the following command:
```
cargo run --features=<tsim|board> --example benchmark
```
# License 

This is software is licensed under MIT(Infineon) and BOOST license (Hightec)

This is software includes part of software developed by Hightec

[License text available here](./LICENSE.txt)
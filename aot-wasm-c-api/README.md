# aot-wasm-c-api

C FFI bindings for the `aot_wasm` WebAssembly Ahead-of-Time compiler runtime on Infineon AURIX TriCore.

## Purpose

This crate produces a **static library** (`libaot_wasm_c_api.a`) that allows C applications to use the `WasmRuntime` without any knowledge of Rust internals. The runtime is exposed as an opaque pointer type (`wasm_runtime_t *`).

## Building

```bash
cargo build --release -p aot-wasm-c-api
```

The static library will be located under `target/<triple>/release/libaot_wasm_c_api.a`.

## C Header

A ready-to-use C header is provided at [`include/aot_wasm_c_api.h`](include/aot_wasm_c_api.h).

## API Summary

| Function                            | Description                                                   |
|-------------------------------------|---------------------------------------------------------------|
| `wasm_runtime_env_init`             | Initialise the heap allocator (call once at startup).         |
| `wasm_runtime_create`               | Create a new runtime instance (validates all arguments).      |
| `wasm_runtime_destroy`              | Destroy a runtime, freeing internal allocations.              |
| `wasm_runtime_parse_and_translate`  | Load and compile a `.wasm` binary.                            |
| `wasm_runtime_call`                 | Call an exported Wasm function by name.                       |
| `wasm_runtime_get_function_size`    | Query the machine-code size of a function.                    |

## Cargo Features

| Feature                   | Default | Description                                                                                         |
|---------------------------|---------|-----------------------------------------------------------------------------------------------------|
| `address-masking`         | yes     | Enable address-masking bounds checking (requires `linear_memory_len == 2^k + 7`).                  |
| `full_instructions`       | yes     | Enable the full Wasm instruction set.                                                               |
| `misaligned-access`       | yes     | Allow misaligned memory accesses in translated code.                                                |
| `panic-handler-callback`  | yes     | On panic, call the external C symbol `void panic_handler(const uint8_t*, size_t)` you must define. |

## Constraints

See the comprehensive documentation in `include/aot_wasm_c_api.h` and the Rust doc-comments in `src/lib.rs`.

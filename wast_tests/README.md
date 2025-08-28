This folder contains tests for the core WebAssembly semantics, as described in [Semantics.md](https://github.com/WebAssembly/design/blob/master/Semantics.md) and specified by the [spec interpreter](https://github.com/WebAssembly/spec/blob/master/interpreter).

This test has been derived from the one stored here:
https://github.com/WebAssembly/spec/tree/w3c-1.0 (commit: f750d21dcc4903280b4db80ca81795968c5557f4)


# Changes

 * Added test file `wast_tests/wrap-around-memory.wast` to test the address wrapping for load and store

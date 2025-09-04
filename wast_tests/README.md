This folder contains tests for the core WebAssembly semantics, as described in [Semantics.md](https://github.com/WebAssembly/design/blob/master/Semantics.md) and specified by the [spec interpreter](https://github.com/WebAssembly/spec/blob/master/interpreter).

This test has been derived from the one stored here:
https://github.com/WebAssembly/spec/tree/w3c-1.0 (commit: f750d21dcc4903280b4db80ca81795968c5557f4)

# Ignored assertions

Presently `tests_generator` recognize in generated  json files, only entries with type `action` and `assert_return`.
This means that in wast all directives are ignored except `invoke` and `assert_return` 


# Changes

 * Added test file `wast_tests/wrap-around-memory.wast` to test the address wrapping for load and store
 * Disabled wast by renaming extensions to .dwast
   * export.wast --> partially supported feature disable for the time being
   * linking.wast --> unsupported feature
   * binary-leb128.wast --> it is only parsing test. Project relay on wasmtime parser and assume it is correct.
   * imports.wast --> partially supported feature disable for the time being
   * names.wast --> Requires export keyword
   * skip-stack-guard-page.wast --> Aurix core can be configured to limit stack size
 * Check inside the *.wast files for other changes marked with regular expression `:[A-Z_]+:`
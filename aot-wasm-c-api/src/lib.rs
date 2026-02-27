//! # aot-wasm-c-api — C FFI for the aot_wasm WebAssembly AOT Runtime
//!
//! This crate produces a **static library** (`libaot_wasm_c_api.a`) that exposes
//! the [`aot_wasm`] WebAssembly Ahead-of-Time compiler runtime to C applications
//! running on Infineon AURIX TriCore microcontrollers.
//!
//! ## Overview
//!
//! The library presents `WasmRuntime` as an **opaque pointer** (`wasm_runtime_t*`)
//! to C code. All interaction goes through the C functions declared in this module.
//!
//! ## Typical C usage
//!
//! ```c
//! #include <stdint.h>
//! #include <stddef.h>
//!
//! /* ---- Types (opaque) ---- */
//! typedef struct wasm_runtime wasm_runtime_t;
//!
//! /* ---- Environment init ---- */
//! extern int32_t  wasm_runtime_env_init(
//!     uint8_t *heap_buf, size_t heap_size);
//!
//! /* ---- Lifecycle ---- */
//! extern wasm_runtime_t *wasm_runtime_create(
//!     uint32_t *instructions, size_t instructions_len,
//!     uint8_t  *linear_memory, size_t linear_memory_len,
//!     uint8_t  *global_space,  size_t global_space_len,
//!     uint32_t *table,         size_t table_len);
//! extern void     wasm_runtime_destroy(wasm_runtime_t *rt);
//!
//! /* ---- Module loading ---- */
//! extern int32_t  wasm_runtime_parse_and_translate(
//!     wasm_runtime_t *rt,
//!     const uint8_t  *wasm_code, size_t wasm_code_len);
//!
//! /* ---- Execution ---- */
//! extern int32_t  wasm_runtime_call(
//!     wasm_runtime_t *rt,
//!     const char     *func_name,
//!     const uint32_t *args_words, size_t args_words_len,
//!     uint8_t         return_kind,
//!     uint64_t       *out_result);
//!
//! /* ---- Utilities ---- */
//! extern int32_t  wasm_runtime_get_function_size(
//!     const wasm_runtime_t *rt,
//!     const char *func_name);
//! ```
//!
//! ## Constraints & Rules
//!
//! 1. **Environment initialisation must come first.**
//!    Call [`wasm_runtime_env_init`] exactly once before any other function in
//!    this library. The heap buffer you pass must live for the entire program
//!    lifetime (typically a static array in `.bss`).
//!
//! 2. **Memory buffers must be statically allocated.**
//!    The `instructions`, `linear_memory`, `global_space`, and `table` arrays
//!    passed to [`wasm_runtime_create`] must remain valid and exclusively owned
//!    by the runtime until [`wasm_runtime_destroy`] is called.
//!    On bare-metal TriCore targets these should be placed in appropriate linker
//!    sections (e.g. `.CPU0.ramcode` for instructions, `.CPU0.data` for data).
//!
//! 3. **`linear_memory` alignment when address-masking is enabled (default).**
//!    The length of `linear_memory` **must** equal `(power_of_two) + 7`.
//!    For example `(1 << 16) + 7 = 65543`. The extra 7 bytes form a guard
//!    buffer required by the address-masking bounds-checking strategy.
//!    Without the `address-masking` feature the size only needs to be > 0.
//!
//! 4. **All buffers must be 4-byte aligned.**
//!    `instructions`, `linear_memory`, `global_space`, and `table` pointers
//!    must be aligned to at least 4 bytes.
//!
//! 5. **Single-threaded use only.**
//!    The runtime is **not** thread-safe. Do not call any API function from
//!    multiple threads (or TriCore cores) concurrently on the same runtime
//!    instance.
//!
//! 6. **One runtime instance per set of buffers.**
//!    Do not share the same instruction / memory buffer between two runtime
//!    instances.
//!
//! 7. **Destroying the runtime releases internal heap allocations.**
//!    After [`wasm_runtime_destroy`], the pointer is invalid. The backing
//!    static buffers can be reused to create a new runtime.
//!
//! 8. **`wasm_runtime_call` argument encoding.**
//!    Arguments are passed as an array of `uint32_t` words. A Wasm `i32`/`f32`
//!    occupies 1 word; a Wasm `i64`/`f64` occupies 2 consecutive words
//!    (little-endian, low word first). The `return_kind` parameter is:
//!    - `0` → no return value,
//!    - `1` → 32-bit return (`i32` / `f32`),
//!    - `2` → 64-bit return (`i64` / `f64`).
//!
//! 9. **Panic handler callback (feature `panic-handler-callback`).**
//!    When the `panic-handler-callback` Cargo feature is enabled (default),
//!    the panic handler will call an external C symbol with the signature
//!    `void panic_handler(const uint8_t *msg, size_t msg_len)` that the
//!    **caller must provide** (define and link). The message is UTF-8 encoded
//!    and **not** null-terminated. The function must **never return** —
//!    after it returns the implementation enters an infinite loop.
//!    If the feature is disabled, panics loop silently forever.

#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(stdsimd)]
extern crate alloc;

use alloc::vec::Vec;
use core::{slice};

use aot_wasm::isa_model::{Immediate, ValueSize};
use aot_wasm::parse_and_translate::WasmRuntime;
use aot_wasm::Heap;

// ---------------------------------------------------------------------------
// Global allocator — initialised once via `wasm_runtime_env_init`.
// ---------------------------------------------------------------------------
#[global_allocator]
static HEAP: Heap = Heap::empty();

// ---------------------------------------------------------------------------
// Panic handler (link-time external symbol)
// ---------------------------------------------------------------------------



extern "C" {
    /// External C function called by the panic handler when the
    /// `panic-handler-callback` feature is enabled.
    ///
    /// The **caller** (C application) must define and link a function with
    /// exactly this signature:
    ///
    /// ```c
    /// void panic_handler(const uint8_t *msg, size_t msg_len);
    /// ```
    ///
    /// `msg` points to a UTF-8 encoded, **not** null-terminated description
    /// of the panic. `msg_len` is its length in bytes.
    ///
    /// # Contract
    ///
    /// The function **must not return**. After it returns the panic handler
    /// enters an infinite loop. It must not call any `wasm_runtime_*`
    /// function or allocate heap memory — the runtime may be in an
    /// inconsistent state.
    #[cfg(feature = "panic-handler-callback")]
    fn panic_handler(msg: *const u8, msg_len: usize);}



/// Rust panic handler.
///
/// When the `panic-handler-callback` feature is enabled (default), the
/// formatted panic message is forwarded to the external C symbol
/// `panic_handler` that the caller must supply at link time.
/// Regardless of whether that function returns, execution enters an
/// infinite loop (panics must diverge on bare-metal).
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // Try to invoke the registered callback.
    
    #[cfg(feature = "panic-handler-callback")]
    {
        // Format the panic info into a small stack buffer.
        // On no_std we use a fixed-size buffer to avoid heap allocation
        // (the allocator itself may have panicked).
        use core::fmt::Write;
        let mut buf = StackWriter::new();
        let _ = write!(buf, "{}", info);
        let msg = buf.as_bytes();
        unsafe {
            panic_handler(msg.as_ptr(), msg.len());
        }
    }
    loop {}
}

/// A small stack-allocated buffer that implements `core::fmt::Write`.
///
/// Used inside the panic handler to format the panic message without
/// requiring a heap allocation.
struct StackWriter {
    buf: [u8; 256],
    pos: usize,
}

impl StackWriter {
    fn new() -> Self {
        Self {
            buf: [0u8; 256],
            pos: 0,
        }
    }
    fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.pos]
    }
}

impl core::fmt::Write for StackWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let remaining = self.buf.len() - self.pos;
        let to_copy = bytes.len().min(remaining);
        self.buf[self.pos..self.pos + to_copy].copy_from_slice(&bytes[..to_copy]);
        self.pos += to_copy;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Return codes
// ---------------------------------------------------------------------------

/// Success.
const WASM_OK: i32 = 0;
/// A required pointer argument was null.
const WASM_ERR_NULL_PTR: i32 = -1;
/// The Wasm binary could not be parsed / translated.
const WASM_ERR_PARSE: i32 = -2;
/// The requested export function was not found.
const WASM_ERR_NOT_FOUND: i32 = -3;
/// An invalid argument value was provided.
const WASM_ERR_INVALID_ARG: i32 = -4;

// ---------------------------------------------------------------------------
// Environment initialisation
// ---------------------------------------------------------------------------

/// Initialise the runtime environment: heap allocator.
///
/// This function **must** be called **exactly once**, before any other
/// function in this library.
///
/// # Arguments
///
/// * `heap_buf`  — pointer to a memory buffer used as the heap. Must
///                 remain valid for the **entire program lifetime**.
/// * `heap_size` — size of the heap buffer in bytes.
///
/// # Panic handler
///
/// When the `panic-handler-callback` Cargo feature is enabled (default),
/// the panic handler will call the external C symbol `panic_handler` at
/// link time. The caller must define and link this function; see constraint
/// 9 in the module documentation.
///
/// # Safety
///
/// * `heap_buf` must point to at least `heap_size` bytes.
///
/// # Returns
///
/// * `0` ([`WASM_OK`]) on success.
/// * `-1` ([`WASM_ERR_NULL_PTR`]) if `heap_buf` is null.
/// * `-4` ([`WASM_ERR_INVALID_ARG`]) if `heap_size` is 0.
#[no_mangle]
pub unsafe extern "C" fn wasm_runtime_env_init(
    heap_buf: *mut u8,
    heap_size: usize,
) -> i32 {
    if heap_buf.is_null() {
        return WASM_ERR_NULL_PTR;
    }
    if heap_size == 0 {
        return WASM_ERR_INVALID_ARG;
    }
    // Initialize the heap allocator.
    HEAP.init(heap_buf as usize, heap_size);
    WASM_OK
}

// ---------------------------------------------------------------------------
// Runtime lifecycle
// ---------------------------------------------------------------------------

/// Create a new [`WasmRuntime`] instance.
///
/// The runtime is returned as an opaque pointer. All backing buffers must
/// remain valid and must **not** be aliased until [`wasm_runtime_destroy`] is
/// called.
///
/// # Safety
///
/// * All pointer / length pairs must describe valid, exclusively-owned,
///   4-byte-aligned memory regions.
/// * `linear_memory_len` must equal `(power_of_two) + 7` (e.g. `32768 + 7`).
/// * The heap allocator must have been initialised beforehand via
///   [`wasm_runtime_env_init`].
///
/// # Returns
///
/// A non-null opaque pointer on success, or `null` on failure:
/// * any pointer argument is null, or any length is zero
/// * any buffer is not 4-byte aligned
/// * `linear_memory_len` does not satisfy `(power_of_two) + 7`
#[no_mangle]
pub unsafe extern "C" fn wasm_runtime_create(
    instructions: *mut u32,
    instructions_len: usize,
    linear_memory: *mut u8,
    linear_memory_len: usize,
    global_space: *mut u8,
    global_space_len: usize,
    table: *mut u32,
    table_len: usize,
) -> *mut WasmRuntime<'static> {
    // Validate pointers
    if instructions.is_null()
        || linear_memory.is_null()
        || global_space.is_null()
        || table.is_null()
    {
        return core::ptr::null_mut();
    }
    if instructions_len == 0
        || linear_memory_len == 0
        || global_space_len == 0
        || table_len == 0
    {
        return core::ptr::null_mut();
    }
    // Validate 4-byte alignment
    if (instructions as usize) % 4 != 0
        || (instructions as usize) % 4 != 0
        || (linear_memory as usize) % 4 != 0
        || (global_space as usize) % 4 != 0
        || (table as usize) % 4 != 0
    {
        return core::ptr::null_mut();
    }

     // With address-masking enabled, we need at least 8 bytes to have a non-empty memory (1 byte) plus the 7-byte guard buffer.
    if linear_memory_len < 8 || !(linear_memory_len-7).is_power_of_two() {
        return core::ptr::null_mut();
    }

    let instr_slice = slice::from_raw_parts_mut(instructions, instructions_len);
    let linear_memory_slice = slice::from_raw_parts_mut(linear_memory, linear_memory_len);
    
    let global_slice = slice::from_raw_parts_mut(global_space, global_space_len);
    let table_slice = slice::from_raw_parts_mut(table, table_len);
    
        let runtime = unsafe {WasmRuntime::new_raw(
            instr_slice,
            linear_memory_slice,
            global_slice,
            table_slice,
        )};

    // Heap-allocate the runtime so we can hand out a stable pointer.
    let boxed = alloc::boxed::Box::new(runtime);
    alloc::boxed::Box::into_raw(boxed)
}

/// Destroy a runtime instance previously created with [`wasm_runtime_create`].
///
/// This releases all heap-allocated internal data structures (function labels,
/// type info, export map, etc.). The backing static buffers (`instructions`,
/// `linear_memory`, `global_space`, `table`) are **not** freed — they are
/// assumed to be statically allocated and may be reused.
///
/// # Safety
///
/// * `rt` must be a pointer previously returned by [`wasm_runtime_create`] and
///   must not have been destroyed already.
/// * After this call the pointer is **invalid** and must not be used again.
/// * Passing a null pointer is a safe no-op.
#[no_mangle]
pub unsafe extern "C" fn wasm_runtime_destroy(rt: *mut WasmRuntime<'static>) {
    if !rt.is_null() {
        // Reconstruct the Box so that Drop runs and memory is freed.
        let _ = alloc::boxed::Box::from_raw(rt);
    }
}

// ---------------------------------------------------------------------------
// Module loading
// ---------------------------------------------------------------------------

/// Parse a WebAssembly binary and translate it to native TriCore machine code.
///
/// This fills the instruction buffer, initialises linear memory and globals,
/// and populates the export table. The runtime is then ready for
/// [`wasm_runtime_call`].
///
/// Calling this function again on the same runtime loads a **new** module,
/// overwriting the previous one (the internal function labels and instruction
/// count are reset).
///
/// # Safety
///
/// * `rt` must be a valid runtime pointer from [`wasm_runtime_create`].
/// * `wasm_code` must point to `wasm_code_len` bytes of a valid `.wasm`
///   binary.
///
/// # Returns
///
/// * `0` ([`WASM_OK`]) on success.
/// * `-1` ([`WASM_ERR_NULL_PTR`]) if any pointer is null.
/// * `-2` ([`WASM_ERR_PARSE`]) if parsing / translation fails.
#[no_mangle]
pub unsafe extern "C" fn wasm_runtime_parse_and_translate(
    rt: *mut WasmRuntime<'static>,
    wasm_code: *const u8,
    wasm_code_len: usize,
) -> i32 {
    if rt.is_null() || wasm_code.is_null() {
        return WASM_ERR_NULL_PTR;
    }
    let runtime = &mut *rt;
    let code = slice::from_raw_parts(wasm_code, wasm_code_len);

    match runtime.parse_and_translate(code) {
        Ok(()) => WASM_OK,
        Err(_) => WASM_ERR_PARSE,
    }
}

// ---------------------------------------------------------------------------
// Function invocation
// ---------------------------------------------------------------------------

/// Call an exported WebAssembly function by name.
///
/// ## Argument encoding
///
/// Arguments are passed as a flat array of `uint32_t` words (`args_words`).
/// * A Wasm **i32 / f32** value occupies **1** word.
/// * A Wasm **i64 / f64** value occupies **2** consecutive words, stored
///   **little-endian** (low word at the lower index).
///
/// ## Return-kind encoding (`return_kind`)
///
/// | Value | Meaning                              |
/// |-------|--------------------------------------|
/// | `0`   | No return value (void).              |
/// | `1`   | 32-bit return (`i32` / `f32`).       |
/// | `2`   | 64-bit return (`i64` / `f64`).       |
///
/// When `return_kind` is non-zero, the result is written to `*out_result` as a
/// `uint64_t`. For 32-bit returns the upper 32 bits are zero.
///
/// # Safety
///
/// * `rt` must be a valid runtime pointer with a module already loaded via
///   [`wasm_runtime_parse_and_translate`].
/// * `func_name` must be a valid null-terminated C string.
/// * `args_words` may be null only if `args_words_len` is 0.
/// * `out_result` may be null if `return_kind` is 0.
///
/// # Returns
///
/// * `0` ([`WASM_OK`]) on success.
/// * `-1` ([`WASM_ERR_NULL_PTR`]) if `rt` or `func_name` is null.
/// * `-3` ([`WASM_ERR_NOT_FOUND`]) if no export with the given name exists.
/// * `-4` ([`WASM_ERR_INVALID_ARG`]) if `return_kind` is out of range or
///   `out_result` is null when a return is expected.
#[no_mangle]
pub unsafe extern "C" fn wasm_runtime_call(
    rt: *mut WasmRuntime<'static>,
    func_name: *const core::ffi::c_char,
    args_words: *const u32,
    args_words_len: usize,
    return_kind: u8,
    out_result: *mut u64,
) -> i32 {
    if rt.is_null() || func_name.is_null() {
        return WASM_ERR_NULL_PTR;
    }
    if return_kind > 2 {
        return WASM_ERR_INVALID_ARG;
    }

    let runtime = &mut *rt;

    // Convert C string to &str
    let name_cstr = core::ffi::CStr::from_ptr(func_name);
    let name_str = match name_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return WASM_ERR_INVALID_ARG,
    };

    // Check the export exists
    if runtime.export_map.get(name_str).is_none() {
        return WASM_ERR_NOT_FOUND;
    }

    // Build the arguments vector
    let words = if args_words.is_null() || args_words_len == 0 {
        &[]
    } else {
        slice::from_raw_parts(args_words, args_words_len)
    };
    let args = words_to_immediates(words);

    let return_size = match return_kind {
        0 => None,
        1 => Some(ValueSize::Word),
        2 => Some(ValueSize::DoubleWord),
        _ => return WASM_ERR_INVALID_ARG,
    };

    // Validate out_result when a return is expected
    if return_size.is_some() && out_result.is_null() {
        return WASM_ERR_INVALID_ARG;
    }

    let result = runtime.call_exported_function(name_str, args, return_size);

    match result {
        Some(Immediate::Word(v)) => {
            *out_result = v as u64;
        }
        Some(Immediate::DoubleWord(v)) => {
            *out_result = v;
        }
        None => {}
    }

    WASM_OK
}

// ---------------------------------------------------------------------------
// Utilities
// ---------------------------------------------------------------------------

/// Get the size (in bytes) of a translated function's machine code.
///
/// Useful for benchmarking and code-size analysis.
///
/// # Safety
///
/// * `rt` must be a valid runtime pointer with a module already loaded.
/// * `func_name` must be a valid null-terminated C string naming an exported
///   function.
///
/// # Returns
///
/// The function size in bytes on success (≥ 0), or a negative error code:
/// * `-1` ([`WASM_ERR_NULL_PTR`]) if any pointer is null.
/// * `-3` ([`WASM_ERR_NOT_FOUND`]) if the function is not found — however the
///   current implementation will panic in that case; prefer checking existence
///   first.
#[no_mangle]
pub unsafe extern "C" fn wasm_runtime_get_function_size(
    rt: *const WasmRuntime<'static>,
    func_name: *const core::ffi::c_char,
) -> i32 {
    if rt.is_null() || func_name.is_null() {
        return WASM_ERR_NULL_PTR;
    }
    let runtime = &*rt;

    let name_cstr = core::ffi::CStr::from_ptr(func_name);
    let name_str = match name_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return WASM_ERR_INVALID_ARG,
    };

    if runtime.export_map.get(name_str).is_none() {
        return WASM_ERR_NOT_FOUND;
    }

    runtime.get_function_size(name_str)
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Convert a flat array of `u32` words into a `Vec<Immediate>`.
///
/// Each word is treated as a 32-bit `Immediate::Word`. If the caller needs to
/// pass 64-bit values, they should place two consecutive words (low, high) and
/// this helper will combine them. However, because the underlying
/// `call_exported_function` internally re-encodes from `Immediate`, we pass
/// each word individually — the Wasm function's type signature determines how
/// the arguments are consumed from the stack.
fn words_to_immediates(words: &[u32]) -> Vec<Immediate> {
    words.iter().map(|&w| Immediate::Word(w)).collect()
}


// ---------------------------------------------------------------------------
// Critical section implementation
// ---------------------------------------------------------------------------

/// Single-core critical section implementation for `embedded_alloc`.
///
/// `embedded_alloc` requires a `critical-section` implementation to make
/// heap allocation interrupt-safe. On TriCore this is done by disabling
/// interrupts (`DISABLE`) on acquire and restoring the previous interrupt
/// enable state (`ENABLE`) on release.
use critical_section::RawRestoreState;

struct SingleCoreCriticalSection;
critical_section::set_impl!(SingleCoreCriticalSection);

unsafe impl critical_section::Impl for SingleCoreCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let old_value = core::arch::tricore::intrinsics::__disable();
        old_value
    }

    unsafe fn release(was_active: RawRestoreState) {
        if was_active {
            core::arch::tricore::intrinsics::__enable();
        }
    }
}


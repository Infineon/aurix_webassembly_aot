/**
 * @file aot_wasm_c_api.h
 * @brief C API for the aot_wasm WebAssembly Ahead-of-Time Compiler Runtime
 *        on Infineon AURIX TriCore.
 *
 * This header declares the public C interface to the Rust-based aot_wasm
 * runtime. The runtime ahead-of-time compiles WebAssembly modules into native
 * TriCore machine code and executes exported functions.
 *
 * @section usage Usage
 *
 * @code
 * #include "aot_wasm_c_api.h"
 *
 * // 1. Provide a heap buffer (must be static / global lifetime)
 * static uint8_t heap_buf[10 * 1024];
 *
 * // 2. Provide runtime backing buffers (place in appropriate linker sections)
 * __attribute__((section(".CPU0.ramcode"), aligned(4)))
 * static uint32_t instructions[4096];
 *
 * // When address-masking is enabled (default), length = (power_of_two) + 7
 * __attribute__((section(".CPU0.data"), aligned(4)))
 * static uint8_t linear_memory[(1u << 16) + 7];
 *
 * __attribute__((section(".CPU0.data"), aligned(4)))
 * static uint8_t global_space[256];
 *
 * __attribute__((section(".CPU0.data"), aligned(4)))
 * static uint32_t table[256];
 *
 * void my_panic_handler(const uint8_t *msg, size_t msg_len)
 * {
 *     // Called at link time when a Rust panic occurs (requires the
 *     // panic-handler-callback Cargo feature, enabled by default).
 *     // WARNING: do NOT call wasm_runtime_* functions here.
 *     my_uart_write(msg, msg_len);
 *     while (1) {} // must not return
 * }
 *
 * void run_wasm(const uint8_t *wasm_binary, size_t wasm_len)
 * {
 *     // Step 1 — initialise environment: heap (once)
 *     wasm_runtime_env_init(heap_buf, sizeof(heap_buf));
 *
 *     // Step 2 — create runtime
 *     wasm_runtime_t *rt = wasm_runtime_create(
 *         instructions, sizeof(instructions) / sizeof(instructions[0]),
 *         linear_memory, sizeof(linear_memory),
 *         global_space, sizeof(global_space),
 *         table, sizeof(table) / sizeof(table[0]));
 *
 *     // Step 3 — load module
 *     int32_t err = wasm_runtime_parse_and_translate(rt, wasm_binary, wasm_len);
 *     if (err != WASM_OK) {  handle error  }
 *
 *     // Step 4 — call an exported function
 *     uint32_t args[] = { 42 };
 *     uint64_t result = 0;
 *     err = wasm_runtime_call(rt, "my_function", args, 1,
 *                             WASM_RETURN_WORD, &result);
 *     // result now contains the return value
 *
 *     // Step 5 — query translated code size (optional, for benchmarking)
 *     int32_t code_bytes = wasm_runtime_get_function_size(rt, "my_function");
 *
 *     // Step 6 — destroy runtime when done
 *     wasm_runtime_destroy(rt);
 * }
 * @endcode
 *
 * @section constraints Constraints & Rules
 *
 * 1. **Environment must be initialised first.**
 *    Call @ref wasm_runtime_env_init exactly once before any other function.
 *    The heap buffer must remain valid for the entire program lifetime.
 *
 * 2. **Backing buffers must be statically allocated.**
 *    The arrays passed to @ref wasm_runtime_create must stay valid and
 *    exclusively owned by the runtime until @ref wasm_runtime_destroy.
 *    On TriCore, place them in appropriate linker sections.
 *
 * 3. **linear_memory size constraint.**
 *    `linear_memory_len` passed to @ref wasm_runtime_create must equal
 *    `(power_of_two) + 7` bytes (e.g. `32768 + 7 = 32775` for a 32 KiB page).
 *    @ref wasm_runtime_create returns NULL if this constraint is violated.
 *
 * 4. **4-byte alignment.**
 *    All buffer pointers passed to @ref wasm_runtime_create must be aligned
 *    to at least 4 bytes. @ref wasm_runtime_create returns NULL on misalignment.
 *
 * 5. **Single-threaded / single-core only.**
 *    The runtime is NOT thread-safe.
 *
 * 6. **One runtime per buffer set.**
 *    Do not share the same buffers between two runtime instances.
 *
 * 7. **Destruction releases internal heap memory.**
 *    After @ref wasm_runtime_destroy the pointer is invalid. Backing
 *    buffers can be reused for a new runtime.
 *
 * 8. **Argument encoding for @ref wasm_runtime_call.**
 *    Arguments are a flat `uint32_t` array.
 *    - i32/f32 → 1 word
 *    - i64/f64 → 2 words (little-endian, low word first)
 *
 * 9. **Panic handler (link-time symbol).**
 *    When the library is built with the `panic-handler-callback` Cargo
 *    feature (enabled by default), a Rust panic will call the C function
 *    `void panic_handler(const uint8_t *msg, size_t msg_len)` that the
 *    **caller must define and link**. See @ref wasm_panic_handler_t for
 *    the required signature and contract. If the feature is disabled,
 *    panics loop silently.
 */

#ifndef AOT_WASM_C_API_H
#define AOT_WASM_C_API_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* -----------------------------------------------------------------------
 * Error codes
 * ----------------------------------------------------------------------- */

/** Operation succeeded. */
#define WASM_OK             ( 0)
/** A required pointer argument was NULL. */
#define WASM_ERR_NULL_PTR   (-1)
/** The Wasm binary could not be parsed or translated. */
#define WASM_ERR_PARSE      (-2)
/** The requested exported function was not found. */
#define WASM_ERR_NOT_FOUND  (-3)
/** An invalid argument value was provided. */
#define WASM_ERR_INVALID_ARG (-4)

/* -----------------------------------------------------------------------
 * Return-kind constants for wasm_runtime_call
 * ----------------------------------------------------------------------- */

/** No return value (void). */
#define WASM_RETURN_NONE        0
/** 32-bit return (i32 / f32). */
#define WASM_RETURN_WORD        1
/** 64-bit return (i64 / f64). */
#define WASM_RETURN_DOUBLE_WORD 2

/* -----------------------------------------------------------------------
 * Panic handler callback type
 * ----------------------------------------------------------------------- */

/**
 * @brief Signature of the external panic handler symbol.
 *
 * When the library is built with the `panic-handler-callback` Cargo feature
 * (enabled by default), a Rust panic calls the C symbol @c panic_handler
 * that the **caller must define and link**. The function must have this
 * exact signature.
 *
 * @param msg      Pointer to a UTF-8 encoded panic message. **Not**
 *                 null-terminated.
 * @param msg_len  Length of the message in bytes.
 *
 * @warning The function **must not return**. After it returns the runtime
 *          enters an infinite loop. Use it to log the error, then halt or
 *          reset the MCU.
 *
 * @warning The function must **not** call any wasm_runtime_* function or
 *          allocate heap memory — the runtime may be in an inconsistent
 *          state.
 *
 * Example definition:
 * @code
 * void panic_handler(const uint8_t *msg, size_t msg_len) {
 *     my_uart_write(msg, msg_len);
 *     while (1) {}
 * }
 * @endcode
 */
typedef void (*wasm_panic_handler_t)(const uint8_t *msg, size_t msg_len);

/* -----------------------------------------------------------------------
 * Opaque runtime type
 * ----------------------------------------------------------------------- */

/**
 * @brief Opaque handle to a WebAssembly AOT runtime instance.
 *
 * The internal layout is private to the Rust implementation. Always use the
 * API functions below to create, interact with, and destroy instances.
 */
typedef struct wasm_runtime wasm_runtime_t;

/* -----------------------------------------------------------------------
 * Environment initialisation
 * ----------------------------------------------------------------------- */

/**
 * @brief Initialise the runtime environment (heap allocator).
 *
 * Must be called **exactly once** before any other function in this library.
 *
 * @param[in] heap_buf   Pointer to a memory buffer for the heap.
 *                       Must remain valid for the program's lifetime.
 * @param[in] heap_size  Size of the heap buffer in bytes.
 *
 * @return WASM_OK on success, or a negative error code:
 *         - @c WASM_ERR_NULL_PTR if @p heap_buf is NULL.
 *         - @c WASM_ERR_INVALID_ARG if @p heap_size is 0.
 *
 * @note To receive panic notifications, define the C symbol
 *       @c panic_handler (see @ref wasm_panic_handler_t) in your application
 *       and build the library with the @c panic-handler-callback feature.
 */
int32_t wasm_runtime_env_init(
    uint8_t *heap_buf, size_t heap_size);

/* -----------------------------------------------------------------------
 * Lifecycle
 * ----------------------------------------------------------------------- */

/**
 * @brief Create a new runtime instance.
 *
 * Validates all arguments before allocating. Returns NULL without side
 * effects if any check fails.
 *
 * @param[in] instructions      Buffer for translated machine code.
 *                              Must be 4-byte aligned.
 * @param[in] instructions_len  Number of uint32_t elements (must be > 0).
 * @param[in] linear_memory     Buffer for Wasm linear memory.
 *                              Must be 4-byte aligned.
 * @param[in] linear_memory_len Size in bytes. Must equal
 *                              <tt>(2^k) + 7</tt> for some <tt>k >= 1</tt>
 *                              (e.g. 32 KiB page: <tt>32768 + 7 = 32775</tt>).
 * @param[in] global_space      Buffer for Wasm global variables.
 *                              Must be 4-byte aligned.
 * @param[in] global_space_len  Size in bytes (must be > 0).
 * @param[in] table             Buffer for the indirect-call table.
 *                              Must be 4-byte aligned.
 * @param[in] table_len         Number of uint32_t elements (must be > 0).
 *
 * @return Non-NULL opaque pointer on success.
 *         NULL on failure (any pointer NULL, any length 0, any pointer
 *         not 4-byte aligned, or @p linear_memory_len not <tt>(2^k)+7</tt>).
 */
wasm_runtime_t *wasm_runtime_create(
    uint32_t *instructions, size_t instructions_len,
    uint8_t  *linear_memory, size_t linear_memory_len,
    uint8_t  *global_space,  size_t global_space_len,
    uint32_t *table,         size_t table_len);

/**
 * @brief Destroy a runtime instance and free internal allocations.
 *
 * The backing static buffers are NOT freed and may be reused.
 * Passing NULL is a safe no-op.
 *
 * @param[in] rt  Pointer previously returned by @ref wasm_runtime_create.
 */
void wasm_runtime_destroy(wasm_runtime_t *rt);

/* -----------------------------------------------------------------------
 * Module loading
 * ----------------------------------------------------------------------- */

/**
 * @brief Parse a `.wasm` binary and translate it to native TriCore code.
 *
 * After a successful call, the runtime is ready for @ref wasm_runtime_call.
 * Calling this again on the same runtime loads a new module, replacing the
 * previous one.
 *
 * @param[in] rt            Valid runtime pointer.
 * @param[in] wasm_code     Pointer to the Wasm binary.
 * @param[in] wasm_code_len Length of the Wasm binary in bytes.
 *
 * @return WASM_OK on success, or a negative error code.
 */
int32_t wasm_runtime_parse_and_translate(
    wasm_runtime_t *rt,
    const uint8_t  *wasm_code, size_t wasm_code_len);

/* -----------------------------------------------------------------------
 * Execution
 * ----------------------------------------------------------------------- */

/**
 * @brief Call an exported Wasm function by name.
 *
 * @param[in]  rt             Valid runtime with a loaded module.
 * @param[in]  func_name      Null-terminated name of the export.
 * @param[in]  args_words     Flat array of uint32_t argument words.
 *                            May be NULL if args_words_len is 0.
 * @param[in]  args_words_len Number of words in args_words.
 * @param[in]  return_kind    One of WASM_RETURN_NONE, WASM_RETURN_WORD,
 *                            or WASM_RETURN_DOUBLE_WORD.
 * @param[out] out_result     Receives the return value. For 32-bit returns
 *                            the upper 32 bits are zero. May be NULL when
 *                            return_kind is WASM_RETURN_NONE.
 *
 * @return WASM_OK on success, or a negative error code.
 */
int32_t wasm_runtime_call(
    wasm_runtime_t *rt,
    const char     *func_name,
    const uint32_t *args_words, size_t args_words_len,
    uint8_t         return_kind,
    uint64_t       *out_result);

/* -----------------------------------------------------------------------
 * Utilities
 * ----------------------------------------------------------------------- */

/**
 * @brief Get the size of a translated function's machine code in bytes.
 *
 * Useful for benchmarking and code-size analysis.
 *
 * @param[in] rt         Valid runtime with a loaded module.
 * @param[in] func_name  Null-terminated name of the exported function.
 *
 * @return Function size in bytes (>= 0) on success, or a negative error code.
 */
int32_t wasm_runtime_get_function_size(
    const wasm_runtime_t *rt,
    const char *func_name);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* AOT_WASM_C_API_H */

#![no_std]
#![no_main]
#![feature(error_in_core)]
extern crate alloc;
mod constant_expression_eval;
pub mod isa_model;
pub mod parse_and_translate;
mod translator;
mod vb;
mod handle_misaligned_load_store;
pub use handle_misaligned_load_store::handle_misaligned_load_store;

#[cfg(test)]
#[defmt_test::tests]
mod tests {
    use defmt as _;
    use embedded_alloc::LlffHeap as Heap;

    #[allow(unused_imports)]
    pub use tc162_rt as _;

    #[allow(unused_imports)]
    pub use tsim_semihosting as _;

    struct MyState {
        flag: bool,
    }

    #[global_allocator]
    static HEAP: Heap = Heap::empty();

    use defmt::assert;
    #[init]
    fn init() -> MyState {
        {
            use core::mem::MaybeUninit;
            const HEAP_SIZE: usize = 10 * 1024;
            static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
            unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
        }
        // state initial value
        MyState { flag: true }
    }

    // This function is called before each test case.
    // It accesses the state created in `init`,
    // though like with `test`, state access is optional.
    #[before_each]
    fn before_each(state: &mut MyState) {
        defmt::println!("State flag before is {}", state.flag);
    }

    // This function is called after each test
    #[after_each]
    fn after_each(state: &mut MyState) {
        defmt::println!("State flag after is {}", state.flag);
    }

    // this unit test doesn't access the state
    #[should_panic]
    #[test]
    fn assert_true() {
        defmt::assert!(true, "error test2");
    }

    // but this test does
    #[test]
    fn assert_flag(state: &mut MyState) {
        assert!(state.flag);
        state.flag = false;
    }
}

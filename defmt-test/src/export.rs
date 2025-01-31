use crate::TestOutcome;
pub use defmt::info;
// #[cfg(feature="tsim")]
use tc162_rt;
#[cfg(feature="board")]
use core::arch::tricore::intrinsics;

/// Terminates the application and makes a semihosting-capable debug tool exit
/// with status code 0.
pub fn exit() -> ! {
    loop {
        // #[cfg(feature="tsim")]
        tc162_rt::exit(0);
    //     #[cfg(feature="board")]
    //     unsafe{
    //         intrinsics::__debug()};
        
    }
}

pub fn check_outcome<T: TestOutcome>(outcome: T, should_error: bool) {
    if outcome.is_success() == should_error {
        let note = if should_error {
            defmt::intern!("`#[should_error]` ")
        } else {
            defmt::intern!("")
        };
        defmt::panic!("{}test failed with outcome: {}", note, outcome);
    }
}

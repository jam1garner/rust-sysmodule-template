#![no_std]
#![no_main]

extern crate alloc;

extern crate nx;
use nx::result::*;
use nx::util;
use nx::diag::assert;
use nx::diag::log;
use nx::fs;

use core::panic;

#[allow(non_upper_case_globals)]
const KiB: usize = 0x400;
const HEAP_SIZE: usize = 16 * KiB;

// We're using 16KB of heap
static mut STACK_HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

#[no_mangle]
pub fn initialize_heap(_hbl_heap: util::PointerAndSize) -> util::PointerAndSize {
    unsafe {
        util::PointerAndSize::new(STACK_HEAP.as_mut_ptr(), STACK_HEAP.len())
    }
}

#[no_mangle]
pub fn main() -> Result<()> {
    fs::initialize()?;
    fs::mount_sd_card("sdmc")?;

    // do stuff here

    fs::finalize();
    Ok(())
}

#[panic_handler]
fn panic_handler(info: &panic::PanicInfo) -> ! {
    util::simple_panic_handler::<log::LmLogger>(info, assert::AssertMode::FatalThrow)
}

#![no_std]
#![no_main]

use nx::{
    diag::{assert, log},
    result::*,
    util,
    fs
};

#[allow(non_upper_case_globals)]
const KiB: usize = 0x400;

// We're using 16KB of heap
const HEAP_SIZE: usize = 16 * KiB;

#[nx::heap]
static mut STACK_HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

#[nx::main]
pub fn main() -> Result<()> {
    fs::initialize()?;
    fs::mount_sd_card("sdmc")?;

    // do stuff here

    fs::finalize();
    Ok(())
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    util::simple_panic_handler::<log::LmLogger>(info, assert::AssertMode::FatalThrow)
}

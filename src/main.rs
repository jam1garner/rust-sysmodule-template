#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

#[macro_use]
extern crate nx;
use nx::svc;
use nx::result::*;
use nx::results;
use nx::util;
use nx::diag::assert;
use nx::diag::log;
use nx::service::vi;
use nx::service::nv;
use nx::gpu;
use nx::service::hid;
use nx::input;
use nx::fs;

use core::panic;

// We're using 8MB of heap
static mut STACK_HEAP: [u8; 0x00004000] = [0; 0x00004000];

#[no_mangle]
pub fn initialize_heap(_hbl_heap: util::PointerAndSize) -> util::PointerAndSize {
    unsafe {
        util::PointerAndSize::new(STACK_HEAP.as_mut_ptr(), STACK_HEAP.len())
    }
}

#[no_mangle]
pub fn main() -> Result<()> {
    // Old Tesla
    fs::delete_directory(String::from("sdmc:/atmosphere/contents/010000000007E51A/flags"));
    fs::delete_directory(String::from("sdmc:/atmosphere/contents/010000000007E51A"));

    // Delete Nichole
    fs::delete_directory(String::from("sdmc:/atmosphere/exefs_patches/nichole_logo"));

    // Delete itself
    fs::delete_directory(String::from("sdmc:/atmosphere/contents/010000000000DA7A/flags"));
    fs::delete_directory(String::from("sdmc:/atmosphere/contents/010000000000DA7A"));

    Ok(())
}

#[panic_handler]
fn panic_handler(info: &panic::PanicInfo) -> ! {
    util::simple_panic_handler::<log::LmLogger>(info, assert::AssertMode::FatalThrow)
}
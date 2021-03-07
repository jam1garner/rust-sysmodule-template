/*
* Copyright 2021 tomGER, git@tomger.eu
*
* Licensed under the EUPL, Version 1.2 or – as soon they will be approved by the European Commission - subsequent versions of the EUPL (the "Licence");
* You may not use this work except in compliance with theLicence.
*
* You may obtain a copy of the Licence at: https://joinup.ec.europa.eu/software/page/eupl
*
* Unless required by applicable law or agreed to in writing, software distributed under the Licence is distributed on an "AS IS" basis,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the Licence for the specific language governing permissions and limitations under the Licence.
*/

#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::String;

extern crate nx;
use nx::result::*;
use nx::util;
use nx::diag::assert;
use nx::diag::log;
use nx::fs;

use core::panic;

// We're using 16KB of heap
static mut STACK_HEAP: [u8; 0x00004000] = [0; 0x00004000];

#[no_mangle]
pub fn initialize_heap(_hbl_heap: util::PointerAndSize) -> util::PointerAndSize {
    unsafe {
        util::PointerAndSize::new(STACK_HEAP.as_mut_ptr(), STACK_HEAP.len())
    }
}

pub fn exists_folder(path: &str) -> bool {
    match fs::get_entry_type(String::from(path)) {
        Ok(i) => match i { 
            fs::DirectoryEntryType::File => false,
            fs::DirectoryEntryType::Directory => true
        }
        Err(_) => false
    }
}

#[no_mangle]
pub fn main() -> Result<()> {
    fs::initialize()?;
    fs::mount_sd_card("sdmc")?;

    let delete_folders = [
    "sdmc:/atmosphere/contents/010000000007E51A/flags",
    "sdmc:/atmosphere/contents/010000000007E51A",
    "sdmc:/atmosphere/exefs_patches/nichole_logo",
    "sdmc:/atmosphere/contents/010000000000DA7A/flags",
    "sdmc:/atmosphere/contents/010000000000DA7A",
    "sdmc:/rustisbad"
    ];

    for i in 0..delete_folders.len() {
        if exists_folder(delete_folders[i]) {
            fs::delete_directory(String::from(delete_folders[i]))?;
        }
    }

    fs::finalize();
    Ok(())
}

#[panic_handler]
fn panic_handler(info: &panic::PanicInfo) -> ! {
    util::simple_panic_handler::<log::LmLogger>(info, assert::AssertMode::FatalThrow)
}
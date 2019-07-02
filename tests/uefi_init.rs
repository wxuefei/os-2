#![no_std]
#![no_main]
#![feature(decl_macro)]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
extern crate log;

use os::drivers::{console::*, uefi_init};
use uefi::prelude::*;

#[no_mangle]
pub extern "C" fn efi_main(image: uefi::Handle, st: SystemTable<Boot>) -> ! {
    let (_st, _map) = uefi_init(image, st);
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    os::test_panic_handler(info)
}

#[test_case]
fn test_print() {
    info!("test_print... ");
    println!("test_println output");
    print!("[ok]");
}
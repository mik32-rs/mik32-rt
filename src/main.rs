// Test binary
#![no_std]
#![no_main]

use core::panic::PanicInfo;

use mik32_rt::*;
use mik32_rt_macros::entry;
use riscv::asm::nop;

#[entry]
fn main() -> ! {
    unsafe {
        //
    }
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(export_name = "trap_handler")]
fn trap() {
    loop {
        nop();
    }
}

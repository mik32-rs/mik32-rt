#![no_std]
#![no_main]

use core::panic::PanicInfo;

use riscv::asm::nop;

#[unsafe(no_mangle)]
fn main() {
    loop {
        nop();
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[unsafe(export_name = "trap_handler")]
fn trap() {
    loop {
        nop();
    }
}

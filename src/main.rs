#![no_std]
#![no_main]

use core::panic::PanicInfo;

use riscv::asm::nop;

#[unsafe(no_mangle)]
fn main() {
    let mut i: i32 = 0;
    let ptr: *mut i32 = &mut i;

    loop {
        unsafe {
            ptr.write_volatile(ptr.read_volatile() + 1);
        }
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

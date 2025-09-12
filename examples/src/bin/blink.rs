// Test binary
#![no_std]
#![no_main]

use core::panic::PanicInfo;

use mik32_rt as _;
use mik32_rt_macros::entry;
use mik32v2_pac::Peripherals;
use riscv::asm::nop;

#[entry]
fn main() -> ! {
    let p = unsafe { Peripherals::steal() };

    p.pm.clk_apb_p_set().modify({
        |_, w| {
            w.uart_0()
                .enable()
                .gpio_0()
                .enable()
                .gpio_1()
                .enable()
                .gpio_2()
                .enable()
        }
    });

    p.pm.clk_apb_m_set()
        .modify(|_, w| w.pad_config().enable().wu().enable().pm().enable());

    p.pad_config
        .pad2_cfg()
        .modify(|_, w| w.port_2_7().func1_gpio());
    p.gpio8_2
        .direction_out()
        .modify(|_, w| unsafe { w.bits(1 << 7) });

    loop {
        p.gpio8_2.output().write(|w| unsafe { w.bits(1 << 7) });
        for _ in 0..1_000_00 {
            nop()
        }
        p.gpio8_2.output().reset();
        for _ in 0..1_000_00 {
            nop()
        }
    }
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

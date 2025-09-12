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
    let p = Peripherals::take().unwrap();

    p.pm.clk_apb_m_set().modify(|_, w| w
        .pad_config().enable()
        .wu().enable()
        .pm().enable()
    );
    
    p.pm.clk_apb_p_set().modify({|_, w| w
        .gpio_0().enable()
        .gpio_1().enable()
        .gpio_2().enable()
        .timer32_1().enable()
    });

    p.pad_config.pad0_cfg().modify(|_, w| w.port_0_0().func3_interface_or_timer());
    p.timer32_1.ch1_cntr().modify(|_, w| w
        .mode().pwm()
        .en().set_bit()
    );
    p.timer32_1.top().modify(|_, w| unsafe { w.tim_top().bits(320) });
    p.timer32_1.ch1_ocr().modify(|_, w| unsafe { w.ocr().bits(320 >> 1) });
    p.timer32_1.enable().modify(|_, w| w.tim_clr().set_bit());
    p.timer32_1.enable().modify(|_, w| w.tim_en().enable());

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

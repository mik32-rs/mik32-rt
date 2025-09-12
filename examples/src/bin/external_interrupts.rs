#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo, ptr};

use mik32_rt as _;
use mik32_rt_macros::entry;
use mik32v2_pac::Peripherals;
use riscv as _;

static mut FLAG: bool = false;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    p.pm.clk_apb_m_set().modify(|_, w| w
        .pad_config().enable()
        .epic().enable()
        .wu().enable()
        .pm().enable()
    );
    
    p.pm.clk_apb_p_set().modify({|_, w| w
        .gpio_irq().enable()
        .gpio_0().enable()
        .gpio_1().enable()
        .gpio_2().enable()
    });

    p.pad_config.pad2_cfg().modify(|_, w| w.port_2_7().func1_gpio());
    p.pad_config.pad2_cfg().modify(|_, w| w.port_2_6().func1_gpio());

    p.gpio8_2.direction_out().modify(|_, w| unsafe { w.bits(1 << 7) });
    p.gpio8_2.direction_in().modify(|_, w| unsafe { w.bits(1 << 6) });

    p.gpio_irq.line_mux().write(|w| unsafe { w.bits(9 << ( 2 * 4) ) });

    p.gpio_irq.level_set().modify(|_, w| unsafe { w.bits(1 << 2) });
    p.gpio_irq.edge().modify(|_, w| unsafe { w.bits(1 << 2) });
    p.gpio_irq.any_edge_clear().modify(|_, w| unsafe { w.bits(1 << 2) });
    p.gpio_irq.enable_set().modify(|_, w| unsafe { w.bits(1 << 2) });
    
    p.epic.mask_level_set().write(|w| w
        .gpio().set_bit()
    );

    unsafe {
        asm!(
            "
                # Enable external interrupts (mie.MEIE <= 1)
                csrr t0, mie # Read the mie register
                li t2, 0x800 # Set the MEIE field (bit 11)
                or t1, t1, t2
                csrw mie, t1 # Update the mie register

                # Enable global interrupts (mstatus.MIE <= 1)
                csrr t0, mstatus # Read the mstatus register
                ori t0, t0, 0x8 # Set MIE field (bit 3)
                csrw mstatus, t0 # Update the mstatus register
            "
        );
    }

    loop {
        let flag = unsafe { ptr::read_volatile(&raw const FLAG) };
        if flag == true {
            p.gpio8_2.set().write(|w| unsafe { w.bits(1 << 7) });
        } else {
            p.gpio8_2.clear().write(|w| unsafe { w.bits(1 << 7) });
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(export_name = "trap_handler")]
fn trap() {
    let p = unsafe {Peripherals::steal()};

    if p.epic.raw_status().read().gpio().bit_is_set() {
        if (p.gpio_irq.interrupt().read().bits()  & (1 << (2))) != 0 {
            unsafe {
                let current = ptr::read_volatile(&raw const FLAG);
                ptr::write_volatile(&raw mut FLAG, !current);
            }
        }
        p.gpio_irq.clear().write(|w| unsafe { w.bits(0b11111111) });
    }
    p.epic.clear().write(|w| unsafe { w.bits(0xFFFFFFFF) });
}
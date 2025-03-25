#![no_std]
#![no_main]
#![feature(riscv_ext_intrinsics)]
#![feature(core_intrinsics)]

use core::{arch::{global_asm}, panic::PanicInfo};
use mik32_rt_macros::entry;
use mik32v2_pac::Peripherals;
use riscv as _;

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
        .uart_0().enable()
    });

    p.pad_config.pad0_cfg().modify(|_, w| w.port_0_6().func2_interface());
    p.usart_0.divider().modify(|_, w| unsafe { w.brr().bits(0x115) });
    p.usart_0.control1().modify(|_, w| w
        .te().enable()
        .ue().enable()
    );

    while p.usart_0.flags().read().teack().bit_is_clear() {};

    let mut i = 0;
    let str = "Start\n";

    while str.as_bytes()[i] != b'\0' {
        p.usart_0.txdata().modify(|_, w| unsafe { w.tdr().bits(str.as_bytes()[i].into()) });
        while p.usart_0.flags().read().tc().bit_is_clear() {};
        i += 1;
    };

    loop {
        
    }
}



#[unsafe(export_name = "trap_handler")]
fn trap() {
    loop {
        
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
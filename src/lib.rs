#![no_std]
#![no_main]

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod asm;
pub use mik32_rt_macros::*;

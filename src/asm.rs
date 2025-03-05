use core::arch::global_asm;

global_asm!(
    r#"
    #define EXCEPTION_STACK_SPACE 128
    #define EXCEPTION_SAVED_REGISTERS 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31

    .weak SmallSystemInit, SystemInit

    .globl trap_entry
    .globl _start_trap

    .altmacro
    .macro memcpy src_beg, src_end, dst, tmp_reg
        j    memcpy_2\@
    memcpy_1\@:
        lw   \tmp_reg, (\src_beg)
        sw   \tmp_reg, (\dst)
        add  \src_beg, \src_beg, 4
        add  \dst, \dst, 4
    memcpy_2\@:
        bltu \src_beg, \src_end, memcpy_1\@
    .endm

    .macro memset dst_beg, dst_end, val_reg
        j    memset_2\@
    memset_1\@:
        sw   \val_reg, (\dst_beg)
        add  \dst_beg, \dst_beg, 4
    memset_2\@:
        bltu \dst_beg, \dst_end, memset_1\@
    .endm
    
    .macro la_abs reg, address
        .option push
        .option norelax
        lui \reg, %hi(\address)
        addi \reg, \reg, %lo(\address)
        .option pop
    .endm

    .macro jalr_abs return_reg, address
        .option push
        .option norelax
        lui \return_reg, %hi(\address)
        jalr \return_reg, %lo(\address)(\return_reg)
        .option pop
    .endm

    .section .init, "ax"

    _start:

        li t0, 128000
        start_loop_delay:
        nop
        addi t0, t0, -1
        bnez t0, start_loop_delay

        la_abs  sp, __sstack
        la_abs  gp, _gp

        la_abs  a1, __sidata
        la_abs  a2, __eidata
        la_abs  a3, __sdata
        memcpy  a1, a2, a3, t0

        la_abs  a1, __siram_text
        la_abs  a2, __eiram_text
        la_abs  a3, __sram_text
        memcpy  a1, a2, a3, t0

        la_abs  a1, __sbss
        la_abs  a2, __ebss
        memset a1, a2, zero

        jalr_abs ra, SmallSystemInit
        jalr_abs ra, SystemInit
        jalr_abs ra, __start_rust
    1:  wfi
        j 1b

    SmallSystemInit:
    SystemInit:
        ret

    .section .init.trap, "ax"
    trap_entry:
        j _start_trap

    .section .trap, "ax"
    _start_trap:
        addi    sp, sp, -(128)
        .irp index, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
            sw      x\index, 4*index(sp)
        .endr

        la      ra, trap_handler
        jalr    ra

        .irp index, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
            lw      x\index, 4*index(sp)
        .endr

        addi    sp, sp, 128
        mret

    "#
);


OUTPUT_FORMAT("elf32-littleriscv", "elf32-littleriscv", "elf32-littleriscv")
OUTPUT_ARCH(riscv)

ENTRY(_start)


/* MEMORY {
  eeprom (RX): ORIGIN = 0x01000000, LENGTH = 8K
  ram (RWX): ORIGIN = 0x02000000, LENGTH = 16K
} */

MEMORY
{
    RAM : ORIGIN = 0x02000000, LENGTH = 16K
    FLASH : ORIGIN = 0x01000000, LENGTH = 8K
}

REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);
REGION_ALIAS("REGION_RAM", RAM);

STACK_SIZE = 1024;

CL_SIZE = 16;

/* REGION_ALIAS("REGION_TEXT", eeprom); */
/* REGION_ALIAS("REGION_RAM", ram); */


SECTIONS {
    .text ORIGIN(REGION_TEXT) : {
        PROVIDE(__TEXT_START__ = .);
        *(.text._start) 
        /* *(.text .text.*) */
        *(.text.smallsysteminit)
        *(.text.SmallSystemInit)
        . = ORIGIN(REGION_TEXT) + 0xC0;
        KEEP(*(.trap_text))

        *(.text)
        *(.text.*)
        *(.rodata)
        *(.rodata.*)      
        . = ALIGN(CL_SIZE);
        PROVIDE(__TEXT_END__ = .);
    } >REGION_TEXT 



    .data : 
    AT( __TEXT_END__ ) {
        PROVIDE(__DATA_START__ = .);
        _gp = .;
        *(.srodata.cst16) *(.srodata.cst8) *(.srodata.cst4) *(.srodata.cst2) *(.srodata*)
        *(.sdata .sdata.* .gnu.linkonce.s.*)
        *(.data .data.*)
        . = ALIGN(CL_SIZE);
        PROVIDE(__DATA_END__ = .);
    } >REGION_RAM
    
    __DATA_IMAGE_START__ = LOADADDR(.data);
    __DATA_IMAGE_END__ = LOADADDR(.data) + SIZEOF(.data);

    /* thread-local data segment */
    .tdata : {
        PROVIDE(_tls_data = .);
        PROVIDE(_tdata_begin = .);
        *(.tdata .tdata.*)
        PROVIDE(_tdata_end = .);
        . = ALIGN(CL_SIZE);
    } >REGION_RAM

    .tbss : {
        PROVIDE(__BSS_START__ = .);
        *(.tbss .tbss.*)
        . = ALIGN(CL_SIZE);
        PROVIDE(_tbss_end = .);
    } >REGION_RAM

    /* bss segment */
    .sbss : {
        *(.sbss .sbss.* .gnu.linkonce.sb.*)
        *(.scommon)
    } >REGION_RAM

    .bss : {
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(CL_SIZE);
        PROVIDE(__BSS_END__ = .);
    } >REGION_RAM

    /* Code intended to be copied to REGION_RAM before execution */
    .ram_text : 
    AT( ALIGN(__BSS_END__, CL_SIZE) ) {
        PROVIDE(__RAM_TEXT_START__ = .);
        *(.ram_text)
        . = ALIGN(CL_SIZE);
        PROVIDE(__RAM_TEXT_END__ = .);
    } > REGION_RAM
    
    __RAM_TEXT_IMAGE_START__ = LOADADDR(.ram_text);
    __RAM_TEXT_IMAGE_END__ = LOADADDR(.ram_text) + SIZEOF(.ram_text);

    ASSERT(__RAM_TEXT_IMAGE_END__ < ORIGIN(REGION_TEXT) + LENGTH(REGION_TEXT), "REGION_TEXT segment overflows")
    ASSERT(__RAM_TEXT_END__ < ORIGIN(REGION_RAM) + LENGTH(REGION_RAM) - STACK_SIZE, "REGION_RAM section overflows")

    _end = .;
    PROVIDE(__end = .);

    /* End of uninitalized data segement */

    .stack ORIGIN(REGION_RAM) + LENGTH(REGION_RAM) - STACK_SIZE : {
        FILL(0);
        PROVIDE(__STACK_START__ = .);
        . += STACK_SIZE;
        PROVIDE(__C_STACK_TOP__ = .);
        PROVIDE(__STACK_END__ = .);
    } >REGION_RAM

    /DISCARD/ : {
        *(.eh_frame .eh_frame.*)
    }
}
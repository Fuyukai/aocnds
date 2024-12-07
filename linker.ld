/* Standard cruft at the beginning of the file. I don't know what omitting this does, 
  so let's not do that. */
OUTPUT_FORMAT("elf32-littlearm")
OUTPUT_ARCH(arm)
ENTRY(_start)

/*
 * The memory layout of the DS is pretty simple; there's three user-usable blocks. 
 */
MEMORY {
    itcm (rwx) : ORIGIN = 0x01ff8000, LENGTH = 32K
    main_ram (rwx) : ORIGIN = 0x02000000, LENGTH = 4M
    dtcm (rw) : ORIGIN = 0x027e0000, LENGTH = 16K
    shared_ram (rw) : ORIGIN = 0x037f800, LENGTH = 32K
}

SECTIONS {
    /* The CRT always needs to be in main memory. Let's put it at the start for good measure. */
    .crt : ALIGN(4) { *(.crt); *(.crt.*) } > main_ram =0x39

    /* The text section should be in main memory. Things that will go into the ITCM can be put 
       there explicitly. */
    .text : ALIGN(4) { *(.text); *(.text.*) } > main_ram =0x39
    /* The data section should also be in RAM. DTCM variables can be placed there explicitly. */
    .data : ALIGN(4) { *(.data); *(.data.*) } > main_ram =0x39
    .rodata : ALIGN(4) { *(.rodata); *(.rodata.*); } > main_ram =0x39
    /* The BSS section doesn't really... exist, but we need to allocate some space for it anyway. */
    .bss : ALIGN(4) { KEEP(*(.bss)); KEEP(*(.bss.*)); } > main_ram

    __ram_end = .;

    /* Let's get rid of anything we haven't explicitly specified. This includes some debugging
       info for now. */
    /DISCARD/ : { * }
}

/* Various addressees used in our CRT assembly. */
__bss_start = ADDR(.bss);
__bss_size = SIZEOF(.bss);

__dtcm_region_start = ORIGIN(dtcm);
__dtcm_region_end = ORIGIN(dtcm) + LENGTH(dtcm) - 4;

__itcm_region_start = ORIGIN(itcm);

__shram_region_start = ORIGIN(shared_ram);

/* The first two stacks are 0x100 long, and the IRQ stack has 0x100 bytes allocated at the start 
   as a "reserved area". The pointer to the IRQ handler is in this area anyway.
   */
__stack_start_irq = __dtcm_region_end - 0x100;
__stack_start_svc = __stack_start_irq - 0x100;
__stack_start_sys = __stack_start_svc - 0x100;

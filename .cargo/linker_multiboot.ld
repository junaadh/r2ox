/* ENTRY(start) */
/* OUTPUT_FORMAT(elf64-x86-64) */

/* SECTIONS { */
    /* . = 1M; */

    /* Ensure that the multiboot header is at the beginning */
    /* .boot : */
    /* { */
        /* Ensure that other boot-related sections are placed here */
        /* For example, bootloader code or data */
        /* KEEP(*(.multiboot_header)) */
    /* } */

    /* .text : ALIGN(4096) */
    /* { */
        /* *(.text) */
    /* } */

    /* .rodata : ALIGN(4096) */
    /* { */
        /* *(.rodata) */
    /* } */

    /* .data : ALIGN(4096) */
    /* { */
        /* *(.data) */
    /* } */
    
    /* Read-write data (uninitialized) and stack */
    /* .bss : ALIGN(4096) */
    /* { */
        /* *(COMMON) */
        /* *(.bss) */
    /* } */
/* } */
ENTRY(start)
OUTPUT_FORMAT(elf64-x86-64)

SECTIONS {
    . = 1M;

    /* Ensure that the boot section is at the beginning */
    .boot :
    {
        KEEP(*(.boot))   /* Include all sections related to boot */
    }

    .text : ALIGN(4096)
    {
        *(.text)
    }

    .rodata : ALIGN(4096)
    {
        *(.rodata)
    }

    .data : ALIGN(4096)
    {
        *(.data)
    }

    .bss : ALIGN(4096)
    {
        *(COMMON)
        *(.bss)
    }

    .dynsym :
    {
        *(.dynsym)
    }

    .gnu.hash :
    {
        *(.gnu.hash)
    }

    .hash :
    {
        *(.hash)
    }

    .dynstr :
    {
        *(.dynstr)
    }

    .rela.dyn :
    {
        *(.rela.dyn)
    }

    .got :
    {
        *(.got)
    }

    .dynamic :
    {
        *(.dynamic)
    }

    .data.rel.ro : ALIGN(4096)
    {
        *(.data.rel.ro)
    }

    .bss : ALIGN(4096)
    {
        *(.bss)
    }

    .debug_abbrev :
    {
        *(.debug_abbrev)
    }

    .debug_info :
    {
        *(.debug_info)
    }

    .debug_aranges :
    {
        *(.debug_aranges)
    }

    .debug_ranges :
    {
        *(.debug_ranges)
    }

    .debug_str :
    {
        *(.debug_str)
    }

    .comment :
    {
        *(.comment)
    }

    .debug_frame :
    {
        *(.debug_frame)
    }

    .debug_line :
    {
        *(.debug_line)
    }

    .debug_loc :
    {
        *(.debug_loc)
    }

    .symtab :
    {
        *(.symtab)
    }

    .shstrtab :
    {
        *(.shstrtab)
    }

    .strtab :
    {
        *(.strtab)
    }
}


SECTIONS {
    . = 0xFFFF800000000000;
    .text : ALIGN(4096) { 
        *(.text)
        *(.text.*)
    }
    .rodata : ALIGN(4096) { 
        *(.rodata)
        *(.rodata.*)
    }
    .data : ALIGN(4096) { 
        *(.data)
        *(.data.*)
    }
    .data : ALIGN(4096) { 
        *(.data)
        *(.data.*)
    }
    .got : ALIGN(4096) { 
        *(.got)
        *(.got.*)
    }
}
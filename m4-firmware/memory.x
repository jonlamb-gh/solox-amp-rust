/* TODO - still need to tune this up, probably should define _stext and stack/heap bits */

MEMORY
{
    /* NOTE K = KiBi = 1024 bytes */

    /* TCM configuration */
    /* TCM(L) region for code, and the TCM(U) region for data */
    /* FLASH in TCM(L) at 0x1FFF_8000 (alias 0x0000_0000), this is 0x007F_8000 from the A9 */
    /* RAM in TCM(U) at 0x2000_0000, this is 0x0080_0000 from the A9 */
    FLASH (rx) : ORIGIN = 0x1FFF8000, LENGTH = 32K
    RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 32K

    /* OCRAM configuration */
    /* TODO - reference has 32K but TRM says 128 KB, should we then have L=128/2=64K */
    /* FLASH in OSCRAM at 0x0091_0000 (upper half of region), this is 0x0091_0000 from the A9 */
    /* RAM in TCM(U) at 0x2000_0000, this is 0x0080_0000 from the A9 */
    /* FLASH (rx) : ORIGIN = 0x00910000, LENGTH = 32K */
    /* RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 32K */

    /* DDR configuration */
    /* FLASH in DDR at 0x9FF0_0000, this is 0x9FF0_0000 from the A9 */
    /* RAM in TCM(U) at 0x2000_0000, this is 0x0080_0000 from the A9 */
    /* FLASH (rx) : ORIGIN = 0x9FF00000, LENGTH = 1024K */
    /* RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 32K */
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* You may want to use this variable to locate the call stack and static
   variables in different memory regions. Below is shown the default value */
/* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */

/* You can use this symbol to customize the location of the .text section */
/* If omitted the .text section will be placed right after the .vector_table
   section */
/* This is required only on microcontrollers that store some configuration right
   after the vector table */
/* _stext = ORIGIN(FLASH) + 0x400; */

/* Size of the heap (in bytes) */
/* _heap_size = 1024; */

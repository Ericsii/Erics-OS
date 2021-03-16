/* 目标架构 */
OUTPUT_ARCH(riscv)

/* 执行入口 */
ENTRY(_start)

/* 数据起始地址 */
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    /* '.' location counter */
    . = BASE_ADDRESS;

    /* start 符号表示全部的开始位置 */
    kernel_start = .;

    text_start = .;

    /* .text 字段 */
    .text : {
        /* 把 entry 函数放在最前面 */
        *(.text.entry)
        *(.text .text.*)
    }

    rodata_start = .;

    /* .rodata 字段 */
    .rodata : {
        *(.rodata .rodata.*)
    }

    data_start = .;

    /* .data 字段 */
    .data : {
        *(.data .data.*)
    }

    bss_start = .;

    /* .bss 字段 */
    .bss : {
        *(.sbss .bss .bss.*)
    }

    /* 结束地址 */
    kernel_end = .;
}
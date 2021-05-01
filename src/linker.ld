/* 目标架构 */
OUTPUT_ARCH(riscv)

/* 执行入口 */
ENTRY(_start)

/* 数据起始地址 */
BASE_ADDRESS = 0xffffffff80200000;

SECTIONS
{
    /* '.' location counter */
    . = BASE_ADDRESS;

    /* start 符号表示全部的开始位置 */
    kernel_start = .;

    /* 加入对齐 */
    . = ALIGN(4K);
    text_start = .;

    /* .text 字段 */
    .text : {
        /* 把 entry 函数放在最前面 */
        *(.text.entry)
        *(.text .text.*)
    }

    /* 加入对齐 */
    . = ALIGN(4K);
    rodata_start = .;

    /* .rodata 字段 */
    .rodata : {
        *(.rodata .rodata.*)
    }

    /* 加入对齐 */
    . = ALIGN(4K);
    data_start = .;

    /* .data 字段 */
    .data : {
        *(.data .data.*)
    }

    /* 加入对齐 */
    . = ALIGN(4K);
    bss_start = .;

    /* .bss 字段 */
    .bss : {
        *(.sbss .bss .bss.*)
    }

    /* 加入对齐 */
    . = ALIGN(4K);
    /* 结束地址 */
    kernel_end = .;
}
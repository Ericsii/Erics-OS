# 保存上下文
.altmacro

# 寄存器宽度
.set    REG_SIZE, 8

# 上下文大小
.set    CONTEXT_SIZE, 34


# 自定义宏

# 保存寄存器
.macro SAVE reg, offset
    sd  \reg, \offset * 8(sp)
.endm

.macro SAVE_N n
    SAVE    x\n, \n
.endm

# 栈中恢复寄存器
.macro LOAD reg, offset
    ld  \reg, \offset * 8(sp)
.endm

.macro LOAD_N n
    LOAD    x\n, \n
.endm



    .section .text
    .globl  __interrupt     # 定义 __interrupt 入口

__interrupt:
    # 申请栈空间
    addi    sp, sp, -34 * 8

    # 保存寄存器 x1 (先保存 x1 为了之后保存 sp 寄存器)
    SAVE    x1, 1
    # 保存 sp 寄存器
    addi    x1, sp, 34 * 8
    SAVE    x1, 2

    # 循环保存 x3-x31
    .set    n, 3
    .rept   29
        SAVE_N  %n
        .set    n, n + 1
    .endr

    # 保存 CSR
    csrr    s1, sstatus
    csrr    s2, sepc
    SAVE    s1, 32
    SAVE    s2, 33

    # 调用 handle_interrupt 函数
    # context: $mut Context
    mv      a0, sp
    # scause: Scause
    csrr    a1, scause
    # stval: usize
    csrr    a2, stval
    jal     handle_interrupt

    # 离开中断恢复现场
    .globl  __restore
__restore:

    # 恢复 CSR
    LOAD    s1, 32
    LOAD    s2, 33
    csrw    sstatus, s1
    csrw    sepc, s2

    # 恢复 universal 寄存器
    LOAD    x1, 1
    .set    n, 3
    .rept   29
        LOAD_N  %n
        .set    n, n + 1
    .endr

    # 恢复 sp
    LOAD    sp, 2
    sret

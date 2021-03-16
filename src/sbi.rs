//! 调用 OpenSBI Machine 层接口
//! 允许未使用的变量
#![allow(unused)]


/// 调用 SBI 编号常量
const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;


/// OpenSBI 调用
/// 使用 ecall 调用系统接口
/// Args: 
///     - which: x17(a7) SBI 调用编号寄存器
///     - arg0: x11(a0) 
///     - arg1: x11(a1)
///     - arg2: x12(a2) SBI 调用参数寄存器
#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
            : "memory"
            : "volatile");
    }
    ret
}


/// 输出字符到 console
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}


/// 从 console 中读取一个字符
pub fn console_getchar() -> usize {
    return sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0);
}


/// 关机指令
pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    unreachable!()
}
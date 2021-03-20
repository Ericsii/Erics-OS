//! 设置时钟中断

use crate::sbi::set_timer;
use riscv::register::{time, sie, sstatus};

/// 初始化始终中断
/// 
/// 开启中断使能，设置第一次时钟中断
pub fn init() {
    unsafe {
        // 开启 STIE 使能
        sie::set_stimer();
        // 开启 SIE (不是 sie 寄存器)
        sstatus::set_sie();
    }
    // 设置下一次时钟中断
    set_next_timeout();
}


/// 设置时钟中断间隔
static INTERVAL: usize = 100000;

/// 设置下一次时钟中断
fn set_next_timeout() {
    set_timer(time::read() + INTERVAL);
}


/// 触发时钟中断计数
pub static mut TICKS: usize = 0;

/// 每一次时钟中断时调用
/// 
/// 设置下一次时钟中断，同时计数 +1
pub fn tick() {
    set_next_timeout();
    unsafe {
        TICKS += 1;
        if TICKS % 100 == 0 {
            println!("{} ticks", TICKS);
        }
    }
}
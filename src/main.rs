//! - #![no-std] 
//!   禁用标准库
#![no_std]
//! - #![no_main]
//!   不使用 main 函数作为程序入口
#![no_main]
//! 使用汇编
//! - #![feature(llvm_asm)]
//!   内嵌汇编
#![feature(llvm_asm)]

//! - #![feature(global_asm)]
//!   内嵌汇编文件
#![feature(global_asm)]

//! - #![feature(panic_info_message)] 
//!   panic! 时，获取其中的信息并打印
#![feature(panic_info_message)]

// 汇编程序入口
global_asm!(include_str!("entry.asm"));

#[macro_use]
mod console;
mod panic;
mod sbi;
mod interrupt;

/// Rust 入口函数
#[no_mangle]
pub extern "C" fn rust_main() {
    // 初始化
    interrupt::init();

    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };
}
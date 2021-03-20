use super::context::Context;
use riscv::register::{
    stvec, scause::Scause, scause::Trap, scause::Interrupt, scause::Exception
};

global_asm!(include_str!("./interrupt.asm"));

/// 初始化中断处理
/// 
/// 写入中断入口，并加入 stvec 中
pub fn init() {
    unsafe {
        extern "C" {
            fn __interrupt();
        }
        // 使用 Direct 模式，将中断入口设置为 `__interrupt`
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);
    }
}

/// 实现中断的入口
/// 
/// Context, scause, stval 作为参数
#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    // println!("\x1b[33;1mInterrupted: {:?}\x1b[0m", scause.cause());

    match scause.cause() {
        // 断点中断
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        // 时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        // 其他情况
        _ => fault(context, scause, stval)
    }
}


/// 处理 ebreak 断点
/// 
/// 继续执行，跳过这条 ebreak
fn breakpoint(_context: &mut Context) {
    // println!("Breakpoint at 0x{:x}", _context.sepc);
}


/// 处理时钟中断
/// 
/// 目前只会在 [`timer`] 模块中进行计数
fn supervisor_timer(_: &Context) {
    super::timer::tick();
}


/// 出现未能解决的异常
fn fault(context: &mut Context, scause: Scause, stval: usize) {
    panic!(
        "Unresolved interrupt: {:?}\n{:x?}\nstval: {:x}",
        scause.cause(),
        context,
        stval
    );
}

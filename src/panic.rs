//! 替代 std 实现 panic, abort

use core::panic::PanicInfo;
use crate::sbi::shutdown;


/// Call back function of panic
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("\x1b[31;1mPanic info: '{}'\x1b[0m", info.message().unwrap());
    shutdown();
}


/// End the program
#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("abort()");
}
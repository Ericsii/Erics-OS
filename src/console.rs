/// 实现对 write_str 的实现

use crate::sbi::*;
use core::fmt::{self, Write};

struct Stdout;


impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut buf = [0u8; 4];
        for ch in s.chars() {
            for code_ptr in ch.encode_utf8(&mut buf).as_bytes().iter() {
                console_putchar(*code_ptr as usize);
            }
        }
        Ok(())
    }
}


/// 打印 ['core::format_args!'] 格式化后的数据
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}


/// 实现 print!
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    };
}


/// 实现 println!
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}